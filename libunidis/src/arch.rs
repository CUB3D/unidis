use crate::UnidisArch;

pub trait Arch {
    const PSPEC: &'static str;
    const CSPEC: &'static str;
    const SLA: &'static [u8];
    const OPINION: Option<&'static str>;
    const ARCH: UnidisArch;
    const ARCH_ID: &'static str;
}
