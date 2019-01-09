use crate::dom::Element;
use crate::{Props, VirtualNode, VirtualNodeElement, VirtualNodeText};

pub fn c<E>(
    function: Box<Fn(Props, Vec<VirtualNode<E>>) -> VirtualNode<E>>,
    data: Option<Props>,
    children: Option<Vec<VirtualNode<E>>>,
) -> VirtualNode<E>
where
    E: Element,
{
    let p = match data {
        Some(p) => p,
        None => Props::new()
    };
    let c = match children {
        Some(c) => c,
        None => vec![]
    };
    function(p, c)
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
