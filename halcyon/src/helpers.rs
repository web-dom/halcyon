use crate::{Props, VirtualNode, VirtualNodeElement, VirtualNodeText};

pub fn h(selector: &str, data: Option<Props>, children: Option<Vec<VirtualNode>>) -> VirtualNode {
    VirtualNode::Element(VirtualNodeElement {
        selector: String::from(selector),
        data: data,
        children: children,
        element: None,
        list_key: None,
    })
}

pub fn t(text: &str) -> VirtualNode {
    VirtualNode::Text(VirtualNodeText {
        element: None,
        text: String::from(text),
    })
}
