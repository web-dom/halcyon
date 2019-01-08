use crate::dom::Element;
use crate::VirtualNode;
use std::fmt::Debug;

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

pub trait Extension<E>: Debug
where
    E: Element,
{
    fn pre(&self) {}
    fn init(&self, _vnode: &VirtualNode<E>) {}
    fn create(&self, _empty_vnode: &VirtualNode<E>, _new_vnode: &VirtualNode<E>) {}
    fn insert(&self, _vnode: &VirtualNode<E>) {}
    fn pre_patch(&self, _old_vnode: &VirtualNode<E>, _new_vnode: &VirtualNode<E>) {}
    fn update(&self, _old_vnode: &VirtualNode<E>, _new_vnode: &VirtualNode<E>) {}
    fn handle(&self, _old_vnode: &VirtualNode<E>, _new_vnode: &VirtualNode<E>) {}
    fn destroy(&self, _vnode: &VirtualNode<E>) {}
    fn remove(&self, _vnode: &VirtualNode<E>) {}
    fn post(&self) {}
}

pub mod attributes;
