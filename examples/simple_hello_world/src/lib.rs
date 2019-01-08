#![feature(proc_macro_hygiene)]
// Basics we need for halcyon
use halcyon::dom::DOM;
use halcyon::{Halcyon, VirtualNode};
// A DOM interface using WebIDL
use halcyon_dom::{WebIDLDOM, WebIDLElement};
// A standard helper macro for creating virtual dom for halcyon
use halcyon_macro::html;
// wasm_bindgen helps us talk to the browser in a standard way
use wasm_bindgen::prelude::*;

// Create a simple functional component
fn hello_world() -> VirtualNode<WebIDLElement> {
    html! {
        <div>{"Hello World!"}</div>
    }
}

// This is where execution will start
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Creates a halcyon instance with default extensions
    // Targeted to use WebIDL to talk with browser page
    let mut halcyon = Halcyon::<WebIDLDOM, WebIDLElement>::new(WebIDLDOM::new());

    // Gets a handle to the body
    let body = halcyon.dom().query_selector("body").unwrap();

    // Renders out the initial component's virtual dom to the body
    halcyon.init_render(body, hello_world());
    Ok(())
}
