#![feature(proc_macro_hygiene)]
use halcyon::prelude::*;
use halcyon_dom::prelude::*;
use halcyon_macro::html;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

// Our counter store
pub struct Counter {
    pub count: i32,
}

// Create a counter store with default values
impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Our actions the store can handle
enum Actions {
    Increment,
    Decrement,
}

impl Reducer<Actions> for Rc<Counter> {
    fn reduce(&self, a: Actions) -> Option<Rc<Counter>> {
        // Reducers take actions and return an Option::Some if there's a new state
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

// Our static store
thread_local! { static STORE : RefCell<Store<Rc<Counter>, Actions>> = RefCell::new(Store::new(Rc::new(Counter{count:0}))); }

// Our counter component
fn counter(_: Props, _: Vec<VirtualNode<WebIDLElement>>) -> VirtualNode<WebIDLElement> {
    Store::connect(&STORE, |state, dispatch| {
        let dispatcher_increment = dispatch.clone();
        let dispatcher_decrement = dispatch.clone();
        html! {
            <div>
                {state.count}
                <div class="counter-button" onclick={move||{
                    dispatcher_increment(Actions::Increment);
                }}>
                    {"+"}
                </div>
                <div class="counter-button" onclick={move||{
                    dispatcher_decrement(Actions::Decrement);
                }}>
                    {"-"}
                </div>
            </div>
        }
    })
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Creates a halcyon instance targeting WebIDL
    thread_local! {
        static HALCYON:RefCell<Halcyon<WebIDLDOM,WebIDLElement>> = RefCell::new(Halcyon::<WebIDLDOM,WebIDLElement>::new(WebIDLDOM::new()));
    };
    // Setup Halcyon:
    // 1. runs initial render to target Element
    // 2. listening to the store for new state and rerenders
    // This uses a closure that calls the component's rendering function
    Halcyon::setup(&HALCYON, &STORE, "body", || html! {<Counter/>});
    Ok(())
}
