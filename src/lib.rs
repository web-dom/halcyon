use crate::extensions::attributes::Attributes;
use crate::extensions::Extension;
use std::cell::RefCell;
use std::rc::Rc;
mod extensions;

type Element = i32;

pub struct Halcyon {
    current_vnode: VirtualNode,
    pre_handlers: Vec<Rc<RefCell<Box<PreHandler>>>>,
    init_handlers: Vec<Rc<RefCell<Box<InitHandler>>>>,
    create_handlers: Vec<Rc<RefCell<Box<CreateHandler>>>>,
    insert_handlers: Vec<Rc<RefCell<Box<InsertHandler>>>>,
    pre_patch_handlers: Vec<Rc<RefCell<Box<PrePatchHandler>>>>,
    update_handlers: Vec<Rc<RefCell<Box<UpdateHandler>>>>,
    post_patch_handlers: Vec<Rc<RefCell<Box<PostPatchHandler>>>>,
    destroy_handlers: Vec<Rc<RefCell<Box<DestroyHandler>>>>,
    remove_handlers: Vec<Rc<RefCell<Box<RemoveHandler>>>>,
    post_handlers: Vec<Rc<RefCell<Box<PostHandler>>>>,
}

trait PreHandler {
    fn handle(&self);
}

trait InitHandler {
    fn handle(&self,vnode:&VirtualNode);
}

trait CreateHandler {
    fn handle(&self,empty_vnode:&VirtualNode,new_vnode:&VirtualNode);
}

trait InsertHandler {
    fn handle(&self,vnode:&VirtualNode);
}

trait PrePatchHandler {
    fn handle(&self,old_vnode:&VirtualNode,new_vnode:&VirtualNode);
}

trait UpdateHandler {
    fn handle(&self,old_vnode:&VirtualNode,new_vnode:&VirtualNode);
}

trait PostPatchHandler {
    fn handle(&self,old_vnode:&VirtualNode,new_vnode:&VirtualNode);
}

trait DestroyHandler {
    fn handle(&self,vnode:&VirtualNode);
}

trait RemoveHandler {
    fn handle(&self,vnode:&VirtualNode);
}

trait PostHandler {
    fn handle(&self);
}

/*
pre	the patch process begins	none
init	a VirtualNode has been added	VirtualNode
create	a DOM element has been created based on a VirtualNode	emptyVnode, VirtualNode
insert	an element has been inserted into the DOM	VirtualNode
prepatch	an element is about to be patched	oldVnode, VirtualNode
update	an element is being updated	oldVnode, VirtualNode
postpatch	an element has been patched	oldVnode, VirtualNode
destroy	an element is directly or indirectly being removed	VirtualNode
remove	an element is directly being removed from the DOM	VirtualNode, removeCallback
post	the patch process is done	none
*/

impl Halcyon {
    pub fn new() -> Halcyon {
        Halcyon {
            current_vnode: VirtualNode::Empty,
            pre_handlers: Vec::new(),
            init_handlers: Vec::new(),
            create_handlers: Vec::new(),
            insert_handlers: Vec::new(),
            pre_patch_handlers: Vec::new(),
            update_handlers: Vec::new(),
            post_patch_handlers: Vec::new(),
            destroy_handlers: Vec::new(),
            remove_handlers: Vec::new(),
            post_handlers: Vec::new(),
        }
    }

    pub fn add_extensions(halcyon:&'static std::thread::LocalKey<Halcyon>, extensions:Vec<Box<Extension>>){
        for e in extensions.iter() {
            e.attach_hooks(&halcyon);
        }
    }

    pub fn patch(&self, new_vnode:VirtualNode){

    }

    pub fn add_pre_hook_handler(&self, handler:Box<PreHandler>){

    }

    pub fn add_init_hook_handler(&self, handler:Box<InitHandler>){

    }

    pub fn add_create_hook_handler(&self, handler:Box<&CreateHandler>){

    }

    pub fn add_insert_hook_handler(&self, handler:Box<InsertHandler>){

    }

    pub fn add_pre_patch_hook_handler(&self, handler:Box<PrePatchHandler>){

    }

    pub fn add_update_hook_handler(&self, handler:Box<UpdateHandler>){

    }

    pub fn add_post_patch_hook_handler(&self, handler:Box<PostPatchHandler>){

    }

    pub fn add_destroy_hook_handler(&self, handler:Box<DestroyHandler>){

    }

    pub fn add_remove_hook_handler(&self, handler:Box<RemoveHandler>){

    }

    pub fn add_post_hook_handler(&self, handler:Box<PostHandler>){

    }
}

pub enum VirtualNode {
    Empty,
    Element(VirtualNodeElement)
}

pub struct VirtualNodeElement {

}

pub fn h() -> VirtualNode {
    VirtualNode::Element(VirtualNodeElement{})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        thread_local!{
            static HALCYON:Halcyon = Halcyon::new();
        };
        Halcyon::add_extensions(&HALCYON,vec![
            Attributes::new()
        ]);
        HALCYON.with(|halcyon|{
            halcyon.patch(h());
        })
    }
}
