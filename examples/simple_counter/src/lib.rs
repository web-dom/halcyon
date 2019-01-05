use halcyon::{Halcyon, VirtualNode};
use halcyon_dom::WebIDLDOM;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate halcyon_macro;

static COUNT: i32 = 42;

fn counter() -> VirtualNode {
    html! {
        <div>{COUNT}</div>
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    thread_local! {
        static HALCYON:Halcyon = Halcyon::new(WebIDLDOM::new());
    };
    HALCYON.with(|halcyon| {
        let body = halcyon.dom().query_selector("body");
        halcyon.render(body, counter());
    });
    Ok(())
}
