pub use crate::dom::{Element, DOM};
use crate::extensions::Extension;
use std::cell::RefCell;
use std::rc::Rc;
pub mod dom;
pub mod extensions;
mod helpers;
pub mod props;
mod store;
mod vnode;

pub use crate::dom::memory::{MemoryDOM, MemoryElement};
pub use crate::extensions::attributes::Attributes;
pub use crate::helpers::{h, t};
pub use crate::props::{Prop, Props};
pub use crate::store::{Reducer, Store};
pub use crate::vnode::{VirtualNode, VirtualNodeElement, VirtualNodeText};

#[derive(Debug)]
pub struct Halcyon {
    api: Box<DOM>,
    current_vnode: VirtualNode,
    extensions: Vec<Box<Extension>>,
}

impl Halcyon {
    pub fn setup<T: Clone + Reducer<P>, P, Q: 'static + Fn() -> VirtualNode>(
        halcyon: &'static std::thread::LocalKey<RefCell<Halcyon>>,
        store: &'static std::thread::LocalKey<RefCell<Store<T, P>>>,
        target: &str,
        render: Q,
    ) {
        let node_renderer = Rc::new(render);
        let halcyon_extra_key = halcyon.clone();
        halcyon.with(|h| {
            let mut h_mut = h.borrow_mut();
            let t = h_mut.dom().query_selector(target);
            // Get the body as our target element
            // Do initial render to element
            h_mut.init_render(t, node_renderer());
        });
        let render_ref = node_renderer.clone();
        store.with(|s| {
            // Add a listener to listen for state changes
            s.borrow().add_listener(Box::new(move || {
                halcyon_extra_key.with(|h| {
                    let mut h_mut = h.borrow_mut();
                    // Rerender everything again with new virtual dom
                    h_mut.render(render_ref());
                })
            }));
        });
    }

    pub fn root(&self) -> &VirtualNode {
        return &self.current_vnode
    }

    pub fn new(api: Box<DOM>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: VirtualNode::Empty,
            extensions: vec![Box::new(Attributes::new())],
        }
    }

    pub fn custom(api: Box<DOM>, extensions: Vec<Box<Extension>>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: VirtualNode::Empty,
            extensions: extensions,
        }
    }

    pub fn dom(&self) -> &'_ Box<DOM> {
        return &self.api;
    }

    pub fn patch(&mut self, new_vnode: VirtualNode) {
        if let VirtualNode::Empty = self.current_vnode {
            self.current_vnode = new_vnode;
            return;
        }

        for e in self.extensions.iter() {
            e.pre();
        }
        self.current_vnode = new_vnode;
        for e in self.extensions.iter() {
            e.post();
        }
    }

    pub fn init_render(&mut self, element: Box<Element>, container: VirtualNode) {
        self.patch(VirtualNode::from_element(element));
        self.patch(container);
    }

    pub fn render(&mut self, container: VirtualNode) {
        self.patch(container);
    }
}
