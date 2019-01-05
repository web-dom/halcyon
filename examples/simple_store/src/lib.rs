#[allow(unused_imports)]
use halcyon::{Reducer, Store};
#[allow(unused_imports)]
use std::cell::RefCell;
use std::rc::Rc;

// This is our store that will hold a simple value
pub struct Counter {
    pub count: i32,
}

impl Counter {
    // Creates a store with default values
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
    // A Reducer processes an action and returns a new state
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // lets create a store
        let store = Store::new(Rc::new(Counter::new()));
        // increment and validate
        store.dispatch(Actions::Increment);
        assert_eq!(store.state().count, 1, "count should be 1");
        // decrement and validate
        store.dispatch(Actions::Decrement);
        assert_eq!(store.state().count, 0, "count should be 0");
    }
}
