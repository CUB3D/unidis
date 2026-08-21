use std::collections::BTreeMap;
use std::sync::LazyLock;
use tracing::level_filters::LevelFilter;
use libundis::{ARCHES, UniDis, UnidisArch};
use anyhow::Context;

pub fn get_arch_map() -> BTreeMap<String, UnidisArch> {
    tracing::info!("get_arch_map(_)");

    let mut o = BTreeMap::new();
    for a in ARCHES {
        o.insert(a.get_arch_id().to_string(), a.get_arch());
    }
    o
}

static ARCH_MAP: LazyLock<BTreeMap<String, UnidisArch>> = LazyLock::new(get_arch_map);

pub fn guess_arch(x: &[u8]) -> anyhow::Result<UnidisArch> {
    tracing::info!("guess_arch(_)");

    let mut res = (0, UnidisArch::ArmV8Le);
    for a in ARCH_MAP.values() {
        let dis = UniDis::new_arch(*a).context("new arch fail")?;
        let mut dis = dis.dissassembler(x.to_vec(), 0).context("dis fail")?;

        let mut c = 0;
        while let Ok(Some(i)) = dis.next_instruction() {
            c += i.bytes.len();
        }
        if c > res.0 {
            res = (c, *a);
        }
    }

    Ok(res.1)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_bytes(size: usize, align: usize) -> *mut u8 {
    tracing::info!("alloc_bytes(size: {size}, align: {align})");
    let layout = std::alloc::Layout::from_size_align(size, align).expect("Layout creation failed");
    unsafe { std::alloc::alloc(layout) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_bytes(ptr: *mut u8, size: usize, align: usize) {
    tracing::info!("free_bytes(..., size: {size}, align: {align})");
    let layout = std::alloc::Layout::from_size_align(size, align).expect("Layout creation failed");
    unsafe { std::alloc::dealloc(ptr, layout) };
}

pub fn load() {
    tracing::info!("load()");
    for arch in ARCH_MAP.keys() {
        emscripten_functions::emscripten::run_script(
            format!(
                r##"
                    const opt1 = document.createElement("option");
                    opt1.value = "{}";
                    opt1.text = "{}";
                    document.getElementById("arch").add(opt1, null);
                "##,
                arch,
                arch,
            )
        );
    }

    tracing::info!("Load done");
}

pub fn disassemble_impl(input_data: &str, base_addr: &str, arch: &str, include_addr: bool, include_bytes: bool) -> anyhow::Result<String> {
    tracing::info!("disassemble_impl(input: {input_data:?}, base_addr: {base_addr:?}, arch: {arch:?}, include_addr: {include_addr:?}, include_bytes: {include_bytes:?})");
    if input_data.is_empty() {
        return Err(anyhow::anyhow!("Give me something to disassemble first"));
    }

    // Parse base addr
    let base_addr = if let Some(stripped) = base_addr.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16)
    } else {
        base_addr.parse::<u64>()
    };
    let base_addr = base_addr.context("Failed to parse number")?;

    let x = hex::decode(input_data.replace(" ", "")).context("I can't decode that as hex, sorry")?;

    let arch = if arch == "Guess for me" {
        match guess_arch(&x) {
            Ok(a) => a,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to guess arch: {:?}", e));
            }
        }
    } else {
        match ARCH_MAP.get(arch) {
            Some(v) => *v,
            None => return Err(anyhow::anyhow!("Bad arch")),
        }
    };

    let mut out = String::new();

    let y = UniDis::new_arch(arch).context("Failed to create arch")?;
    let mut x = y.dissassembler(x, 0).context("Failed to dis")?;

    while let Ok(Some(c)) = x.next_instruction() {
        if include_addr {
            out.push_str(&format!("{:08x}: ", c.address() + base_addr));
        }
        if include_bytes {
            for b in c.bytes() {
                out.push_str(&format!("{:02X} ", b));
            }
        }
        out.push_str("        ");
        out.push_str(&c.memonic());
        let mut args = c.args().into_iter().peekable();
        if args.peek().is_some() {
            out.push_str("    ");
        }
        while let Some(arg) = args.next() {
            out.push_str(&arg);
            if args.peek().is_some() {
                out.push_str(", ");
            }
        }
        out.push('\n');
    }
    println!("out = {out}");

    Ok(out)
}

