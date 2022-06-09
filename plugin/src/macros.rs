//! macros module contains all the necessary macros to work with the plugin
//! and register new RPC method/Hooks/Subscription.
//!
//! author: https://github.com/vincenzopalazzo

#[macro_export]
macro_rules! add_rpc {
    ($plugin:expr, $method:ident) => {
        let rpc = $method::new();
        $plugin.add_rpc_method(
            rpc.name.as_str(),
            rpc.usage.as_str(),
            rpc.description.as_str(),
            rpc.clone(),
        );
    };
}
