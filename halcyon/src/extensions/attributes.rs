use crate::dom::Element;
use crate::extensions::Extension;
use crate::VirtualNode;

#[derive(Debug)]
pub struct Attributes {}

impl<E> Extension<E> for Attributes
where
    E: Element,
{
    fn pre(&self) {
    }
    fn create(&self, old_vnode: &VirtualNode<E>, new_vnode: &VirtualNode<E>) {
        update_attributes(old_vnode, new_vnode);
    }
    fn update(&self, old_vnode: &VirtualNode<E>, new_vnode: &VirtualNode<E>) {
        update_attributes(old_vnode, new_vnode);
    }
    fn post(&self) {
    }
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {}
    }
}

fn update_attributes<E>(_old_vnode: &VirtualNode<E>, _new_vnode: &VirtualNode<E>)
where
    E: Element,
{
}
