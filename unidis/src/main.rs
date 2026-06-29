use std::{io::stdout, path::PathBuf};

use clap::Parser;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use goblin::Object;
use libundis::opinion::Opinions;
use libundis::{UniDis, UnidisArch};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to disassemble
    #[arg()]
    pub tgt: PathBuf,

    #[arg(short, long)]
    pub arch: Option<UnidisArch>,

    /// Set the number of instructions to disassemble
    #[arg(short, long)]
    pub count: Option<i64>,
}

pub fn dis_binary(arch: UnidisArch, d: Vec<u8>, address_offset: u64, count_limit: &mut Option<i64>) -> anyhow::Result<()> {
    let dis = UniDis::new_arch(arch)?;

    let mut dis = dis.dissassembler(d, address_offset)?;

    loop {
        let nx = dis.next_instruction();

        // Handle count limit
        if let Some(tgt_cnt) = count_limit {
            *tgt_cnt -= 1;
            if *tgt_cnt <= 0 {
                return Ok(());
            }
        }

        if let Ok(d) = nx {
            if let Some(d) = d {
                stdout()
                    .execute(SetForegroundColor(Color::Blue))?
                    .execute(Print(format!("{:08x}", d.address() + address_offset)))?
                    .execute(SetForegroundColor(Color::White))?
                    .execute(Print(": "))?
                    .execute(ResetColor)?;

                for b in d.bytes() {
                    stdout()
                        .execute(Print(format!("{:02x} ", b)))?
                        .execute(ResetColor)?;
                }

                let pad = 50usize
                    .checked_sub(8)
                    .unwrap()
                    .checked_sub(2)
                    .unwrap()
                    .checked_sub(3 * d.bytes().len())
                    .unwrap();
                stdout().execute(Print(" ".repeat(pad)))?;

                stdout()
                    .execute(SetForegroundColor(Color::White))?
                    .execute(Print(format!("{} ", d.memonic())))?;

                let pad = 16usize.checked_sub(d.memonic().len()).unwrap();
                stdout().execute(Print(" ".repeat(pad)))?;

                let args = d.args();
                let mut args = args.iter().peekable();
                while let Some(arg) = args.next() {
                    stdout()
                        .execute(SetForegroundColor(Color::Yellow))?
                        .execute(Print(arg.to_string()))?
                        .execute(ResetColor)?;

                    if args.peek().is_some() {
                        stdout().execute(Print(", "))?;
                    }
                }

                stdout().execute(ResetColor)?.execute(Print("\n"))?;
            } else {
                return Ok(());
            }
        } else {
            stdout()
                .execute(Print("<Unknown>".to_string()))?
                .execute(ResetColor)?
                .execute(Print("\n"))?;
            dis.skip_bytes(1);
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let d = std::fs::read(&args.tgt)?;

    let mut count_limit = args.count;

    // let elf_bytes = std::fs::read("../../../riscv-emu/x64_emu/run/test_c/a.out").expect("Failed to load binary");

    let bin = goblin::Object::parse(d.as_slice())?;

    match bin {
        // Object::Mach(m) => {
        //     match m {
        //         Mach::Fat(f) => {
        //             let f = f.get(0).unwrap();
        //             match f {
        //                 SingleArch::MachO(macho) => {
        //                     for s in &macho.segments {
        //                         s.data
        //                     }
        //                 }
        //                 _ => panic!()
        //             }
        //         }
        //         _ => panic!()
        //     }
        // }
        Object::Elf(elf) => {
            // println!("{}", elf.header.e_machine);
            let elf_arch = Opinions::default().lookup_elf(elf.header.e_machine as _).unwrap();

            let arch = args.arch.unwrap_or(elf_arch);

            for section_header in elf.section_headers.iter() {
                if !section_header.is_executable() {
                    continue;
                }

                let n = elf.shdr_strtab.get_at(section_header.sh_name).unwrap();
                println!("{}", n);
                let file_data = d.get(section_header.file_range().unwrap()).unwrap();
                dis_binary(arch, file_data.to_vec(), section_header.sh_addr, &mut count_limit)?;
            }

            // for section in &elf.program_headers {
            //     // Ignore non-loadable sections
            //     if section.p_type != goblin::elf::program_header::PT_LOAD {
            //         continue;
            //     }
            //
            //     if !section.is_executable() {
            //         continue;
            //     }
            //
            //     let (size, _align) = (section.p_memsz as usize, 0usize);
            //
            //     // Size is inclusive
            //     let mut section_data = vec![0u8; size];
            //
            //     // Fill in the filesize bytes with the contents of the file, the rest will be left set to 0
            //     section_data[..section.p_filesz as usize].copy_from_slice(d.get((section.p_offset as usize)..(section.p_offset as usize + section.p_filesz as usize)).unwrap());
            //
            //     println!("{:08x}", elf.header.e_entry);
            //     println!("{:08x}", elf.section_headers.iter().nth(4).unwrap().sh_addr);
            //     println!("{:08x}", section.p_vaddr);
            //     println!("{:x?}", &section_data[..0x10]);
            //     dis_binary(&args, arch, section_data)?;
            // }
        }
        _ => panic!(),
    }




    Ok(())
}
