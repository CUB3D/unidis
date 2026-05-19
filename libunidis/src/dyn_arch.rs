use crate::arch::Arch;
use crate::UnidisArch;

pub trait DynArch {
    fn get_pspec(&self) -> &'static str;
    fn get_sla(&self) -> &'static [u8];
    fn get_opinion(&self) -> &'static str;
    fn get_arch(&self) -> UnidisArch;
}

impl<T: Arch> DynArch for T {

    fn get_pspec(&self) -> &'static str {
        Self::PSPEC
    }

    fn get_sla(&self) -> &'static [u8] {
        Self::SLA
    }

    fn get_opinion(&self) -> &'static str {
        Self::OPINION
    }

    fn get_arch(&self) -> UnidisArch {
        Self::ARCH
    }
}