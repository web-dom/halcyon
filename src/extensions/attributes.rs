use crate::extensions::Extension;
use crate::{VNode,Halcyon,UpdateHandler,CreateHandler};
pub struct Attributes {}

impl Extension for Attributes {
    fn attach_hooks(&self,halcyon:&'static std::thread::LocalKey<Halcyon>){
        halcyon.with(|h|{
            h.add_create_hook_handler(Box::new(self));
        });
    }
}

fn update_attributes(old_vnode:&VNode,new_vnode:&VNode){

}

impl CreateHandler for Attributes {
    fn handle(&self,old_vnode:&VNode,new_vnode:&VNode){
        update_attributes(old_vnode,new_vnode);
    }
}

impl UpdateHandler for Attributes {
    fn handle(&self,old_vnode:&VNode,new_vnode:&VNode){
        update_attributes(old_vnode,new_vnode);
    }
}

impl Attributes {
    pub fn new() -> Box<Extension> {
        Box::new(Attributes {})
    }
}
