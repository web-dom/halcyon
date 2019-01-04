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
        assert!(expected == result, "basic element did not match");
    }
}
