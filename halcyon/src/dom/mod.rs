use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Element: Debug {
    fn get_tag(&self) -> String;
}

pub trait DOM: Debug {
    fn query_selector(&self, selector: &str) -> Rc<RefCell<Element>>;
}

pub mod memory;
