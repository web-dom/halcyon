#![feature(proc_macro_hygiene)]
use halcyon::{Halcyon, Reducer, Store, VirtualNode};
use halcyon_dom::WebIDLDOM;
use halcyon_macro::html;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

enum Actions {
    Increment,
    Decrement,
}

impl Reducer<Actions> for Rc<Counter> {
    fn reduce(&self, a: Actions) -> Option<Rc<Counter>> {
        // Reducers only return an Option if state changed
        match a {
            Actions::Increment => Some(Rc::new(Counter {
                count: self.count + 1,
            })),
            Actions::Decrement => Some(Rc::new(Counter {
                count: self.count - 1,
            })),
        }
    }
}

thread_local! { static STORE : RefCell<Store<Rc<Counter>, Actions>> = RefCell::new(Store::new(Rc::new(Counter{count:0}))); }

fn counter(c: i32) -> VirtualNode {
    html! {
        <div>{c}</div>
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    thread_local! {
        static HALCYON:Halcyon = Halcyon::new(WebIDLDOM::new());
    };
    HALCYON.with(|halcyon| {
        let body = halcyon.dom().query_selector("body");
        halcyon.render(body, counter(42));
    });
    Ok(())
}
