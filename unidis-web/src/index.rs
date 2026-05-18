use std::collections::BTreeMap;
use std::sync::LazyLock;
use actix_web::{HttpResponse};
use actix_web::web::Form;
use askama::Template;
use libundis::{UniDis, UnidisArch, ARCHES};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    output: String,
    arches: Vec<String>,
    input_data: String,
}

pub fn get_arch_map() -> BTreeMap<String, UnidisArch> {
    let mut o = BTreeMap::new();
    for a in ARCHES {
        o.insert(format!("{:?}", a.get_arch()), a.get_arch());
    }
    o
}

const ARCH_MAP: LazyLock<BTreeMap<String, UnidisArch>> = LazyLock::new(get_arch_map);

pub async fn render_index_page(
    output: String,
    input_data: String,
) -> HttpResponse {
        let template = IndexTemplate {
        output,
        arches: ARCH_MAP.keys().cloned().collect(),
            input_data,
    }
    .render()
    .expect("Unable to render template");
    HttpResponse::Ok().body(template)
}

pub async fn index_head() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn index_get() -> HttpResponse {
    render_index_page("".to_string(), "".to_string()).await
}

#[derive(Debug, Deserialize)]
pub struct DisReq {
    input_data: String,
    arch: String,
    base_addr: String,
}

pub async fn index_post(b: Form<DisReq>) -> HttpResponse {
    tracing::info!("Incoming request: {:#?}", b);
    let x = match hex::decode(b.input_data.replace(" ", "")) {
        Ok(v) => v,
        Err(e) => return HttpResponse::BadRequest().body(format!("{:?}", e)),
    };

    if x.len() > 128 {
        return HttpResponse::BadRequest().body("Input data is too long");
    }

    let base_addr = if b.base_addr.starts_with("0x") {
        u64::from_str_radix(&b.base_addr[2..], 16)
    } else {
        b.base_addr.parse::<u64>()
    };
    let base_addr = match base_addr {
        Ok(base_addr) => base_addr,
        Err(e) => return HttpResponse::BadRequest().body(format!("{:?}", e)),
    };

    let mut out = String::new();

    let arch = match ARCH_MAP.get(&b.arch) {
        Some(v) => *v,
        None => return HttpResponse::BadRequest().body("arch not found"),
    };

    let mut x = UniDis::new_arch(x, arch).unwrap();

    while let Ok(Some(c)) = x.next() {
        out.push_str(&format!("{:08x}: ", c.address() + base_addr));
        for b in c.bytes() {
            out.push_str(&format!("{:02X} ", b));
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

    render_index_page(out, b.input_data.clone()).await
}