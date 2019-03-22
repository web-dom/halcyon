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
```html
<div id="helloworld"></div>
<script src="http://unpkg.com/web-dom@latest/web-dom.min.js"></script>
<web-dom module="helloworld.wasm"></web-dom>
```
