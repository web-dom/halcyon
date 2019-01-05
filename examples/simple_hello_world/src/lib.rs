#![feature(proc_macro_hygiene)]
use wasm_bindgen::prelude::*;
use halcyon::{Halcyon, VirtualNode};
use halcyon_dom::WebIDLDOM;
use halcyon_macro::html;

fn hello_world() -> VirtualNode {
    html!{
        <div>{"Hello World!"}</div>
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    thread_local! {
        static HALCYON:Halcyon = Halcyon::new(WebIDLDOM::new());
    };
    HALCYON.with(|halcyon| {
        let body = halcyon.dom().query_selector("body");
        halcyon.render(body, hello_world());
    });
    Ok(())
}
