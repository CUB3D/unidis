use crate::arch::Arch;
use crate::cspec::CompilerSpec;
use crate::UnidisArch;

pub trait DynArch {
    fn get_pspec(&self) -> &'static str;
    fn get_sla(&self) -> &'static [u8];
    fn get_opinion(&self) -> Option<&'static str>;
    fn get_arch(&self) -> UnidisArch;

    fn get_arch_id(&self) -> &'static str;
    fn get_cspec(&self) -> &'static str;

    fn read_cspec(&self) -> CompilerSpec {
        CompilerSpec::from_bytes(self.get_cspec().as_bytes())
    }
}

impl<T: Arch> DynArch for T {

    fn get_pspec(&self) -> &'static str {
        Self::PSPEC
    }

    fn get_sla(&self) -> &'static [u8] {
        Self::SLA
    }

    fn get_opinion(&self) -> Option<&'static str> {
        Self::OPINION
    }

    fn get_arch(&self) -> UnidisArch {
        Self::ARCH
    }

    fn get_arch_id(&self) -> &'static str {
        Self::ARCH_ID
    }

    fn get_cspec(&self) -> &'static str { Self::CSPEC }
}