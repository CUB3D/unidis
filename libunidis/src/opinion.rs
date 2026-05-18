use std::str::FromStr;
use xmltree::Element;
use crate::UnidisArch;

pub struct Opinions {
}

impl Opinions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn lookup_elf(&self, tgt: u64) -> Option<UnidisArch> {
        for arch in crate::ARCHES {
            let of = OpinionFile::from_bytes(arch.get_opinion().as_bytes());
            if of.find_elf(tgt).is_some() {
                return Some(arch.get_arch());
            }
        }

        None
    }
}

pub struct OpinionFile {
    pub constraint: Element,
}

impl OpinionFile {
    pub fn from_bytes(d: &[u8]) -> Self {
        let tree = Element::parse(&d[..]);
        Self { constraint: tree.unwrap() }
    }

    pub fn find_elf(&self, tgt: u64) -> Option<()> {
        for c in self.constraint.children.iter() {
            if let Some(e) = c.as_element() {
                if e.name == "constraint" && e.attributes.iter().any(|(k, v)| k == "loader" && v == "Executable and Linking Format (ELF)") {

                    for c in e.children.iter() {
                        if let Some(e) = c.as_element() {

                            if let Some((_, prim)) = e.attributes.iter().find(|&(k, _)| k == "primary") {
                                // println!("{}", prim);
                                let v = u64::from_str(prim).unwrap();
                                if tgt == v {
                                    return Some(());
                                }
                            }


                            for c in e.children.iter() {
                                if let Some(e) = c.as_element() {
                                    if let Some((_, prim)) = e.attributes.iter().find(|&(k, _)| k == "primary") {
                                        // println!("{}", prim);
                                        let v = u64::from_str(prim).unwrap();
                                        if tgt == v {
                                            return Some(());
                                        }
                                    }
                                }
                            }


                        }
                    }


                }
            }
        }

        None
    }
}
