use crate::extensions::Extension;
use crate::VirtualNode;
pub struct Attributes {}

impl Extension for Attributes {
    fn pre(&self){
        println!("started")
    }
    fn create(&self, old_vnode: &VirtualNode, new_vnode: &VirtualNode) {
        update_attributes(old_vnode, new_vnode);
    }
    fn update(&self, old_vnode: &VirtualNode, new_vnode: &VirtualNode) {
        update_attributes(old_vnode, new_vnode);
    }
    fn post(&self){
        println!("ended")
    }
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {}
    }
}

fn update_attributes(old_vnode: &VirtualNode, new_vnode: &VirtualNode) {}
