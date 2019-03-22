# halcyon
A non-monothlithic modular virtual DOM 

```rust
fn hello_world(_: Props, _: Vec<VNode>) -> VNode {
    html! {
        <div id="helloworld">{"Hello World!"}</div>
    }
}

#[start]
pub fn main() -> () {
    let mut halcyon = Halcyon::<WebDOM, WebElement>::new(WebDOM::new());
    let root = halcyon.dom().query_selector("#helloworld").unwrap();
    halcyon.render(root, html! {<HelloWorld/>});
}
```
