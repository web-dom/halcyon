#![feature(proc_macro_hygiene)]
#[cfg(test)]
mod tests {
    use halcyon::DOM;
    use halcyon::{Halcyon, VirtualNode};
    use halcyon_dom_memory::{MemoryDOM, MemoryElement};
    use halcyon_macro::html;

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

    #[test]
    fn simple_nesting() {
        let mut halcyon = Halcyon::<MemoryDOM, MemoryElement>::new(MemoryDOM::new());
        let body = halcyon
            .dom()
            .query_selector("body")
            .expect("there should be a body");
        let c = html!(<div><h1>{"hello world!"}</h1></div>);
        halcyon.init_render(body, c);
        let root = halcyon.root().expect("there should be a root element");
        println!("{:?}", root.get_element().unwrap().node);
        match &root {
            VirtualNode::Element(r) => {
                assert_eq!("div", r.selector, "selector should be div: {:?}", r);
            }
            _ => panic!("should not be none"),
        };
    }
}
