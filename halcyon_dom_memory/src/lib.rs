use halcyon::{Element, DOM};
use rctree::Node;
use std::collections::HashMap;

pub mod prelude;

#[derive(Debug)]
pub struct NodeData {
    pub tag: String,
    pub inner_text: Option<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub struct MemoryDOM {
    pub root: Node<NodeData>,
}

fn node_to_string(node: &Node<NodeData>) -> String {
    let n = node.borrow();
    match &n.inner_text {
        Some(t) => t.clone(),
        None => {
            let attributes = n
                .attributes
                .iter()
                .map(|(n, v)| format!(r#" {}="{}""#, n, v))
                .collect::<Vec<String>>()
                .join("");
            let children = node
                .children()
                .map(|x| node_to_string(&x))
                .collect::<Vec<String>>()
                .join("");
            format!("<{}{}>{}</{}>", n.tag, attributes, children, n.tag)
        }
    }
}

impl MemoryDOM {
    pub fn new() -> MemoryDOM {
        let mut md = MemoryDOM {
            root: Node::new(NodeData {
                tag: "html".to_string(),
                inner_text: None,
                attributes: HashMap::new(),
            }),
        };
        md.root.append(Node::new(NodeData {
            tag: "body".to_string(),
            inner_text: None,
            attributes: HashMap::new(),
        }));
        md
    }
}

impl DOM<MemoryElement> for MemoryDOM {
    fn query_selector(&self, tag: &str) -> Option<MemoryElement> {
        if tag == "body" {
            return match self.root.first_child() {
                Some(fc) => Some(MemoryElement { node: fc }),
                None => None,
            };
        }
        panic!("not implemented query_selector");
    }

    fn create_text_node(&self, txt: &str) -> MemoryElement {
        MemoryElement {
            node: Node::new(NodeData {
                tag: "!text".to_string(),
                inner_text: Some(txt.to_string()),
                attributes: HashMap::new(),
            }),
        }
    }

    fn create_node(&self, tag: &str) -> MemoryElement {
        MemoryElement {
            node: Node::new(NodeData {
                tag: tag.to_string(),
                inner_text: None,
                attributes: HashMap::new(),
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MemoryElement {
    pub node: Node<NodeData>,
}

impl Element for MemoryElement {
    fn get_tag(&self) -> String {
        self.node.borrow().tag.clone()
    }

    fn get_parent(&self) -> Option<MemoryElement> {
        match self.node.parent() {
            Some(p) => Some(MemoryElement { node: p }),
            None => None,
        }
    }

    fn next_sibling(&self) -> Option<MemoryElement> {
        match self.node.next_sibling() {
            Some(s) => Some(MemoryElement { node: s }),
            None => None,
        }
    }

    fn insert_before(
        &mut self,
        element_to_insert: &MemoryElement,
        target: Option<&mut MemoryElement>,
    ) {
        match target {
            Some(t) => t.node.insert_before(element_to_insert.node.clone()),
            None => self.node.append(element_to_insert.node.clone()),
        }
    }

    fn remove(&mut self) {
        self.node.detach();
    }

    fn append_child(&mut self, element: &MemoryElement) {
        self.node.append(element.node.clone());
    }

    fn set_attribute(&mut self, name: &str, value: &str) {
        self.node
            .borrow_mut()
            .attributes
            .insert(name.to_string(), value.to_string());
    }

    fn to_string(&self) -> String {
        node_to_string(&self.node)
    }
}
