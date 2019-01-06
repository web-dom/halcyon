use std::fmt::Debug;

pub trait Element: Debug {
    fn get_tag(&self) -> String;
}

pub trait DOM: Debug {
    fn query_selector(&self, selector: &str) -> Box<Element>;
}

pub mod memory;
