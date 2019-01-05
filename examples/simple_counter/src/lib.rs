use halcyon::{Halcyon, VirtualNode,Store,Reducer};
use halcyon_dom::WebIDLDOM;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate halcyon_macro;
use std::rc::Rc;
use std::cell::RefCell;

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

fn get_count() -> i32{
    STORE.with(|s| s.borrow().state().count)
}

fn counter() -> VirtualNode {
    html!{
        <div>{ get_count() }</div>
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
