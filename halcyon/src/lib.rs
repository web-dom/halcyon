pub use crate::dom::{Element, DOM};
use crate::extensions::Extension;
use std::cell::RefCell;
use std::rc::Rc;
pub mod dom;
pub mod extensions;
pub mod props;
mod helpers;
mod store;
mod vnode;

pub use crate::dom::memory::{MemoryDOM,MemoryElement};
pub use crate::extensions::attributes::Attributes;
pub use crate::props::{Prop,Props};
pub use crate::store::{Reducer,Store};
pub use crate::helpers::{h,t};
pub use crate::vnode::{VirtualNode,VirtualNodeText,VirtualNodeElement};

#[derive(Debug)]
pub struct Halcyon {
    api: Box<DOM>,
    current_vnode: RefCell<Option<VirtualNode>>,
    extensions: Vec<Box<Extension>>,
}

impl Halcyon {
    pub fn setup<T: Clone + Reducer<P>, P, Q: 'static + Fn() -> VirtualNode>(
        halcyon: &'static std::thread::LocalKey<Halcyon>,
        store: &'static std::thread::LocalKey<RefCell<Store<T, P>>>,
        target: &str,
        render: Q,
    ) {
        let node_renderer = Rc::new(render);
        let halcyon_extra_key = halcyon.clone();
        halcyon.with(|h| {
            let t = h.dom().query_selector(target);
            // Get the body as our target element
            // Do initial render to element
            h.init_render(t, node_renderer());
        });
        let render_ref = node_renderer.clone();
        store.with(|s| {
            // Add a listener to listen for state changes
            s.borrow().add_listener(Box::new(move || {
                halcyon_extra_key.with(|h| {
                    // Rerender everything again with new virtual dom
                    h.render(render_ref());
                })
            }));
        });
    }

    pub fn new(api: Box<DOM>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: RefCell::new(None),
            extensions: vec![Box::new(Attributes::new())],
        }
    }

    pub fn custom(api: Box<DOM>, extensions: Vec<Box<Extension>>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: RefCell::new(None),
            extensions: extensions,
        }
    }

    pub fn dom(&self) -> &'_ Box<DOM> {
        return &self.api;
    }

    pub fn has_patched(&self) -> bool {
        let c = self.current_vnode.borrow();
        if let None = *c {
            return false;
        }
        return true;
    }

    pub fn patch(&self, new_vnode: VirtualNode) {
        let mut c = self.current_vnode.borrow_mut();
        if let None = *c {
            *c = Some(new_vnode);
            return;
        }

        for e in self.extensions.iter() {
            e.pre();
        }
        *c = Some(new_vnode);
        for e in self.extensions.iter() {
            e.post();
        }
    }

    pub fn init_render(&self, element: Rc<RefCell<Element>>, container: VirtualNode) {
        self.patch(VirtualNode::from_element(element));
        self.patch(container);
    }

    pub fn render(&self, container: VirtualNode) {
        self.patch(container);
    }
}
