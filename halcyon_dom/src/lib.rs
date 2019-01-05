use halcyon::{Element, DOM};
use std::cell::RefCell;
use std::rc::Rc;

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
        self.el.tag_name()
    }
}

impl DOM for WebIDLDOM {
    fn query_selector(&self, selector: &str) -> Rc<RefCell<Element>> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        Rc::new(RefCell::new(WebIDLElement {
            el: document
                .query_selector(selector)
                .expect("could not query selected element")
                .expect("did not find selected element"),
        }))
    }
}
