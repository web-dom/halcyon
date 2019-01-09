use std::fmt::Debug;

pub trait Element: Debug + PartialEq + Sized {
    fn get_tag(&self) -> String;
    fn get_parent(&self) -> Option<Self>;
    fn next_sibling(&self) -> Option<Self>;
    fn insert_before(&mut self, element: &Self, target: Option<&mut Self>);
    fn remove(&mut self);
    fn append_child(&mut self, element: &Self);
    fn set_attribute(&mut self, name: &str, value: &str);
}

pub trait DOM<E>: Debug
where
    E: Element,
{
    fn query_selector(&self, tag: &str) -> Option<E>;
    fn create_node(&self, tag: &str) -> E;
    fn create_text_node(&self, txt: &str) -> E;
}
