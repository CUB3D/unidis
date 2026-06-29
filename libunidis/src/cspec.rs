use xmltree::Element;

pub enum CspecRetaddr {
    Varnode { space: String, offset: i64, size: i64 },
    Register { name: String },
}

pub struct CompilerSpec {
    pub root: Element,

    pub return_addr: CspecRetaddr,
}

impl CompilerSpec {
    pub fn from_bytes(d: &[u8]) -> Self {
        let root = Element::parse(d).unwrap();

        let retaddr = root.get_child("returnaddress").unwrap();
        let childel = retaddr.children.first().unwrap().as_element().unwrap();
        let return_addr = match childel.name.as_ref() {
            "register" => {
                let n = childel.attributes.get("name");
                CspecRetaddr::Register {name: n.unwrap().clone()}
            }
            "varnode" => {
                let space = childel.attributes.get("space").unwrap();
                let offset = childel.attributes.get("offset").unwrap();
                let size = childel.attributes.get("size").unwrap();

                CspecRetaddr::Varnode {
                    space: space.to_string(),
                    offset: offset.parse::<i64>().unwrap(),
                    size: size.parse::<i64>().unwrap(),
                }
            }
            _ => panic!()
        };


        Self {
            root: Element::parse(d).unwrap(),
            return_addr,
        }
    }
}