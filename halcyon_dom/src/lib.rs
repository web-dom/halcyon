use halcyon::{Element, DOM};

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebIDLDOM {}

impl WebIDLDOM {
    pub fn new() -> WebIDLDOM {
        WebIDLDOM {}
    }
}

#[derive(Debug)]
pub struct WebIDLElement {
    el: web_sys::Element,
}

impl Element for WebIDLElement {
    fn get_tag(&self) -> String {
        self.el.tag_name().clone()
    }

    fn get_parent(&self) -> Option<WebIDLElement> {
        panic!("todo")
    }

    fn next_sibling(&self) -> Option<WebIDLElement> {
        panic!("not implemented")
    }

    fn insert_before(
        &mut self,
        _element_to_insert: &WebIDLElement,
        _target: Option<&mut WebIDLElement>,
    ) {
        panic!("not implemented")
    }

    fn remove(&mut self) {
        panic!("not implemented");
    }
}

impl DOM<WebIDLElement> for WebIDLDOM {
    fn query_selector(&self, selector: &str) -> Option<WebIDLElement> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        Some(WebIDLElement {
            el: document
                .query_selector(selector)
                .expect("could not query selected element")
                .expect("did not find selected element"),
        })
    }

    fn create_text_node(&self, _txt: &str) -> WebIDLElement {
        panic!("not implemented");
    }

    fn create_node(&self, _tag: &str) -> WebIDLElement {
        panic!("not implemented");
    }
}

impl PartialEq for WebIDLElement {
    fn eq(&self, _other: &WebIDLElement) -> bool {
        true
    }
}
