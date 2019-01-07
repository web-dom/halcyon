use halcyon::{Element, DOM};

type NodeHandle = usize;

#[derive(Debug)]
struct DOMData{
    tag:String
}

#[derive(Debug)]
pub struct MemoryDOM {
    root:NodeHandle,
    nodes:Vec<DOMData>
}

impl MemoryDOM {
    pub fn new() -> Box<DOM> {
        Box::new(MemoryDOM{
            root:0,
            nodes:vec![DOMData{tag:"body".to_string()}]
        })
    }
}

impl DOM for MemoryDOM {
    fn query_selector(&self, selector: &str) -> Option<Box<Element>> {
        if selector == "body" {
            return Some(Box::new(MemoryElement{node:self.root}));
        }
        panic!("not implemented yet")
    }
}

#[derive(Debug)]
pub struct MemoryElement {
    node:NodeHandle
}

impl Element for MemoryElement {
    fn get_tag(&self) -> String {
        return "div".to_string()
    }
}
