pub use crate::dom::{Element, DOM};
use crate::extensions::Extension;
use std::cell::RefCell;
use std::rc::Rc;
pub mod dom;
pub mod extensions;
pub mod props;
mod store;

pub use crate::dom::MemoryDOM;
pub use crate::dom::MemoryElement;
pub use crate::extensions::attributes::Attributes;
pub use crate::props::Prop;
pub use crate::props::Props;
pub use crate::store::Reducer;
pub use crate::store::Store;

#[derive(Debug)]
pub struct Halcyon {
    api: Box<DOM>,
    current_vnode: RefCell<Option<VirtualNode>>,
    extensions: Vec<Box<Extension>>,
}

impl Halcyon {
    pub fn setup<T: Clone + Reducer<P>, P>(
        halcyon: &'static std::thread::LocalKey<Halcyon>,
        store: &'static std::thread::LocalKey<RefCell<Store<T, P>>>,
        target: &str,
        render: Box<Fn() -> VirtualNode>,
    ) {
        let node_renderer = Rc::new(render);
        halcyon.with(|h| {
            let t = h.dom().query_selector(target);
            // Get the body as our target element
            // Do initial render to element
            h.init_render(t, node_renderer());
        });
        let render_ref = node_renderer.clone();
        store.with(|s| {
            // Add a listener to listen for state changes
            s.borrow().add_listener(Box::new(|| {
                halcyon.with(|h| {
                    // Rerender everything again with new virtual dom
                    h.render(render_ref());
                })
            }));
        });
    }

    pub fn new(api: Box<DOM>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: RefCell::new(None),
            extensions: vec![Box::new(Attributes::new())],
        }
    }

    pub fn custom(api: Box<DOM>, extensions: Vec<Box<Extension>>) -> Halcyon {
        Halcyon {
            api: api,
            current_vnode: RefCell::new(None),
            extensions: extensions,
        }
    }

    pub fn dom(&self) -> &'_ Box<DOM> {
        return &self.api;
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

    pub fn init_render(&self, element: Rc<RefCell<Element>>, container: VirtualNode) {
        self.patch(VirtualNode::from_element(element));
        self.patch(container);
    }

    pub fn render(&self, container: VirtualNode) {
        self.patch(container);
    }
}

#[derive(Debug, PartialEq)]
pub enum VirtualNode {
    Element(VirtualNodeElement),
    Text(VirtualNodeText),
}

impl<T> From<T> for VirtualNode
where
    T: std::fmt::Display,
{
    fn from(v: T) -> VirtualNode {
        t(&format!("{}", v))
    }
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

type VirtualNodeData = props::Props;
type Key = i32;

#[derive(Debug)]
pub struct VirtualNodeElement {
    selector: String,
    data: Option<VirtualNodeData>,
    children: Option<Vec<VirtualNode>>,
    element: Option<Rc<RefCell<Element>>>,
    list_key: Option<Key>,
}

impl PartialEq for VirtualNodeElement {
    fn eq(&self, other: &VirtualNodeElement) -> bool {
        self.selector == other.selector
            && self.data == other.data
            && self.list_key == other.list_key
            && self.children == other.children
    }
}

#[derive(Debug)]
pub struct VirtualNodeText {
    element: Option<Rc<RefCell<Element>>>,
    text: String,
}

impl PartialEq for VirtualNodeText {
    fn eq(&self, other: &VirtualNodeText) -> bool {
        self.text == other.text
    }
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
            static HALCYON:Halcyon = Halcyon::new(MemoryDOM::new());
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
