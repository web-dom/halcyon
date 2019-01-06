#![feature(proc_macro_hygiene)]
#[cfg(test)]
mod tests {
    use halcyon::{Halcyon, MemoryDOM, MemoryElement,VirtualNode};
    use halcyon_macro::html;

    #[test]
    fn simplest() {
        let mut halcyon = Halcyon::new(MemoryDOM::new());
        let body = MemoryElement::new("body");
        halcyon.init_render(body, html!(<div></div>));
        let root = halcyon.root().expect("there should be a root element");
        match &root {
            VirtualNode::Element(r) => {
                assert_eq!("div",r.selector,"selector should be div: {:?}", r);
            }
            _ => panic!("should not be none")
        };
    }
}
