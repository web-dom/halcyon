#![feature(proc_macro_hygiene)]

#[cfg(test)]
mod tests {
    use halcyon::VirtualNode;
    use halcyon_dom_memory::MemoryElement;
    use halcyon_macro::html;

    #[test]
    fn basic_element() {
        let expected: VirtualNode<MemoryElement> = halcyon::h("div", None, None);
        let result = html! {
            <div></div>
        };
        assert_eq!(expected, result, "basic element did not match");
    }

    #[test]
    fn small_element() {
        let expected: VirtualNode<MemoryElement> = halcyon::h("div", None, None);
        let result = html! {
            <div/>
        };
        assert_eq!(expected, result, "basic element did not match");
    }

    #[test]
    fn simple_child() {
        let expected: VirtualNode<MemoryElement> =
            halcyon::h("div", None, Some(vec![halcyon::h("div", None, None)]));
        let result = html! {
            <div><div/></div>
        };
        assert_eq!(expected, result, "simple child did not match");
    }

    #[test]
    fn simple_code_child() {
        let expected: VirtualNode<MemoryElement> =
            halcyon::h("div", None, Some(vec![halcyon::t("hello world")]));
        let result = html! {
            <div>{"hello world"}</div>
        };
        assert_eq!(expected, result, "simple code child did not match");
    }

    #[test]
    fn basic_text_attribute() {
        let expected: VirtualNode<MemoryElement> = halcyon::h(
            "div",
            Some({
                let mut h = halcyon::Props::new();
                h.insert("a".to_string(), halcyon::Prop::from("123".to_string()));
                h
            }),
            None,
        );
        let result = html! {
            <div a="123"></div>
        };
        assert_eq!(expected, result, "basic attribut did not match");
    }

    #[test]
    fn multiple_text_attribute() {
        let expected: VirtualNode<MemoryElement> = halcyon::h(
            "div",
            Some({
                let mut h = halcyon::Props::new();
                h.insert("a".to_string(), halcyon::Prop::from("123".to_string()));
                h.insert("b".to_string(), halcyon::Prop::from("abc".to_string()));
                h
            }),
            None,
        );
        let result = html! {
            <div a="123" b="abc"></div>
        };
        assert_eq!(expected, result, "basic attribute did not match");
    }

    #[test]
    fn basic_code_attribute() {
        let expected: VirtualNode<MemoryElement> = halcyon::h(
            "div",
            Some({
                let mut h = halcyon::Props::new();
                h.insert("a".to_string(), halcyon::Prop::from(123i32));
                h
            }),
            None,
        );
        let result = html! {
            <div a={123i32}></div>
        };
        assert_eq!(expected, result, "basic code attribute did not match");
    }

    #[test]
    fn basic_string_attribute() {
        let expected: VirtualNode<MemoryElement> = halcyon::h(
            "div",
            Some({
                let mut h = halcyon::Props::new();
                h.insert("a".to_string(), halcyon::Prop::from("hello"));
                h
            }),
            None,
        );
        let result = html! {
            <div a={"hello"}></div>
        };
        assert_eq!(expected, result, "basic code attribute did not match");
    }
}
