//! macros module containing all the necessary macros to work with the plugin
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

/// register_notification - give the possibility to register a notification
#[macro_export]
macro_rules! register_notification {
    ($plugin:expr, $notification:ident) => {
        let callback = $notification::new();
        $plugin.register_notification(callback.on_event.as_str(), callback.clone());
    };
}

/// emit a compiler error
#[macro_export]
macro_rules! error {
    ($($msg:tt)*) => {{
        let msg = format!($($msg)*);
        PluginError::new(-1, &msg, None)
    }};
}

pub use add_rpc;
pub use error;
pub use register_notification;
