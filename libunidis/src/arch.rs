use crate::UnidisArch;

pub trait Arch {
    const PSPEC: &'static str;
    const SLA: &'static [u8];
    const OPINION: &'static str;
    const ARCH: UnidisArch;
}
