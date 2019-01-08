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
    }
}
