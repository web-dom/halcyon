use crate::extensions::attributes::Attributes;
use crate::extensions::Extension;
use std::cell::RefCell;
use std::rc::Rc;
mod extensions;

type Element = i32;

pub struct Halcyon {
    target_element: Element,
    current_vnode: VNode,
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
    fn handle(&self,vnode:&VNode);
}

trait CreateHandler {
    fn handle(&self,empty_vnode:&VNode,new_vnode:&VNode);
}

trait InsertHandler {
    fn handle(&self,vnode:&VNode);
}

trait PrePatchHandler {
    fn handle(&self,old_vnode:&VNode,new_vnode:&VNode);
}

trait UpdateHandler {
    fn handle(&self,old_vnode:&VNode,new_vnode:&VNode);
}

trait PostPatchHandler {
    fn handle(&self,old_vnode:&VNode,new_vnode:&VNode);
}

trait DestroyHandler {
    fn handle(&self,vnode:&VNode);
}

trait RemoveHandler {
    fn handle(&self,vnode:&VNode);
}

trait PostHandler {
    fn handle(&self);
}

/*
pre	the patch process begins	none
init	a vnode has been added	vnode
create	a DOM element has been created based on a vnode	emptyVnode, vnode
insert	an element has been inserted into the DOM	vnode
prepatch	an element is about to be patched	oldVnode, vnode
update	an element is being updated	oldVnode, vnode
postpatch	an element has been patched	oldVnode, vnode
destroy	an element is directly or indirectly being removed	vnode
remove	an element is directly being removed from the DOM	vnode, removeCallback
post	the patch process is done	none
*/

impl Halcyon {
    pub fn new(target_element:Element) -> Halcyon {
        Halcyon {
            current_vnode: VNode{},
            target_element: target_element,
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

    pub fn patch(&self, new_vnode:VNode){

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

pub struct VNode {

}

pub fn h() -> VNode {
    VNode{}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        thread_local!{
            static HALCYON:Halcyon = Halcyon::new(0);
        };
        Halcyon::add_extensions(&HALCYON,vec![
            Attributes::new()
        ]);
        HALCYON.with(|halcyon|{
            halcyon.patch(h());
        })
    }
}
