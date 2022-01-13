//! Core of the plugin API

pub trait Plugin {
    /// Main function that start the handshake process
    /// with c-lightning
    fn start(&self) {
        // TODO start here the process of the handshake
        // so the API should provide a default implementation
        // of this method
    }
}
