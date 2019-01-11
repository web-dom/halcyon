#![feature(proc_macro_hygiene)]
use halcyon::prelude::*;
use halcyon_dom_memory::prelude::*;
use halcyon_macro::html;

// Create a simple functional component
fn hello_world(_: Props, _: Vec<VirtualNode<MemoryElement>>) -> VirtualNode<MemoryElement> {
    html! {
        <div id="helloworld"><h1>{"Hello World!"}</h1></div>
    }
}

fn main() {
    let mut halcyon = Halcyon::<MemoryDOM, MemoryElement>::new(MemoryDOM::new());
    let body = halcyon
        .dom()
        .query_selector("body")
        .expect("body should exist");

    // Renders out the initial component's virtual dom to the body
    halcyon.init_render(body, html! {<HelloWorld/>});
    println!("{}", halcyon.render_to_string());
}
