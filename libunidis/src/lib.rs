//! The universal dissassembler

use clap::ValueEnum;
use libsla::{
    Address, GhidraSleigh, InstructionBytes, NativeDisassembly, PcodeDisassembly, Sleigh,
    VarnodeData,
};

pub mod opinion;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum UnidisArch {
    X86_64,
    Arm,
    Hexagon,
    Riscv,
    AArch64,
}

pub trait Arch {
    const PSPEC: &'static str;
    const SLA: &'static [u8];
    const OPINION: &'static str;
    const ARCH: UnidisArch;
}

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

pub struct ArchX86;
impl Arch for ArchX86 {
    const PSPEC: &'static str = include_str!("../data/x86-64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/x86-64.sla");
    const OPINION: &'static str = include_str!("../data/x86.opinion");
    const ARCH: UnidisArch = UnidisArch::X86_64;
}

pub struct ArchArmV8;
impl Arch for ArchArmV8 {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM8_le.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
}


pub struct ArchHexagon;
impl Arch for ArchHexagon {
    const PSPEC: &'static str = include_str!("../data/Hexagon/data/languages/hexagon.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/Hexagon/data/languages/hexagon.sla");
    const OPINION: &'static str = include_str!("../data/Hexagon/data/languages/hexagon.opinion");
    const ARCH: UnidisArch = UnidisArch::Hexagon;
}

pub struct ArchRiscV;
impl Arch for ArchRiscV {
    const PSPEC: &'static str = include_str!("../data/RISCV/data/languages/RV64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/RISCV/data/languages/riscv.lp64d.sla");
    const OPINION: &'static str = include_str!("../data/RISCV/data/languages/riscv.opinion");
    const ARCH: UnidisArch = UnidisArch::Riscv;
}

pub struct ArchAArch64;
impl Arch for ArchAArch64 {
    const PSPEC: &'static str = include_str!("../data/AARCH64/data/languages/AARCH64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/AARCH64/data/languages/AARCH64.sla");
    const OPINION: &'static str = include_str!("../data/AARCH64/data/languages/AARCH64.opinion");
    const ARCH: UnidisArch = UnidisArch::AArch64;
}

pub const ARCHES: &[&dyn DynArch] = &[
    &ArchX86,
    &ArchArmV8,
    &ArchHexagon,
    &ArchRiscV,
    &ArchAArch64,
];

pub struct UniDisInstruction {
    pub res: NativeDisassembly,
    pub pcode: PcodeDisassembly,
    pub bytes: Vec<u8>,
}

impl UniDisInstruction {
    pub fn to_str(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.res.instruction.mnemonic);
        out.push(' ');
        out.push_str(&self.res.instruction.body);

        out
    }

    pub fn input_varnodes(&self) -> Vec<VarnodeData> {
        let mut all_in = Vec::new();
        for p in &self.pcode.instructions {
            all_in.extend_from_slice(&p.inputs);
        }
        all_in.dedup();

        for p in &self.pcode.instructions {
            if let Some(o) = &p.output {
                if let Some(pos) = all_in.iter().position(|p| p == o) {
                    all_in.remove(pos);
                }
            }
        }

        all_in
    }

    pub fn args(&self) -> Vec<String> {
        let body = &self.res.instruction.body;
        if body.contains(",") {
            body.split(",").map(|s| s.to_owned()).collect()
        } else {
            vec![body.to_owned()]
        }
    }

    pub fn address(&self) -> u64 {
        self.res.origin.address.offset
    }

    pub fn memonic(&self) -> String {
        self.res.instruction.mnemonic.clone()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub struct UniDis {
    pub sleigh: GhidraSleigh,
    pub instructions: InstructionBytes,
    pub data: Vec<u8>,

    pub current_pos: Address,
}

impl UniDis {
    pub fn new_arch(d: Vec<u8>, arc: UnidisArch) -> anyhow::Result<Self> {
        let x = match arc {
            UnidisArch::X86_64 => UniDis::new::<ArchX86>(d)?,
            UnidisArch::Arm => UniDis::new::<ArchArmV8>(d)?,
            UnidisArch::Hexagon => UniDis::new::<ArchHexagon>(d)?,
            UnidisArch::Riscv => UniDis::new::<ArchRiscV>(d)?,
            UnidisArch::AArch64 => UniDis::new::<ArchAArch64>(d)?,
        };
        Ok(x)
    }

    pub fn new<Ar: Arch>(data: Vec<u8>) -> anyhow::Result<Self> {
        let sleigh = GhidraSleigh::builder()
            .processor_spec(Ar::PSPEC)?
            .build(Ar::SLA)?;

        let address_space = sleigh.default_code_space();
        let current_pos = Address::new(address_space, 0);

        Ok(Self {
            sleigh,
            instructions: InstructionBytes::new(data.clone()),
            data,
            current_pos,
        })
    }

    pub fn dissassemble(&self) -> anyhow::Result<UniDisInstruction> {
        let native_disassembly = self
            .sleigh
            .disassemble_native(&self.instructions, self.current_pos.clone())?;
        let pcode = self
            .sleigh
            .disassemble_pcode(&self.instructions, self.current_pos.clone())?;
         //println!("{:#?}", native_disassembly);

        let bytes = self.data[self.current_pos.offset as usize..]
            [..native_disassembly.origin.size as usize]
            .to_vec();

        Ok(UniDisInstruction {
            res: native_disassembly,
            pcode,
            bytes,
        })
    }

    pub fn skip_bytes(&mut self, cnt: u64) {
        self.current_pos.offset += cnt;
    }

    pub fn next(&mut self) -> anyhow::Result<Option<UniDisInstruction>> {
        if self.current_pos.offset >= self.data.len() as u64 {
            return Ok(None);
        }
        let i = self.dissassemble()?;
        self.current_pos.offset += i.res.origin.size as u64;
        Ok(Some(i))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
pub mod test {
    use std::error::Error;

    use crate::{ArchX86, UniDis};

    #[test]
    pub fn test_1() -> Result<(), Box<dyn Error>> {
        let mut x86 = UniDis::new::<ArchX86>(vec![
            0x89, 0xF0, 0x85, 0xFF, 0x74, 0x09, 0x99, 0xF7, 0xFF, 0x89, 0xF8, 0x89, 0xD7, 0xEB,
            0xF3, 0xC3,
        ])?;
        while let Some(i) = x86.next()? {
            println!("{}", i.to_str());
        }
        Ok(())
    }
}
