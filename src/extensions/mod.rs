use crate::Halcyon;

pub trait Extension {
    fn attach_hooks(&self,halcyon:&'static std::thread::LocalKey<Halcyon>);
}

pub mod attributes;
