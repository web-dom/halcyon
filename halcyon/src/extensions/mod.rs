use crate::dom::Element;
use crate::VirtualNodeElement;
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
    fn init(&self, _vnode: &VirtualNodeElement<E>) {}
    fn create(&self, _new_vnode: &mut VirtualNodeElement<E>) {}
    fn insert(&self, _vnode: &VirtualNodeElement<E>) {}
    fn pre_patch(&self, _old_vnode: &VirtualNodeElement<E>, _new_vnode: &VirtualNodeElement<E>) {}
    fn update(
        &self,
        _old_vnode: &mut VirtualNodeElement<E>,
        _new_vnode: &mut VirtualNodeElement<E>,
    ) {
    }
    fn handle(&self, _old_vnode: &VirtualNodeElement<E>, _new_vnode: &VirtualNodeElement<E>) {}
    fn destroy(&self, _vnode: &VirtualNodeElement<E>) {}
    fn remove(&self, _vnode: &VirtualNodeElement<E>) {}
    fn post(&self) {}
}

pub mod attributes;
