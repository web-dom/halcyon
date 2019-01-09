use halcyon::{Element, DOM};
use rctree::Node;

pub mod prelude;

#[derive(Debug)]
pub struct NodeData {
    pub tag: String,
    pub inner_text: Option<String>,
}

#[derive(Debug)]
pub struct MemoryDOM {
    pub root: Node<NodeData>,
}

impl MemoryDOM {
    pub fn new() -> MemoryDOM {
        let mut md = MemoryDOM {
            root: Node::new(NodeData {
                tag: "html".to_string(),
                inner_text: None,
            }),
        };
        md.root.append(Node::new(NodeData {
            tag: "body".to_string(),
            inner_text: None,
        }));
        md
    }
}

impl DOM<MemoryElement> for MemoryDOM {
    fn query_selector(&self, selector: &str) -> Option<MemoryElement> {
        if selector == "body" {
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
            }),
        }
    }

    fn create_node(&self, tag: &str) -> MemoryElement {
        MemoryElement {
            node: Node::new(NodeData {
                tag: tag.to_string(),
                inner_text: None,
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
}
