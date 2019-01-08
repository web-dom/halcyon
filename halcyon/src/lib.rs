pub use crate::dom::{Element, DOM};
use crate::extensions::Extension;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::LocalKey;

mod dom;
mod extensions;
mod helpers;
mod props;
mod store;
mod vnode;

pub use crate::extensions::attributes::Attributes;
pub use crate::helpers::{h, t};
pub use crate::props::{Prop, Props};
pub use crate::store::{Reducer, Store};
pub use crate::vnode::{VirtualNode, VirtualNodeElement, VirtualNodeText};

#[derive(Debug)]
pub struct Halcyon<D: DOM<E>, E: Element> {
    api: D,
    current_vnode: Option<VirtualNode<E>>,
    extensions: Vec<Box<Extension<E>>>,
    inserted_vnodes: Vec<VirtualNode<E>>,
}

impl<D, E> Halcyon<D, E>
where
    D: DOM<E>,
    E: Element,
{
    pub fn setup<T: Clone + Reducer<P>, P, Q: 'static + Fn() -> VirtualNode<E>>(
        halcyon: &'static LocalKey<RefCell<Halcyon<D, E>>>,
        store: &'static LocalKey<RefCell<Store<T, P>>>,
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

    pub fn root(&self) -> Option<&VirtualNode<E>> {
        match self.current_vnode.as_ref() {
            Some(s) => Some(s),
            None => None,
        }
    }

    pub fn new<M, N>(api: M) -> Halcyon<M, N>
    where
        M: DOM<N>,
        N: Element,
    {
        Halcyon {
            api: api,
            current_vnode: None,
            extensions: vec![Box::new(Attributes::new())],
            inserted_vnodes: vec![],
        }
    }

    pub fn custom<M, N>(api: M, extensions: Vec<Box<Extension<N>>>) -> Halcyon<M, N>
    where
        M: DOM<N>,
        N: Element,
    {
        Halcyon {
            api: api,
            current_vnode: None,
            extensions: extensions,
            inserted_vnodes: vec![],
        }
    }

    pub fn dom(&self) -> &'_ D {
        return &self.api;
    }

    pub fn create_element(&self, vnode: &mut VirtualNode<E>) {
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
                    let created_element = children[i].get_element().expect("should have element we just created");
                    let parent_element = el.element.as_mut().expect("parent should have element here");
                    parent_element.append_child(created_element);
                }
            }
        }
        // if its text, create the text node
        if let VirtualNode::Text(tx) = &vnode {
            vnode.set_element(self.api.create_text_node(&tx.text));
        }
    }

    /*fn remove_vnodes(&self, parent_element: E, vnodes: Vec<&mut VirtualNode<E>>, start: usize, end: usize) {
        for i in start..end {
            match vnodes[i] {
                VirtualNode::Element(el) => (),
                VirtualNode::Text(el) => vnodes[i].get_element().expect("if we are removing, it should have element").remove(),
            }
        }
    }*/

    pub fn patch(&mut self, mut new_vnode: VirtualNode<E>) {
        if let None = self.current_vnode {
            self.current_vnode = Some(new_vnode);
            return;
        }
        self.inserted_vnodes.clear();

        // Tell all extensions we are about to patch
        for e in self.extensions.iter() {
            e.pre();
        }
        if let Some(old_vnode) = self.current_vnode.as_ref() {
            if old_vnode.same(&new_vnode) {
                // If nodes look like they are the same
            } else {
                // If nodes look like they are completely different
                let mut parent_element = old_vnode
                    .get_parent_element()
                    .expect("should always be a parent element");
                self.create_element(&mut new_vnode);
                let new_element = new_vnode
                    .get_element()
                    .expect("this should have element because we just made them");
                let old_element = old_vnode
                    .get_element()
                    .expect("this should have element because it was put up on screen");
                let mut old_next_sibling = old_element.next_sibling();
                parent_element.insert_before(new_element, old_next_sibling.as_mut());
            }
        }
        if let Some(old_vnode) = self.current_vnode.as_mut() {
            if !old_vnode.same(&new_vnode) {
                // if they were not the same
                let e:&mut E = old_vnode.get_element_mut().expect("if its old it should have element");
                e.remove();
                self.current_vnode = Some(new_vnode);
            }
        }

        // tell all extensions patching is entirely complete
        for e in self.extensions.iter() {
            e.post();
        }
    }

    pub fn init_render(&mut self, element: E, container: VirtualNode<E>) {
        self.patch(VirtualNode::from_element(element));
        self.patch(container);
    }

    pub fn render(&mut self, container: VirtualNode<E>) {
        self.patch(container);
    }
}
