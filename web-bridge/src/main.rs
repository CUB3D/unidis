use std::sync::LazyLock;
use std::collections::BTreeMap;
use tracing::instrument;
use tracing::level_filters::LevelFilter;
use libundis::{ARCHES, UniDis, UnidisArch};

pub fn get_arch_map() -> BTreeMap<String, UnidisArch> {
    tracing::info!("get_arch_map(_)");

    let mut o = BTreeMap::new();
    for a in ARCHES {
        o.insert(a.get_arch_id().to_string(), a.get_arch());
    }
    o
}

const ARCH_MAP: LazyLock<BTreeMap<String, UnidisArch>> = LazyLock::new(get_arch_map);

pub fn guess_arch(x: &[u8]) -> UnidisArch {
    tracing::info!("guess_arch(_)");

    let mut res = (0, UnidisArch::ArmV8Le);
    for a in ARCH_MAP.values() {
        let dis = UniDis::new_arch(*a).expect("new arch fail");
    //     let mut dis = dis.dissassembler(x.to_vec(), 0).expect("dis fail");
    //
    //     let mut c = 0;
    //     while let Ok(Some(i)) = dis.next_instruction() {
    //         c += i.bytes.len();
    //     }
    //     if c > res.0 {
    //         res = (c, *a);
    //     }
    }

    res.1
}

#[unsafe(no_mangle)]
pub extern "C" fn guess_arch_bridge(x: *mut u8, len: usize) -> usize {
    tracing::info!("guess_arch_bridge({}, {len})", x as usize);
    if x.is_null() {
        return 1;
    }
    let slc = unsafe { std::slice::from_raw_parts(x, len) };
    // print!("Guess arch");
    guess_arch(slc);

    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn alloc_bytes(size: usize, align: usize) -> *mut u8 {
    tracing::info!("alloc_bytes(size: {size}, align: {align})");
    let layout = std::alloc::Layout::from_size_align(size, align).expect("Alloc fail");
    unsafe { std::alloc::alloc(layout) }
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
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
}