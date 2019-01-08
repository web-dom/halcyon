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

pub use crate::extensions::attributes::Attributes;
pub use crate::helpers::{h, t};
pub use crate::props::{Prop, Props};
pub use crate::store::{Reducer, Store};
pub use crate::vnode::{VirtualNode, VirtualNodeElement, VirtualNodeText};

#[derive(Debug)]
pub struct Halcyon {
    api: Box<DOM>,
    current_vnode: Option<VirtualNode>,
    extensions: Vec<Box<Extension>>,
    inserted_vnodes: Vec<VirtualNode>,
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
            let t = h_mut
                .dom()
                .query_selector(target)
                .expect("could not find target DOM element");
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

    pub fn root(&self) -> Option<&VirtualNode> {
        match self.current_vnode.as_ref() {
            Some(s) => Some(s),
            None => None,
        }
    }

    pub fn new(api: Box<DOM>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: None,
            extensions: vec![Box::new(Attributes::new())],
            inserted_vnodes: vec![],
        }
    }

    pub fn custom(api: Box<DOM>, extensions: Vec<Box<Extension>>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: None,
            extensions: extensions,
            inserted_vnodes: vec![],
        }
    }

    pub fn dom(&self) -> &'_ Box<DOM> {
        return &self.api;
    }

    pub fn create_element(&self, vnode: &mut VirtualNode) {
        // if its a normal element
        if let VirtualNode::Element(el) = &vnode {
            vnode.set_element(self.api.create_node(&el.selector));
        }
        // then also create all its children
        if let VirtualNode::Element(el) = vnode {
            if el.children.is_some() {
                let children = el.children.as_mut().unwrap();
                for i in 0..children.len() {
                    self.create_element(&mut children[i]);
                }
            }
        }
        // if its text, create the text node
        if let VirtualNode::Text(tx) = &vnode {
            vnode.set_element(self.api.create_text_node(&tx.text));
        }
    }

    pub fn patch(&mut self, mut new_vnode: VirtualNode) {
        if let None = self.current_vnode {
            self.current_vnode = Some(new_vnode);
            return;
        }
        self.inserted_vnodes.clear();

        // Tell all extensions we are about to patch
        for e in self.extensions.iter() {
            e.pre();
        }

        if let Some(old_node) = &self.current_vnode {
            if old_node.same(&new_vnode) {
                // If nodes look like they are the same
            } else {
                // If nodes look like they are completely different
                let parent_element = old_node
                    .get_parent_element()
                    .expect("should always be a parent element");
                self.create_element(&mut new_vnode);
                let d = self.dom();
                //let new_element = new_vnode.get_element();
                //d.insert_before(parent,new_vnode.get_element(),d.next_sibling(old_node.get_element()))
            }
        }

        // tell all extensions patching is entirely complete
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
