use crate::dom::Element;
use crate::{Props, VirtualNode, VirtualNodeElement, VirtualNodeText};

pub enum C<E> where E:Element{
    Fn0(Box<Fn()-> VirtualNode<E>>),
    Fn1(Box<Fn(Option<Props>)-> VirtualNode<E>>),
    Fn2(Box<Fn(Option<Props>,Option<Vec<VirtualNode<E>>>)-> VirtualNode<E>>)
}

impl<E,F: 'static> From<F> for C<E> where E:Element,F:Fn()->VirtualNode<E>{
    fn from(f:F) -> C<E>{
        C::Fn0(Box::new(f))
    }
}

pub fn c<E>(
    function: C<E>,
    data: Option<Props>,
    children: Option<Vec<VirtualNode<E>>>,
) -> VirtualNode<E>
where
    E: Element,
{
    match function {
        C::Fn0(f) => f(),
        C::Fn1(f) => f(data),
        C::Fn2(f) => f(data,children)
    }
}

pub fn h<E>(
    selector: &str,
    data: Option<Props>,
    children: Option<Vec<VirtualNode<E>>>,
) -> VirtualNode<E>
where
    E: Element,
{
    VirtualNode::Element(VirtualNodeElement {
        selector: String::from(selector),
        data: data,
        children: children,
        element: None,
        list_key: None,
    })
}

pub fn t<E>(text: &str) -> VirtualNode<E>
where
    E: Element,
{
    VirtualNode::Text(VirtualNodeText {
        element: None,
        text: String::from(text),
    })
}
