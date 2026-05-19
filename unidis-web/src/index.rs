use actix_web::HttpResponse;
use actix_web::web::Form;
use askama::Template;
use libundis::{ARCHES, UniDis, UnidisArch};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::LazyLock;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    output: String,
    arches: Vec<(String, bool)>,
    input_data: String,
    include_bytes: bool,
    include_address: bool,
    output_title: String,
}

pub fn get_arch_map() -> BTreeMap<String, UnidisArch> {
    let mut o = BTreeMap::new();
    for a in ARCHES {
        o.insert(a.get_arch_id().to_string(), a.get_arch());
    }
    o
}

const ARCH_MAP: LazyLock<BTreeMap<String, UnidisArch>> = LazyLock::new(get_arch_map);

pub async fn render_index_page(
    output: String,
    input_data: String,
    include_bytes: bool,
    include_address: bool,
    selected_arch: String,
    output_title: String,
) -> HttpResponse {
    let template = IndexTemplate {
        output,
        arches: ARCH_MAP.keys().cloned().map(|s| (s.clone(), s == selected_arch)).collect(),
        input_data,
        include_bytes,
        include_address,
        output_title,
    }
    .render()
    .expect("Unable to render template");
    HttpResponse::Ok().body(template)
}

pub async fn index_head() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn index_get() -> HttpResponse {
    render_index_page("".to_string(), "".to_string(), true, true, "".to_string(), "Output".to_string()).await
}

#[derive(Debug, Deserialize)]
pub struct DisReq {
    mode: String,

    input_data: String,
    arch: String,
    base_addr: String,
    include_bytes: Option<String>,
    include_addr: Option<String>,
}

/// Guess the architecture of the input bytes
/// This will attempt disassembly with all the known architectures, picking the one that successfully disassembles the largest number of bytes
pub fn guess_arch(x: &[u8]) -> UnidisArch {
    let mut res = (0, UnidisArch::Arm);
    for a in ARCH_MAP.values() {
        let mut dis = UniDis::new_arch(x.to_vec(), *a).unwrap();

        let mut c = 0;
        while let Ok(Some(i)) = dis.next() {
            c += i.bytes.len();
        }
        if c > res.0 {
            res = (c, *a);
        }
    }

    res.1
}

pub async fn index_post(b: Form<DisReq>) -> HttpResponse {
    tracing::info!("Incoming request: {:#?}", b);

    // Parse base addr
    let base_addr = if b.base_addr.starts_with("0x") {
        u64::from_str_radix(&b.base_addr[2..], 16)
    } else {
        b.base_addr.parse::<u64>()
    };
    let base_addr = match base_addr {
        Ok(base_addr) => base_addr,
        Err(e) => return HttpResponse::BadRequest().body(format!("{:?}", e)),
    };

    if b.mode == "DISASSEMBLE" {
        let x = match hex::decode(b.input_data.replace(" ", "")) {
            Ok(v) => v,
            Err(e) => return HttpResponse::BadRequest().body(format!("{:?}", e)),
        };

        if x.len() > 128 {
            return HttpResponse::BadRequest().body("Input data is too long");
        }

        let mut out = String::new();

        let arch = if b.arch == "Guess for me" {
            guess_arch(&x)
        } else {
            match ARCH_MAP.get(&b.arch) {
                Some(v) => *v,
                None => return HttpResponse::BadRequest().body("arch not found"),
            }
        };

        let mut x = UniDis::new_arch(x, arch).unwrap();

        while let Ok(Some(c)) = x.next() {
            if b.include_addr.is_some() {
                out.push_str(&format!("{:08x}: ", c.address() + base_addr));
            }
            if b.include_bytes.is_some() {
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

        render_index_page(out, b.input_data.clone(), b.include_bytes.is_some(), b.include_addr.is_some(), b.arch.clone(), "Disassembly Output".to_string()).await
    } else {
        use keystone_engine::*;
        let asm = match Keystone::new(Arch::X86, Mode::MODE_64) {
            Ok(v) => v,
            Err(e) => return HttpResponse::BadRequest().body(format!("{:?}", e)),
        };

        let res = match asm.asm(b.input_data.clone(), base_addr) {
            Ok(v) => v,
            Err(e) => {
                return render_index_page(format!("Failed to assemble: {e:?}"), b.input_data.clone(), b.include_bytes.is_some(), b.include_addr.is_some(), b.arch.clone(), "Assembly Output".to_string()).await;
            },
        };

        let mut out = String::new();
        for c in res.bytes {
            out.push_str(&format!("{:02X} ", c));
        }
        let out = out.trim().to_string();

        render_index_page(out, b.input_data.clone(), b.include_bytes.is_some(), b.include_addr.is_some(), b.arch.clone(), "Assembly Output".to_string()).await
    }
}
