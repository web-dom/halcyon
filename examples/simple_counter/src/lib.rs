use halcyon::{Attributes, Halcyon, VirtualNode};
use halcyon_dom::WebIDLDOM;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate halcyon_macro;

static count: i32 = 42;

fn counter() -> VirtualNode {
    html! {
        <div>{count}</div>
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    thread_local! {
        static HALCYON:Halcyon = Halcyon::new(WebIDLDOM::new(),vec![Box::new(Attributes::new())]);
    };
    HALCYON.with(|halcyon| {
        let body = halcyon.dom().query_selector("body");
        halcyon.render(body, counter());
    });
    Ok(())
}
