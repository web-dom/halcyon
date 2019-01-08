use halcyon::{Element, DOM};

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebIDLDOM {}

impl WebIDLDOM {
    pub fn new() -> Box<DOM> {
        Box::new(WebIDLDOM {})
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

    fn get_parent(&self) -> Option<Box<Element>> {
        panic!("todo")
    }
}

impl DOM for WebIDLDOM {
    fn query_selector(&self, selector: &str) -> Option<Box<Element>> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        Some(Box::new(WebIDLElement {
            el: document
                .query_selector(selector)
                .expect("could not query selected element")
                .expect("did not find selected element"),
        }))
    }

    fn create_text_node(&self, _txt: &str) -> Box<Element> {
        panic!("not implemented");
    }
}
