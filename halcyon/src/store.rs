use std::cell::Ref;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::marker::Sized;
use std::rc::Rc;
use std::thread::LocalKey;

pub trait Reducer<P> {
    fn reduce(&self, action: P) -> Option<Self>
    where
        Self: Sized;
}

pub struct Store<T, P>
where
    T: Clone,
{
    state: Rc<RefCell<T>>,
    listeners: Rc<RefCell<Vec<Box<Fn()>>>>,
    _p: PhantomData<P>,
}

impl<T: Reducer<P>, P> Store<T, P>
where
    T: Clone,
{
    pub fn new(initial_value: T) -> Store<T, P> {
        Store {
            state: Rc::new(RefCell::new(initial_value)),
            _p: PhantomData,
            listeners: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn connect<V, Q: Fn(T, Rc<Fn(P)>) -> V>(
        store_thread_key: &'static LocalKey<RefCell<Store<T, P>>>,
        handler: Q,
    ) -> V {
        store_thread_key.with(|store| {
            handler(
                store.borrow().state.borrow().clone(),
                Rc::new(move |p| store_thread_key.with(|store| store.borrow().dispatch(p))),
            )
        })
    }

    pub fn dispatch(&self, action: P) {
        let t = self.state.borrow_mut().reduce(action);
        if let Some(new_state) = t {
            *self.state.borrow_mut() = new_state;
            for listener in self.listeners.borrow().iter() {
                listener();
            }
        }
    }

    pub fn add_listener(&self, listener: Box<Fn()>) {
        self.listeners.borrow_mut().push(listener)
    }

    pub fn state(&self) -> Ref<T> {
        self.state.borrow()
    }
}
