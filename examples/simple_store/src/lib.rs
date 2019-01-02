#[allow(unused_imports)]
use halcyon::{Reducer, Store};
#[allow(unused_imports)]
use std::cell::RefCell;
use std::rc::Rc;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Create a store and put our initial state in it
        thread_local!(static STORE : RefCell<Store<Rc<Counter>, Actions>> = RefCell::new(Store::new(Rc::new(Counter::new()))));
        STORE.with(|store| {
            // get a reference of the static store
            let store_ref = store.borrow();
            // dispatch your action to increment
            store.borrow().dispatch(Actions::Increment);
            // be careful about borrowing the state and then dispatching (which will borrow the state to modify and cause error)
            assert!(store_ref.state.borrow().count == 1, "count should be 1");
            // dispatch action to decrement
            store_ref.dispatch(Actions::Decrement);
            assert!(store_ref.state.borrow().count == 0, "count should be 0");
        })
    }
}
