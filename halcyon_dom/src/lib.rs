use halcyon::{Element, DOM};

pub mod prelude;

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebIDLDOM {
    document: web_sys::Document,
}

impl WebIDLDOM {
    pub fn new() -> WebIDLDOM {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        WebIDLDOM { document: document }
    }
}

#[derive(Debug)]
pub struct WebIDLElement {
    tag: String,
    el: NodeHandle,
}

#[derive(Debug)]
enum NodeHandle {
    Element(web_sys::Element),
    Text(web_sys::Text),
    Unknown(web_sys::Node),
}

impl NodeHandle {
    fn as_node(&self) -> &web_sys::Node {
        match self {
            NodeHandle::Element(n) => &n,
            NodeHandle::Text(n) => &n,
            NodeHandle::Unknown(n) => &n,
        }
    }
}

impl Element for WebIDLElement {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }

    fn get_parent(&self) -> Option<WebIDLElement> {
        let n = self
            .el
            .as_node()
            .parent_element()
            .expect("should have parent");
        Some(WebIDLElement {
            tag: n.tag_name(),
            el: NodeHandle::Element(n),
        })
    }

    fn next_sibling(&self) -> Option<WebIDLElement> {
        let s = self.el.as_node().next_sibling();
        match s {
            Some(n) => Some(WebIDLElement {
                tag: "not-sure-what-next-sibling-should-be".to_string(),
                el: NodeHandle::Unknown(n),
            }),
            None => None,
        }
    }

    fn insert_before(&mut self, element: &WebIDLElement, target: Option<&mut WebIDLElement>) {
        match target {
            Some(n) => self
                .el
                .as_node()
                .insert_before(element.el.as_node(), Some(n.el.as_node())),
            None => self.el.as_node().insert_before(element.el.as_node(), None),
        }
        .expect("should have inserted");
    }

    fn remove(&mut self) {
        self.el
            .as_node()
            .parent_node()
            .expect("should have parent")
            .remove_child(self.el.as_node())
            .expect("should have removed");
    }

    fn append_child(&mut self, element: &WebIDLElement) {
        self.el
            .as_node()
            .append_child(element.el.as_node())
            .expect("should have appended");
    }

    fn set_attribute(&mut self, name: &str, value: &str) {
        match &mut self.el {
            NodeHandle::Element(el) => {
                el.set_attribute(name, value)
                    .expect("could not set attribute");
            }
            NodeHandle::Text(_) => (),
            NodeHandle::Unknown(_) => (),
        };
    }
}

impl DOM<WebIDLElement> for WebIDLDOM {
    fn query_selector(&self, tag: &str) -> Option<WebIDLElement> {
        let el = self
            .document
            .query_selector(tag)
            .expect("could not query selected element")
            .expect("did not find selected element");
        Some(WebIDLElement {
            tag: el.tag_name(),
            el: NodeHandle::Element(el),
        })
    }

    fn create_text_node(&self, txt: &str) -> WebIDLElement {
        WebIDLElement {
            tag: "!text".to_string(),
            el: NodeHandle::Text(self.document.create_text_node(txt)),
        }
    }

    fn create_node(&self, tag: &str) -> WebIDLElement {
        WebIDLElement {
            tag: tag.to_string(),
            el: NodeHandle::Element(
                self.document
                    .create_element(tag)
                    .expect("should be able to create element"),
            ),
        }
    }
}

impl PartialEq for WebIDLElement {
    fn eq(&self, _other: &WebIDLElement) -> bool {
        true
    }
}
