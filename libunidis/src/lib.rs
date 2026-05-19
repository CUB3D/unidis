//! The universal dissassembler

use clap::ValueEnum;
use libsla::{
    Address, GhidraSleigh, InstructionBytes, NativeDisassembly, PcodeDisassembly, Sleigh,
    VarnodeData,
};
use crate::arch::Arch;
use crate::dyn_arch::DynArch;

pub mod opinion;
mod arch;
mod dyn_arch;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum UnidisArch {
    X86_64,
    Arm,
    Hexagon,
    Riscv,
    AArch64,
}

pub struct ArchX86;
impl Arch for ArchX86 {
    const PSPEC: &'static str = include_str!("../data/x86-64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/x86-64.sla");
    const OPINION: &'static str = include_str!("../data/x86.opinion");
    const ARCH: UnidisArch = UnidisArch::X86_64;
    const ARCH_ID: &'static str = "X86_64::LE::default";
}

pub struct ArchArmV8Le;
impl Arch for ArchArmV8Le {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM8_le.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::LE::V8";
}

pub struct ArchArmV8Be;
impl Arch for ArchArmV8Be {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM8_be.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::BE::V8";
}

pub struct ArchArmV8mBe;
impl Arch for ArchArmV8mBe {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM8m_be.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::BE::V8m";
}

pub struct ArchArmV8mLe;
impl Arch for ArchArmV8mLe {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM8m_le.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::LE::V8m";
}

pub struct ArchArmV7Le;
impl Arch for ArchArmV7Le {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM7_le.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::LE::V7";
}

pub struct ArchArmV7Be;
impl Arch for ArchArmV7Be {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM7_be.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::BE::V7";
}

pub struct ArchArmV6Le;
impl Arch for ArchArmV6Le {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM6_le.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::LE::V6";
}

pub struct ArchArmV6Be;
impl Arch for ArchArmV6Be {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM6_be.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::BE::V6";
}

pub struct ArchArmV5Le;
impl Arch for ArchArmV5Le {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM5_le.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::LE::V5";
}

pub struct ArchArmV5Be;
impl Arch for ArchArmV5Be {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM5_be.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::BE::V5";
}

pub struct ArchArmV5tLe;
impl Arch for ArchArmV5tLe {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM5t_le.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::LE::V5t";
}

pub struct ArchArmV5tBe;
impl Arch for ArchArmV5tBe {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM5t_be.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::BE::V5t";
}

pub struct ArchArmV4Le;
impl Arch for ArchArmV4Le {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM4_le.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::LE::V4";
}

pub struct ArchArmV4Be;
impl Arch for ArchArmV4Be {
    const PSPEC: &'static str = include_str!("../data/ARM/data/languages/ARMCortex.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/ARM/data/languages/ARM4_be.sla");
    const OPINION: &'static str = include_str!("../data/ARM/data/languages/ARM.opinion");
    const ARCH: UnidisArch = UnidisArch::Arm;
    const ARCH_ID: &'static str = "ARM::BE::V4";
}

pub struct ArchHexagon;
impl Arch for ArchHexagon {
    const PSPEC: &'static str = include_str!("../data/Hexagon/data/languages/hexagon.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/Hexagon/data/languages/hexagon.sla");
    const OPINION: &'static str = include_str!("../data/Hexagon/data/languages/hexagon.opinion");
    const ARCH: UnidisArch = UnidisArch::Hexagon;
    const ARCH_ID: &'static str = "QDSP6::LE::default";
}

pub struct ArchRiscV64;
impl Arch for ArchRiscV64 {
    const PSPEC: &'static str = include_str!("../data/RISCV/data/languages/RV64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/RISCV/data/languages/riscv.lp64d.sla");
    const OPINION: &'static str = include_str!("../data/RISCV/data/languages/riscv.opinion");
    const ARCH: UnidisArch = UnidisArch::Riscv;
    const ARCH_ID: &'static str = "RISCV::64::LE::default";
}

pub struct ArchRiscV32;
impl Arch for ArchRiscV32 {
    const PSPEC: &'static str = include_str!("../data/RISCV/data/languages/RV32.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/RISCV/data/languages/riscv.ilp32d.sla");
    const OPINION: &'static str = include_str!("../data/RISCV/data/languages/riscv.opinion");
    const ARCH: UnidisArch = UnidisArch::Riscv;
    const ARCH_ID: &'static str = "RISCV::32::LE::default";
}

pub struct ArchRiscV64Andestar;
impl Arch for ArchRiscV64Andestar {
    const PSPEC: &'static str = include_str!("../data/RISCV/data/languages/RV64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/RISCV/data/languages/andestar_v5.sla");
    const OPINION: &'static str = include_str!("../data/RISCV/data/languages/riscv.opinion");
    const ARCH: UnidisArch = UnidisArch::Riscv;
    const ARCH_ID: &'static str = "RISCV::64::LE::Andestar";
}

pub struct ArchAArch64Le;
impl Arch for ArchAArch64Le {
    const PSPEC: &'static str = include_str!("../data/AARCH64/data/languages/AARCH64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/AARCH64/data/languages/AARCH64.sla");
    const OPINION: &'static str = include_str!("../data/AARCH64/data/languages/AARCH64.opinion");
    const ARCH: UnidisArch = UnidisArch::AArch64;
    const ARCH_ID: &'static str = "AArch64::LE::default";
}

pub struct ArchAArch64Be;
impl Arch for ArchAArch64Be {
    const PSPEC: &'static str = include_str!("../data/AARCH64/data/languages/AARCH64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/AARCH64/data/languages/AARCH64BE.sla");
    const OPINION: &'static str = include_str!("../data/AARCH64/data/languages/AARCH64.opinion");
    const ARCH: UnidisArch = UnidisArch::AArch64;
    const ARCH_ID: &'static str = "AArch64::BE::default";
}

pub struct ArchAArch64Apple;
impl Arch for ArchAArch64Apple {
    const PSPEC: &'static str = include_str!("../data/AARCH64/data/languages/AARCH64.pspec");
    const SLA: &'static [u8] = include_bytes!("../data/AARCH64/data/languages/AARCH64_AppleSilicon.sla");
    const OPINION: &'static str = include_str!("../data/AARCH64/data/languages/AARCH64.opinion");
    const ARCH: UnidisArch = UnidisArch::AArch64;
    const ARCH_ID: &'static str = "AArch64::LE::Apple-Silicon";
}

pub const ARCHES: &[&dyn DynArch] = &[
    &ArchX86,
    &ArchArmV8mLe,
    &ArchArmV8mBe,
    &ArchArmV8Le,
    &ArchArmV8Be,
    &ArchArmV7Le,
    &ArchArmV7Be,
    &ArchArmV6Le,
    &ArchArmV6Be,
    &ArchArmV5Le,
    &ArchArmV5Be,
    &ArchArmV5tLe,
    &ArchArmV5tBe,
    &ArchArmV4Le,
    &ArchArmV4Be,
    &ArchHexagon,
    &ArchRiscV64,
    &ArchRiscV32,
    &ArchRiscV64Andestar,
    &ArchAArch64Le,
    &ArchAArch64Be,
    &ArchAArch64Apple,
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
            UnidisArch::Arm => UniDis::new::<ArchArmV8Le>(d)?,
            UnidisArch::Hexagon => UniDis::new::<ArchHexagon>(d)?,
            UnidisArch::Riscv => UniDis::new::<ArchRiscV64>(d)?,
            UnidisArch::AArch64 => UniDis::new::<ArchAArch64Le>(d)?,
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
            [..native_disassembly.origin.size]
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
