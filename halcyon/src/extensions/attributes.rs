use crate::dom::Element;
use crate::extensions::Extension;
use crate::props::Prop;
use crate::VirtualNodeElement;

#[derive(Debug)]
pub struct Attributes {}

impl<E> Extension<E> for Attributes
where
    E: Element,
{
    fn pre(&self) {}
    fn create(&self, new_vnode: &mut VirtualNodeElement<E>) {
        update_attributes(None, new_vnode);
    }
    fn update(&self, old_vnode: &mut VirtualNodeElement<E>, new_vnode: &mut VirtualNodeElement<E>) {
        update_attributes(Some(old_vnode), new_vnode);
    }
    fn post(&self) {}
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {}
    }
}

fn update_attributes<E>(
    _old_vnode: Option<&mut VirtualNodeElement<E>>,
    new_vnode: &mut VirtualNodeElement<E>,
) where
    E: Element,
{
    if let Some(data) = new_vnode.data.as_mut() {
        for (n, v) in data {
            match v {
                Prop::String(s) => new_vnode.element.as_mut().unwrap().set_attribute(&n, &s),
                _ => (),
            }
        }
    }
}
