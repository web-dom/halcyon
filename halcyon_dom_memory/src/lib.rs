use halcyon::{Element, DOM};
use rctree::Node;

#[derive(Debug)]
struct NodeData {
    tag: String,
    inner_text: Option<String>,
}

#[derive(Debug)]
pub struct MemoryDOM {
    root: Node<NodeData>,
}

impl MemoryDOM {
    pub fn new() -> Box<DOM> {
        let mut md = Box::new(MemoryDOM {
            root: Node::new(NodeData {
                tag: "html".to_string(),
                inner_text: None,
            }),
        });
        md.root.append(Node::new(NodeData {
            tag: "body".to_string(),
            inner_text: None,
        }));
        md
    }
}

impl DOM for MemoryDOM {
    fn query_selector(&self, selector: &str) -> Option<Box<Element>> {
        if selector == "body" {
            return match self.root.first_child() {
                Some(fc) => Some(Box::new(MemoryElement { node: fc })),
                None => None,
            };
        }
        panic!("not implemented query_selector");
    }

    fn create_text_node(&self, txt: &str) -> Box<Element> {
        Box::new(MemoryElement {
            node: Node::new(NodeData {
                tag: "body".to_string(),
                inner_text: Some(txt.to_string()),
            }),
        })
    }
}

#[derive(Debug)]
pub struct MemoryElement {
    node: Node<NodeData>,
}

impl Element for MemoryElement {
    fn get_tag(&self) -> String {
        self.node.borrow().tag.clone()
    }

    fn get_parent(&self) -> Option<Box<Element>> {
        match self.node.parent() {
            Some(p) => Some(Box::new(MemoryElement { node: p })),
            None => None,
        }
    }
}