pub fn assemble_impl(input_data: &str, base_addr: &str, arch: &str) -> anyhow::Result<String> {
    tracing::info!("assemble_impl(input: {input_data:?}, base_addr: {base_addr:?}, arch: {arch:?})");
    if input_data.is_empty() {
        return Err(anyhow::anyhow!("Give me something to assemble first"));
    }

    // Parse base addr
    let base_addr = if let Some(stripped) = base_addr.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16)
    } else {
        base_addr.parse::<u64>()
    };
    let base_addr = base_addr.context("Failed to parse number")?;
    
    use keystone_engine::*;

    let (arch, mode) = if arch == "Guess for me" {
        return Err(anyhow::anyhow!("I can't guess what architecture you want to assemble for!"));
    } else {
        match ARCH_MAP.get(arch) {
            Some(v) => match v {
                UnidisArch::X86_64 => (Arch::X86, Mode::MODE_64),
                UnidisArch::ArmV8Le => (Arch::ARM, Mode::V8),
                UnidisArch::AArch64 => (Arch::ARM64, Mode::LITTLE_ENDIAN),
                UnidisArch::Hexagon => (Arch::HEXAGON, Mode::LITTLE_ENDIAN),
                _ => {
                    return Err(anyhow::anyhow!("Unsupported architecture, currently we can only assemble for:\n- X86_64\n- Arm\n- ARM64\n- Hexagon\n"));
                },
            },
            None => {
                return Err(anyhow::anyhow!("Arch not found"));
            }
        }
    };

    let asm = match Keystone::new(arch, mode) {
        Ok(v) => v,
        Err(e) => return Err(anyhow::anyhow!("Keystone error: {:?}", e)),
    };

    let res = match asm.asm(input_data.to_string(), base_addr) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return Err(anyhow::anyhow!("Failed to assemble: {:?}", e));
        },
    };

    let mut out = String::new();
    for c in res.bytes {
        out.push_str(&format!("{:02X} ", c));
    }
    let out = out.trim().to_string();
    println!("out = {out:?}");

    // Returning dies here, probably unsoundness in keystone causing a fault on dropping the engine. Lets just do this for now
    emscripten_functions::emscripten::run_script(
        format!(
            r##"
                document.getElementById("output").value = "{}";
            "##,
            out.replace("\n", "\\n"),
        )
    );

    Ok(out)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn assemble(
    data_ptr: *mut u8,
    data_len: usize,

    base_addr_ptr: *mut u8,
    base_addr_len: usize,

    arch_ptr: *mut u8,
    arch_len: usize,
) -> i32 {
    tracing::info!("assemble({}, {data_len})", data_ptr as usize);
    if data_ptr.is_null() || base_addr_ptr.is_null() || arch_ptr.is_null() {
        return 1;
    }
    let input_data: &[u8] = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    let input_data = String::from_utf8(input_data.to_vec()).expect("Bad input_data");
    let base_addr = unsafe { std::slice::from_raw_parts(base_addr_ptr, base_addr_len) };
    let base_addr = String::from_utf8(base_addr.to_vec()).expect("Bad base_addr");
    let arch = unsafe { std::slice::from_raw_parts(arch_ptr, arch_len) };
    let arch = String::from_utf8(arch.to_vec()).expect("Bad arch");

    emscripten_functions::emscripten::run_script(
    r##"
            document.getElementById("output_title").textContent = "Assembly Output";
        "##
    );

    match assemble_impl(&input_data, &base_addr, &arch) {
        Ok(_out) => {
            // emscripten_functions::emscripten::run_script(
            //     format!(
            //         r##"
            //             document.getElementById("output").value = "{}";
            //         "##,
            //         out.replace("\n", "\\n"),
            //     )
            // );
            0
        }
        Err(e) => {
            println!("Assembly error: {:?}", e);
            emscripten_functions::emscripten::run_script(
                format!(
                    r##"
                        document.getElementById("output").value = "{}";
                    "##,
                    e.to_string().replace("\n", "\\n"),
                )
            );
            1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dissassemble(
    data_ptr: *mut u8,
    data_len: usize,

    base_addr_ptr: *mut u8,
    base_addr_len: usize,

    arch_ptr: *mut u8,
    arch_len: usize,

    include_addr: i32,
    include_bytes: i32,
) -> i32 {
    tracing::info!("dissassemble({}, {data_len})", data_ptr as usize);
    if data_ptr.is_null() || base_addr_ptr.is_null() || arch_ptr.is_null() {
        return 1;
    }
    let input_data: &[u8] = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    let input_data = String::from_utf8(input_data.to_vec()).expect("Bad input_data");
    let base_addr = unsafe { std::slice::from_raw_parts(base_addr_ptr, base_addr_len) };
    let base_addr = String::from_utf8(base_addr.to_vec()).expect("Bad base_addr");
    let arch = unsafe { std::slice::from_raw_parts(arch_ptr, arch_len) };
    let arch = String::from_utf8(arch.to_vec()).expect("Bad arch");

    emscripten_functions::emscripten::run_script(
        r##"
            document.getElementById("output_title").textContent = "Disassembly Output";
        "##,
    );

    match disassemble_impl(&input_data, &base_addr, &arch, include_addr != 0, include_bytes != 0) {
        Ok(out) => {
            println!("Disas ok");
            emscripten_functions::emscripten::run_script(
                format!(
                    r##"
                        document.getElementById("output").value = "{}";
                    "##,
                    out.replace("\n", "\\n"),
                )
            );

            0
        }
        Err(e) => {
            println!("Disassembly error: {:?}", e);
            emscripten_functions::emscripten::run_script(
                format!(
                    r##"
                        document.getElementById("output").value = "{}";
                    "##,
                    e.to_string().replace("\n", "\\n"),
                )
            );

            1
        }
    }

}

pub fn main() {
    tracing_subscriber::fmt::fmt()
        .with_ansi(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy()
        )
        .init();
    tracing::info!("starting up");
    println!("Started");
    load();
}

#[test]
pub fn foo() {
    guess_arch(b"Test");
    assemble_impl("nop", "0x0", "X86_64::LE::default");
}