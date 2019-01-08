use crate::{t, Element, Props};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum VirtualNode<E: Element> {
    Element(VirtualNodeElement<E>),
    Text(VirtualNodeText<E>),
}

impl<E> VirtualNode<E>
where
    E: Element,
{
    pub fn same(&self, other: &VirtualNode<E>) -> bool {
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

    pub fn get_parent_element(&self) -> Option<E> {
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

    pub fn set_element(&mut self, element: E) {
        match self {
            VirtualNode::Element(e) => e.element = Some(element),
            VirtualNode::Text(e) => e.element = Some(element),
        }
    }

    pub fn get_element(&self) -> Option<&E> {
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

impl<T, E> From<T> for VirtualNode<E>
where
    T: Display,
    E: Element,
{
    fn from(v: T) -> VirtualNode<E> {
        t(&format!("{}", v))
    }
}

impl<E> VirtualNode<E>
where
    E: Element,
{
    pub fn from_element(e: E) -> VirtualNode<E> {
        VirtualNode::Element(VirtualNodeElement {
            selector: e.get_tag(),
            data: None,
            children: None,
            element: Some(e),
            list_key: None,
        })
    }
}

#[derive(Debug)]
pub struct VirtualNodeElement<E: Element> {
    pub selector: String,
    pub data: Option<Props>,
    pub children: Option<Vec<VirtualNode<E>>>,
    pub element: Option<E>,
    pub list_key: Option<String>,
}

impl<E> PartialEq for VirtualNodeElement<E>
where
    E: Element,
{
    fn eq(&self, other: &VirtualNodeElement<E>) -> bool {
        self.selector == other.selector
            && self.data == other.data
            && self.list_key == other.list_key
            && self.children == other.children
    }
}

#[derive(Debug)]
pub struct VirtualNodeText<E: Element> {
    pub element: Option<E>,
    pub text: String,
}

impl<E> PartialEq for VirtualNodeText<E>
where
    E: Element,
{
    fn eq(&self, other: &VirtualNodeText<E>) -> bool {
        self.text == other.text
    }
}
