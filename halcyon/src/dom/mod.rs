use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Element: Debug {
    fn get_tag(&self) -> String;
}

pub trait DOM: Debug {
    fn query_selector(&self, selector: &str) -> Rc<RefCell<Element>>;
}

#[derive(Debug)]
pub struct MemoryDOM {}

impl MemoryDOM {
    pub fn new() -> Box<DOM> {
        Box::new(MemoryDOM {})
    }
}

#[derive(Debug)]
pub struct MemoryElement {
    tag: String,
}

impl MemoryElement {
    pub fn new(tag: &str) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(MemoryElement {
            tag: String::from(tag),
        }))
    }
}

impl Element for MemoryElement {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

impl DOM for MemoryDOM {
    fn query_selector(&self, selector: &str) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(MemoryElement {
            tag: String::from(selector),
        }))
    }
}
