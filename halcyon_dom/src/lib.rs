use halcyon::{Element, DOM};

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
    el: web_sys::Node,
}

impl Element for WebIDLElement {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }

    fn get_parent(&self) -> Option<WebIDLElement> {
        let n = self.el.parent_element().expect("should have parent");
        Some(WebIDLElement {
            tag: n.tag_name(),
            el: n.into(),
        })
    }

    fn next_sibling(&self) -> Option<WebIDLElement> {
        let s = self.el.next_sibling();
        match s {
            Some(n) => Some(WebIDLElement {
                tag: "not-sure-what-next-sibling-should-be".to_string(),
                el: n.into(),
            }),
            None => None,
        }
    }

    fn insert_before(&mut self, element: &WebIDLElement, target: Option<&mut WebIDLElement>) {
        match target {
            Some(n) => self.el.insert_before(&element.el, Some(&n.el)),
            None => self.el.insert_before(&element.el, None),
        }
        .expect("should have inserted");
    }

    fn remove(&mut self) {
        self.el
            .parent_node()
            .expect("should have parent")
            .remove_child(&self.el)
            .expect("should have removed");
    }

    fn append_child(&mut self, element: &WebIDLElement) {
        self.el
            .append_child(&element.el)
            .expect("should have appended");
    }
}

impl DOM<WebIDLElement> for WebIDLDOM {
    fn query_selector(&self, selector: &str) -> Option<WebIDLElement> {
        let el = self
            .document
            .query_selector(selector)
            .expect("could not query selected element")
            .expect("did not find selected element");
        Some(WebIDLElement {
            tag: el.tag_name(),
            el: el.into(),
        })
    }

    fn create_text_node(&self, txt: &str) -> WebIDLElement {
        WebIDLElement {
            tag: "!text".to_string(),
            el: self.document.create_text_node(txt).into(),
        }
    }

    fn create_node(&self, tag: &str) -> WebIDLElement {
        WebIDLElement {
            tag: tag.to_string(),
            el: self
                .document
                .create_element(tag)
                .expect("should be able to create element")
                .into(),
        }
    }
}

impl PartialEq for WebIDLElement {
    fn eq(&self, _other: &WebIDLElement) -> bool {
        true
    }
}
