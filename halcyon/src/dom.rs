use std::fmt::Debug;

pub trait Element: Debug + PartialEq + Sized {
    fn get_tag(&self) -> String;
    fn get_parent(&self) -> Option<Self>;
    fn next_sibling(&self) -> Option<Self>;
    fn insert_before(&mut self, element_to_insert:&Self,target:Option<&Self>);
}

pub trait DOM<E>: Debug
where
    E: Element,
{
    fn query_selector(&self, selector: &str) -> Option<E>;
    fn create_node(&self, tag: &str) -> E;
    fn create_text_node(&self, txt: &str) -> E;
}
