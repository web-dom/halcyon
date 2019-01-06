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
        let root = halcyon.root();
        match &root {
            VirtualNode::Element(r) => {
                match &r.element {
                    Some(r) => assert_eq!("div",r.get_tag(),"tag should be div"),
                    _ => panic!("should not be none"),
                }
            }
            _ => panic!("should not be none")
        }
    }
}
