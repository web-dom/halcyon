#![feature(proc_macro_hygiene)]
#[cfg(test)]
mod tests {
    use halcyon::DOM;
    use halcyon::{Halcyon, VirtualNode};
    use halcyon_dom_memory::{MemoryDOM, MemoryElement};
    use halcyon_macro::html;

    #[test]
    fn simplest() {
        let mut halcyon = Halcyon::<MemoryDOM, MemoryElement>::new(MemoryDOM::new());
        let body = halcyon
            .dom()
            .query_selector("body")
            .expect("there should be a body");
        halcyon.init_render(body, html!(<div></div>));
        let root = halcyon.root().expect("there should be a root element");
        match &root {
            VirtualNode::Element(r) => {
                assert_eq!("div", r.selector, "selector should be div: {:?}", r);
            }
            _ => panic!("should not be none"),
        };
        let html_element = &halcyon.dom().root;
        assert_eq!("html", html_element.borrow().tag);
        assert_eq!("div", html_element.first_child().unwrap().borrow().tag);
        assert_eq!(
            true,
            html_element.first_child().unwrap().first_child().is_none()
        );
    }

    #[test]
    fn simple_nesting() {
        let mut halcyon = Halcyon::<MemoryDOM, MemoryElement>::new(MemoryDOM::new());
        let body = halcyon
            .dom()
            .query_selector("body")
            .expect("there should be a body");
        halcyon.init_render(body, html!(<div><h1>{"hello world!"}</h1></div>));
        let root = halcyon.root().expect("there should be a root element");
        match &root {
            VirtualNode::Element(r) => {
                assert_eq!("div", r.selector, "selector should be div: {:?}", r);
            }
            _ => panic!("should not be none"),
        };
        let html_element = &halcyon.dom().root;
        assert_eq!("html", html_element.borrow().tag);
        assert_eq!("div", html_element.first_child().unwrap().borrow().tag);
        assert_eq!(
            "h1",
            html_element
                .first_child()
                .unwrap()
                .first_child()
                .unwrap()
                .borrow()
                .tag
        );
        assert_eq!(
            "!text",
            html_element
                .first_child()
                .unwrap()
                .first_child()
                .unwrap()
                .first_child()
                .unwrap()
                .borrow()
                .tag
        );
        assert_eq!(
            "hello world!",
            html_element
                .first_child()
                .unwrap()
                .first_child()
                .unwrap()
                .first_child()
                .unwrap()
                .borrow()
                .inner_text
                .as_ref()
                .unwrap()
        );
    }
}
