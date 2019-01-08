use std::fmt::Debug;

pub trait Element: Debug {
    fn get_tag(&self) -> String;
    fn get_parent(&self) -> Option<Box<Element>>;
}

pub trait DOM: Debug {
    fn query_selector(&self, selector: &str) -> Option<Box<Element>>;
    fn create_text_node(&self, txt: &str) -> Box<Element>;
}
