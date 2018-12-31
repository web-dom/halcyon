use std::rc::Rc;
use std::cell::RefCell;

pub  trait Element {
    fn get_tag(&self) -> String;
}

pub trait DOM {
    fn query_selector(&self,selector:&str) -> Rc<RefCell<Element>>;
}

pub struct MemoryDOM {}

impl MemoryDOM {
    pub fn new() -> Box<DOM> {
        Box::new(MemoryDOM{})
    }
}

pub struct MemoryElement {
    tag:String
}

impl MemoryElement {
    pub fn new(tag:&str) -> Rc<RefCell<Element>>{
        Rc::new(RefCell::new(MemoryElement{
            tag:String::from(tag)
        }))
    }
}

impl Element for MemoryElement{
    fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

impl DOM for MemoryDOM {
    fn query_selector(&self,selector:&str) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(MemoryElement{
            tag:String::from(selector)
        }))
    }
}
