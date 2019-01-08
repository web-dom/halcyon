use crate::{t, Element, Props};

#[derive(Debug, PartialEq)]
pub enum VirtualNode {
    Element(VirtualNodeElement),
    Text(VirtualNodeText),
}

impl VirtualNode {
    pub fn same(&self, other: &VirtualNode) -> bool {
        match self {
            VirtualNode::Element(e) => match other {
                VirtualNode::Element(oe) => e.list_key == oe.list_key && e.selector == oe.selector,
                _ => false,
            },
            VirtualNode::Text(e) => match other {
                VirtualNode::Text(oe) => e.text == oe.text,
                _ => false,
            },
        }
    }

    pub fn get_parent_element(&self) -> Option<Box<Element>> {
        match self {
            VirtualNode::Element(e) => match &e.element {
                Some(el) => el.get_parent(),
                None => None,
            },
            VirtualNode::Text(e) => match &e.element {
                Some(el) => el.get_parent(),
                None => None,
            },
        }
    }

    pub fn set_element(&mut self, element: Box<Element>) {
        match self {
            VirtualNode::Element(e) => e.element = Some(element),
            VirtualNode::Text(e) => e.element = Some(element),
        }
    }

    pub fn get_element(&self) -> Option<&Box<Element>> {
        match self {
            VirtualNode::Element(e) => match e.element.as_ref() {
                Some(el) => Some(el),
                None => None,
            },
            VirtualNode::Text(e) => match e.element.as_ref() {
                Some(el) => Some(el),
                None => None,
            },
        }
    }
}

impl<T> From<T> for VirtualNode
where
    T: std::fmt::Display,
{
    fn from(v: T) -> VirtualNode {
        t(&format!("{}", v))
    }
}

impl VirtualNode {
    pub fn from_element(e: Box<Element>) -> VirtualNode {
        VirtualNode::Element(VirtualNodeElement {
            selector: String::from("div"),
            data: None,
            children: None,
            element: Some(e),
            list_key: None,
        })
    }
}

#[derive(Debug)]
pub struct VirtualNodeElement {
    pub selector: String,
    pub data: Option<Props>,
    pub children: Option<Vec<VirtualNode>>,
    pub element: Option<Box<Element>>,
    pub list_key: Option<String>,
}

impl PartialEq for VirtualNodeElement {
    fn eq(&self, other: &VirtualNodeElement) -> bool {
        self.selector == other.selector
            && self.data == other.data
            && self.list_key == other.list_key
            && self.children == other.children
    }
}

#[derive(Debug)]
pub struct VirtualNodeText {
    pub element: Option<Box<Element>>,
    pub text: String,
}

impl PartialEq for VirtualNodeText {
    fn eq(&self, other: &VirtualNodeText) -> bool {
        self.text == other.text
    }
}
