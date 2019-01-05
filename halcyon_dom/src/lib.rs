use halcyon::{Element, DOM};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebIDLDOM {}

impl WebIDLDOM {
    pub fn new() -> Box<DOM> {
        Box::new(WebIDLDOM {})
    }
}

#[derive(Debug)]
pub struct WebIDLElement {
    tag: String,
}

impl WebIDLElement {
    pub fn new(tag: &str) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(WebIDLElement {
            tag: String::from(tag),
        }))
    }
}

impl Element for WebIDLElement {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

impl WebIDLDOM {
    pub fn query_selector(selector: &str) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(WebIDLElement {
            tag: String::from(selector),
        }))
    }
}

impl DOM for WebIDLDOM {
    fn query_selector(&self, selector: &str) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(WebIDLElement {
            tag: String::from(selector),
        }))
    }
}
