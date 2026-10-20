//! Answer a request after its handler has returned.
//!
//! The plugin reads one request at a time from lightningd. A handler that
//! overrides [`RPCCommand::call_deferred`](crate::commands::RPCCommand::call_deferred)
//! gets a [`Responder`] it can move to another thread or task, so the
//! plugin goes on reading requests while the first one is still being
//! served. JSON-RPC responses carry the request id, so they may be written
//! in any order.
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use clightningrpc_common::json_utils::init_success_response;
use clightningrpc_common::types::Id;
use serde_json::Value;

use crate::errors::PluginError;

/// The plugin's output to lightningd, shared by every pending request.
pub(crate) struct Output {
    writer: Mutex<Box<dyn Write + Send>>,
    failed: AtomicBool,
}

impl Output {
    pub(crate) fn new<W: Write + Send + 'static>(writer: W) -> Arc<Self> {
        Arc::new(Output {
            writer: Mutex::new(Box::new(writer)),
            failed: AtomicBool::new(false),
        })
    }

    /// Write one message in a single `write_all`, so responses written from
    /// several threads never interleave.
    fn write(&self, message: &Value) {
        let bytes = serde_json::to_string(message).unwrap();
        // A panic while writing must not stop the other responses.
        let mut writer = self
            .writer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        // If the write fails the parent (lightningd) has gone away.
        if writer.write_all(bytes.as_bytes()).is_err() || writer.flush().is_err() {
            self.failed.store(true, Ordering::SeqCst);
        }
    }

    /// Whether a write failed: lightningd is gone and the plugin should stop.
    pub(crate) fn failed(&self) -> bool {
        self.failed.load(Ordering::SeqCst)
    }
}

/// Sends the response to one request, from any thread.
///
/// Dropping a `Responder` without calling [`Responder::respond`] answers the
/// request with an error, so lightningd never waits forever.
pub struct Responder {
    id: Option<Id>,
    output: Arc<Output>,
}

impl Responder {
    pub(crate) fn new(id: Id, output: Arc<Output>) -> Self {
        Responder {
            id: Some(id),
            output,
        }
    }

    /// The id of the request this responder answers.
    pub fn id(&self) -> &Id {
        self.id
            .as_ref()
            .expect("the id is only taken when answering")
    }

    /// Answer the request with `result`.
    pub fn respond(mut self, result: Result<Value, PluginError>) {
        let id = self.id.take().expect("a request is answered once");
        self.send(id, result);
    }

    fn send(&self, id: Id, result: Result<Value, PluginError>) {
        let mut response = init_success_response(id);
        match result {
            Ok(value) => response["result"] = value,
            Err(err) => response["error"] = serde_json::to_value(err).unwrap(),
        }
        self.output.write(&response);
    }
}

impl Drop for Responder {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            self.send(
                id,
                Err(PluginError::new(
                    -1,
                    "the plugin dropped the request without answering",
                    None,
                )),
            );
        }
    }
}
