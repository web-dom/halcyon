use crate::extensions::Extension;
use crate::VirtualNode;

#[derive(Debug)]
pub struct Attributes {}

impl Extension for Attributes {
    fn pre(&self) {
        println!("Attributes Extension::started")
    }
    fn create(&self, old_vnode: &VirtualNode, new_vnode: &VirtualNode) {
        update_attributes(old_vnode, new_vnode);
    }
    fn update(&self, old_vnode: &VirtualNode, new_vnode: &VirtualNode) {
        update_attributes(old_vnode, new_vnode);
    }
    fn post(&self) {
        println!("Attributes Extension::ended")
    }
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {}
    }
}

fn update_attributes(old_vnode: &VirtualNode, new_vnode: &VirtualNode) {}
