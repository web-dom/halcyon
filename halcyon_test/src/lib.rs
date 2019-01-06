#![feature(proc_macro_hygiene)]
#[cfg(test)]
mod tests {
    use halcyon::{Halcyon,MemoryDOM,MemoryElement};
    use halcyon_macro::html;

    #[test]
    fn simplest() {
        let halcyon = Halcyon::new(MemoryDOM::new());
        let body = MemoryElement::new("body");
        halcyon.init_render(body,html!(<div></div>))
    }
}
