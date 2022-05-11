//!
//!
//! author: https://github.com/vincenzopalazzo
pub mod builtin;
pub mod json_utils;
pub mod types;

use serde_json;

use super::plugin::Plugin;

pub trait RPCMethod<T: Clone>: RPCMethodClone<T> {
    fn call<'c>(&self, plugin: &mut Plugin<T>, request: &'c serde_json::Value)
        -> serde_json::Value;
}

// Splitting RPCMethodClone into its own trait allows us to provide a blanket
// implementation for all compatible types, without having to implement the
// rest of RPCMethod. In this case, we implement it for all types that have
// 'static lifetime (*i.e.* they don't contain non-'static pointers), and
// implement both RPCMethod and Clone.  Don't ask me how the compiler resolves
// implementing RPCMethodClone for dyn Animal when RPCMethod requires RPCMethodClone;
// I have *no* idea why this works.
pub trait RPCMethodClone<T: Clone> {
    fn clone_box(&self) -> Box<dyn RPCMethod<T>>;
}

impl<'a, F, T: Clone> RPCMethodClone<T> for F
where
    F: 'static + RPCMethod<T> + Clone,
{
    fn clone_box(&self) -> Box<dyn RPCMethod<T>> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl<T: Clone> Clone for Box<dyn RPCMethod<T>> {
    fn clone(&self) -> Box<dyn RPCMethod<T>> {
        self.clone_box()
    }
}
