use crate::dom::{Element, DOM};

#[derive(Debug)]
#[allow(dead_code)]
pub struct MemoryDOM {}

impl MemoryDOM {
    pub fn new() -> Box<DOM> {
        Box::new(MemoryDOM {})
    }
}

impl DOM for MemoryDOM {
    fn query_selector(&self, selector: &str) -> Box<Element> {
        Box::new(MemoryElement {
            tag: String::from(selector),
        })
    }
}

#[derive(Debug)]
pub struct MemoryElement {
    tag: String,
}

impl MemoryElement {
    pub fn new(tag: &str) -> Box<Element> {
        Box::new(MemoryElement {
            tag: String::from(tag),
        })
    }
}

impl Element for MemoryElement {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }
}
