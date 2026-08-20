use std::sync::LazyLock;
use std::collections::BTreeMap;

use libundis::{ARCHES, UniDis, UnidisArch};

pub fn get_arch_map() -> BTreeMap<String, UnidisArch> {
    let mut o = BTreeMap::new();
    for a in ARCHES {
        o.insert(a.get_arch_id().to_string(), a.get_arch());
    }
    o
}

const ARCH_MAP: LazyLock<BTreeMap<String, UnidisArch>> = LazyLock::new(get_arch_map);

pub fn guess_arch(x: &[u8]) -> UnidisArch {
    let mut res = (0, UnidisArch::ArmV8Le);
    for a in ARCH_MAP.values() {
        let dis = UniDis::new_arch(*a).expect("new arch fail");
        let mut dis = dis.dissassembler(x.to_vec(), 0).expect("dis fail");

        let mut c = 0;
        while let Ok(Some(i)) = dis.next_instruction() {
            c += i.bytes.len();
        }
        if c > res.0 {
            res = (c, *a);
        }
    }

    res.1
}

#[unsafe(no_mangle)]
pub extern "C" fn guess_arch_bridge(x: *const u8, len: usize) -> usize {
    let slc = unsafe { std::slice::from_raw_parts(x, len) };
    print!("Guess arch");
    guess_arch(slc);

    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_bytes(size: usize, align: usize) -> *mut u8 {
    use std::alloc::{Layout, alloc};
    let layout = Layout::from_size_align(size, align).expect("Alloc fail");
    alloc(layout)
}

pub fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
}