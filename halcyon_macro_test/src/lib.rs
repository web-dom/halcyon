#[allow(unused_imports)]
extern crate halcyon;
#[macro_use]
#[allow(unused_imports)]
extern crate halcyon_macro;

#[cfg(test)]
mod tests {
    #[test]
    fn basic_element() {
        let expected = halcyon::h("div", None, None);
        let result = html! {
            <div></div>
        };
        assert_eq!(expected, result, "basic element did not match");
    }

    #[test]
    fn small_element() {
        let expected = halcyon::h("div", None, None);
        let result = html! {
            <div/>
        };
        assert_eq!(expected, result, "basic element did not match");
    }

    #[test]
    fn simple_child() {
        let expected = halcyon::h("div", None, Some(vec![halcyon::h("div", None, None)]));
        let result = html! {
            <div><div/></div>
        };
        assert_eq!(expected, result, "basic element did not match");
    }

    #[test]
    fn basic_text_attribute() {
        let expected = halcyon::h(
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
        let expected = halcyon::h(
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
        let expected = halcyon::h(
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
}
