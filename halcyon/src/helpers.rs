use crate::dom::Element;
use crate::{Props, VirtualNode, VirtualNodeElement, VirtualNodeText};

pub fn c<E>(
    function: Box<Fn(Option<Props>, Option<Vec<VirtualNode<E>>>) -> VirtualNode<E>>,
    data: Option<Props>,
    children: Option<Vec<VirtualNode<E>>>,
) -> VirtualNode<E>
where
    E: Element,
{
    function(data, children)
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
