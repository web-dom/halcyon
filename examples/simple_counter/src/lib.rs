use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = Element::from(document.body().unwrap());
    body.set_inner_html("blah");
    Ok(())
}
