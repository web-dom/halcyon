use crate::dom::{Element, DOM};
use crate::extensions::Extension;
use std::cell::RefCell;
use std::rc::Rc;
pub mod dom;
pub mod extensions;
mod store;

pub use crate::store::Reducer;
pub use crate::store::Store;

#[derive(Debug)]
pub struct Halcyon {
    api: Box<DOM>,
    current_vnode: RefCell<Option<VirtualNode>>,
    extensions: Vec<Box<Extension>>,
}

impl Halcyon {
    pub fn new(api: Box<DOM>, extensions: Vec<Box<Extension>>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: RefCell::new(None),
            extensions: extensions,
        }
    }

    pub fn has_patched(&self) -> bool {
        let c = self.current_vnode.borrow();
        if let None = *c {
            return false;
        }
        return true;
    }

    pub fn patch(&self, new_vnode: VirtualNode) {
        let mut c = self.current_vnode.borrow_mut();
        if let None = *c {
            *c = Some(new_vnode);
            return;
        }

        for e in self.extensions.iter() {
            e.pre();
        }
        *c = Some(new_vnode);
        for e in self.extensions.iter() {
            e.post();
        }
    }
}

#[derive(Debug)]
pub enum VirtualNode {
    Element(VirtualNodeElement),
    Text(VirtualNodeText),
}

impl VirtualNode {
    pub fn from_element(e: Rc<RefCell<Element>>) -> VirtualNode {
        VirtualNode::Element(VirtualNodeElement {
            selector: String::from("div"),
            data: None,
            children: None,
            element: Some(e),
            list_key: None,
        })
    }
}

type VirtualNodeData = i32;
type Key = i32;

#[derive(Debug)]
pub struct VirtualNodeElement {
    selector: String,
    data: Option<VirtualNodeData>,
    children: Option<Vec<VirtualNode>>,
    element: Option<Rc<RefCell<Element>>>,
    list_key: Option<Key>,
}

#[derive(Debug)]
pub struct VirtualNodeText {
    element: Option<Rc<RefCell<Element>>>,
    text: String,
}

pub fn h(
    selector: &str,
    data: Option<VirtualNodeData>,
    children: Option<Vec<VirtualNode>>,
) -> VirtualNode {
    VirtualNode::Element(VirtualNodeElement {
        selector: String::from(selector),
        data: data,
        children: children,
        element: None,
        list_key: None,
    })
}

pub fn t(text: &str) -> VirtualNode {
    VirtualNode::Text(VirtualNodeText {
        element: None,
        text: String::from(text),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::{MemoryDOM, MemoryElement};
    use crate::extensions::attributes::Attributes;

    fn render(element: Rc<RefCell<Element>>, container: VirtualNode) {
        thread_local! {
            static HALCYON:Halcyon = Halcyon::new(MemoryDOM::new(),vec![Box::new(Attributes::new())]);
        };
        HALCYON.with(|halcyon| {
            if !halcyon.has_patched() {
                // If it's our first time
                // Render the existing element
                halcyon.patch(VirtualNode::from_element(element));
            }
            // Render the new virtual dom
            halcyon.patch(container);
            println!("{:?}", halcyon);
        });
    }

    fn hello_world(name: Option<&str>) -> VirtualNode {
        let n = match name {
            Some(v) => v,
            _ => "World",
        };
        h("div", None, Some(vec![t(&format!("Hello {}", n))]))
    }

    #[test]
    fn it_works() {
        let body = MemoryElement::new("body");
        render(body.clone(), hello_world(None));
        render(body.clone(), hello_world(Some("Richard")));
    }
}
