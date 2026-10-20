//! Core of the plugin API
//!
//! Unofficial API interface to develop plugin in Rust.
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{BufRead, Write};
use std::string::String;
use std::sync::Arc;

use clightningrpc_common::json_utils::{add_str, init_payload};
use clightningrpc_common::types::Request;
use serde_json::Value;

use crate::commands::builtin::{InitRPC, ManifestRPC};
use crate::commands::types::{CLNConf, RPCHookInfo, RPCMethodInfo};
use crate::commands::RPCCommand;
use crate::errors::PluginError;
use crate::responder::{Output, Responder};
use crate::types::{LogLevel, RpcOption};

#[cfg(feature = "log")]
pub use log::*;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Plugin<T>
where
    // FIXME: move the static life time to a local life time for plugin
    T: 'static + Clone,
{
    pub state: T,
    /// all the option contained inside the
    /// hash map.
    pub option: HashMap<String, RpcOption>,
    /// all the options rpc method that the
    /// plugin need to support, included the builtin rpc method.
    pub rpc_method: HashMap<String, Box<dyn RPCCommand<T>>>,
    /// keep the info of the method in a separate list
    /// FIXME: move the RPCMethodInfo as key of the rpc_method map.
    pub rpc_info: HashSet<RPCMethodInfo>,
    /// all the hook where the plugin is register during the configuration
    pub rpc_hook: HashMap<String, Box<dyn RPCCommand<T>>>,
    /// keep all the info about the hooks in a separate set.
    /// FIXME: put the RPCHookInfo as key of the hash map.
    pub hook_info: HashSet<RPCHookInfo>,
    /// all the notification that the plugin is register on
    pub rpc_notification: HashMap<String, Box<dyn RPCCommand<T>>>,
    /// mark a plugin as dynamic, in this way the plugin can be run
    /// from core lightning without stop the lightningd daemon
    pub dynamic: bool,
    /// core lightning configuration sent with the init call.
    pub configuration: Option<CLNConf>,
    /// onInit callback called when the method on init is ran.
    on_init: Option<Arc<dyn Fn(&mut Plugin<T>) -> Value>>,
}

#[cfg(feature = "log")]
pub struct Log;

#[cfg(feature = "log")]
impl log::Log for Log {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level: LogLevel = record.level().into();
            let msg = record.args();

            let mut writer = io::stdout();
            let mut payload = init_payload();
            add_str(&mut payload, "level", &level.to_string());
            add_str(&mut payload, "message", &format!("{msg}"));
            let request = Request {
                id: None,
                jsonrpc: "2.0".to_owned(),
                method: "log".to_owned(),
                params: payload,
            };
            let _ = writer.write_all(serde_json::to_string(&request).unwrap().as_bytes());
            let _ = writer.flush();
        }
    }

    fn flush(&self) {}
}

impl<'a, T: 'a + Clone> Plugin<T> {
    pub fn new(state: T, dynamic: bool) -> Self {
        Plugin {
            state,
            option: HashMap::new(),
            rpc_method: HashMap::new(),
            rpc_info: HashSet::new(),
            rpc_hook: HashMap::new(),
            hook_info: HashSet::new(),
            rpc_notification: HashMap::new(),
            dynamic,
            configuration: None,
            on_init: None,
        }
    }

    pub fn on_init<C: 'static>(&'a mut self, callback: C) -> Self
    where
        C: Fn(&mut Plugin<T>) -> Value,
    {
        self.on_init = Some(Arc::new(callback));
        self.clone()
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        let mut writer = io::stdout();
        let mut payload = init_payload();
        add_str(&mut payload, "level", &level.to_string());
        add_str(&mut payload, "message", msg);
        let request = Request {
            id: None,
            jsonrpc: "2.0".to_owned(),
            method: "log".to_owned(),
            params: payload,
        };
        // Best-effort: if the parent (lightningd) has gone away the write
        // will fail with BrokenPipe. Drop the error rather than panic — the
        // main read loop will observe EOF on stdin and exit cleanly.
        let _ = writer.write_all(serde_json::to_string(&request).unwrap().as_bytes());
        let _ = writer.flush();
    }

    /// register the plugin option.
    pub fn add_opt(
        &mut self,
        name: &str,
        opt_type: &str,
        def_val: Option<String>,
        description: &str,
        deprecated: bool,
    ) -> &mut Self {
        let def_val = match opt_type {
            "flag" | "bool" => {
                def_val.and_then(|val| Some(serde_json::json!(val.parse::<bool>().unwrap())))
            }
            "int" => def_val.and_then(|val| Some(serde_json::json!(val.parse::<i64>().unwrap()))),
            "string" => def_val.and_then(|val| Some(serde_json::json!(val))),
            _ => unreachable!("{opt_type} not supported"),
        };
        self.option.insert(
            name.to_owned(),
            RpcOption {
                name: name.to_string(),
                opt_typ: opt_type.to_string(),
                default: def_val,
                description: description.to_string(),
                deprecated,
                value: None,
            },
        );
        self
    }

    /// get an optionue that cln sent back to the plugin.
    pub fn get_opt<R: for<'de> serde::de::Deserialize<'de>>(&self, name: &str) -> Option<R> {
        self.option.get(name).and_then(|value| value.value())
    }

    // FIXME: adding the long description as parameter
    pub fn add_rpc_method<F: 'static>(
        &'a mut self,
        name: &str,
        usage: &str,
        description: &str,
        callback: F,
    ) -> Self
    where
        F: RPCCommand<T> + 'static,
    {
        self.rpc_method.insert(name.to_owned(), Box::new(callback));
        self.rpc_info.insert(RPCMethodInfo {
            name: name.to_string(),
            usage: usage.to_string(),
            description: description.to_string(),
            long_description: description.to_string(),
            deprecated: false,
        });
        self.clone()
    }

    /// Run the method or hook `name`, which answers through `responder`
    /// now or later.
    fn dispatch(&'a mut self, name: &str, params: serde_json::Value, responder: Responder) {
        let command = self
            .rpc_method
            .get(name)
            .or_else(|| self.rpc_hook.get(name))
            .cloned();
        match command {
            Some(command) => command.call_deferred(self, params, responder),
            None => responder.respond(Err(crate::error!(
                "callback for method/hook `{name}` not found"
            ))),
        }
    }

    fn handle_notification(&'a mut self, name: &str, params: serde_json::Value) {
        let notification = self.rpc_notification.get(name).unwrap().clone();
        notification.call_void(self, &params);
    }

    pub fn register_hook<F: 'static>(
        &'a mut self,
        hook_name: &str,
        before: Option<Vec<String>>,
        after: Option<Vec<String>>,
        callback: F,
    ) -> Self
    where
        F: RPCCommand<T> + 'static,
    {
        self.register_hook_with_filters(hook_name, before, after, None, callback)
    }

    /// Register a hook that Core Lightning only calls for the given
    /// `filters`: command names for `rpc_command` (Core Lightning v25.12 or
    /// later), message types for `custommsg`. Hooks that do not support
    /// filters make Core Lightning refuse the plugin.
    pub fn register_hook_with_filters<F: 'static>(
        &'a mut self,
        hook_name: &str,
        before: Option<Vec<String>>,
        after: Option<Vec<String>>,
        filters: Option<Vec<serde_json::Value>>,
        callback: F,
    ) -> Self
    where
        F: RPCCommand<T> + 'static,
    {
        self.rpc_hook
            .insert(hook_name.to_owned(), Box::new(callback));
        self.hook_info.insert(RPCHookInfo {
            name: hook_name.to_owned(),
            before,
            after,
            filters,
        });
        self.clone()
    }

    pub fn register_notification<F: 'static>(&mut self, name: &str, callback: F) -> Self
    where
        F: 'static + RPCCommand<T> + Clone,
    {
        self.rpc_notification
            .insert(name.to_owned(), Box::new(callback));
        self.clone()
    }

    pub fn start(self) {
        self.start_with_io(io::stdin().lock(), io::stdout());
    }

    /// Serve requests read from `reader` and write the responses to
    /// `writer`, until `reader` reaches EOF.
    fn start_with_io<R: BufRead, W: Write + Send + 'static>(mut self, mut reader: R, writer: W) {
        let output = Output::new(writer);
        let mut buffer = String::new();
        #[cfg(feature = "log")]
        {
            use std::str::FromStr;
            // We are compatible with the cln-plugin
            let level = std::env::var("CLN_PLUGIN_LOG").unwrap_or("info".to_string());
            let level = LevelFilter::from_str(&level).unwrap();
            let _ = log::set_logger(&Log {}).map(|()| log::set_max_level(level));
        }
        self.rpc_method
            .insert("getmanifest".to_owned(), Box::new(ManifestRPC {}));
        self.rpc_method.insert(
            "init".to_owned(),
            Box::new(InitRPC::<T> {
                on_init: self.on_init.clone(),
            }),
        );
        // Read input and handle EOF to prevent high CPU usage if the sending process dies
        while let Ok(n) = reader.read_line(&mut buffer) {
            if n == 0 {
                // EOF reached
                break;
            }
            let req_str = buffer.to_string();
            buffer.clear();
            let Ok(request) = serde_json::from_str::<Request<serde_json::Value>>(&req_str) else {
                continue;
            };
            if let Some(id) = request.id {
                // when the id is specified this is a RPC or Hook, so we need to return a response,
                // now or later.
                let responder = Responder::new(id, output.clone());
                self.dispatch(&request.method, request.params, responder);
                // If a write failed the parent (lightningd) has gone away;
                // exit cleanly instead of panicking on BrokenPipe.
                if output.failed() {
                    break;
                }
            } else {
                // in case of the id is None, we are receiving the notification, so the server is not
                // interested in the answer.
                self.handle_notification(&request.method, request.params);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::commands::builtin::ManifestRPC;

    #[derive(Clone)]
    struct Hook;

    impl RPCCommand<()> for Hook {}

    fn manifest_hooks(plugin: &mut Plugin<()>) -> Vec<Value> {
        let manifest = ManifestRPC {}.call(plugin, json!({})).unwrap();
        manifest["hooks"].as_array().unwrap().clone()
    }

    #[test]
    fn hooks_without_filters_keep_the_old_manifest() {
        let mut plugin = Plugin::new((), false);
        plugin.register_hook("htlc_accepted", None, None, Hook);
        let hooks = manifest_hooks(&mut plugin);
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0]["name"], "htlc_accepted");
        assert!(hooks[0].get("filters").is_none());
    }

    #[test]
    fn hook_filters_are_in_the_manifest() {
        let mut plugin = Plugin::new((), false);
        plugin.register_hook_with_filters(
            "rpc_command",
            None,
            None,
            Some(vec![json!("pay"), json!("xpay")]),
            Hook,
        );
        plugin.register_hook_with_filters("custommsg", None, None, Some(vec![json!(32769)]), Hook);
        let mut hooks = manifest_hooks(&mut plugin);
        hooks.sort_by_key(|h| h["name"].as_str().unwrap().to_owned());
        assert_eq!(hooks[0]["name"], "custommsg");
        assert_eq!(hooks[0]["filters"], json!([32769]));
        assert_eq!(hooks[1]["name"], "rpc_command");
        assert_eq!(hooks[1]["filters"], json!(["pay", "xpay"]));
    }

    /// A writer the test can read back while the plugin still holds it.
    #[derive(Clone, Default)]
    struct SharedBuf(Arc<std::sync::Mutex<Vec<u8>>>);

    impl Write for SharedBuf {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl SharedBuf {
        /// The responses written so far, in order.
        fn responses(&self) -> Vec<Value> {
            let bytes = self.0.lock().unwrap().clone();
            serde_json::Deserializer::from_slice(&bytes)
                .into_iter::<Value>()
                .map(|v| v.unwrap())
                .collect()
        }
    }

    fn request(id: u16, method: &str) -> String {
        format!(
            "{}\n",
            json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": {} })
        )
    }

    #[derive(Clone)]
    struct Echo(&'static str);

    impl RPCCommand<()> for Echo {
        fn call(&self, _: &mut Plugin<()>, _: Value) -> Result<Value, PluginError> {
            Ok(json!({ "from": self.0 }))
        }
    }

    /// Answers from another thread once the test sends it a value.
    #[derive(Clone)]
    struct Deferred {
        release: Arc<std::sync::Mutex<Option<std::sync::mpsc::Receiver<Value>>>>,
        done: std::sync::mpsc::Sender<()>,
    }

    impl RPCCommand<()> for Deferred {
        fn call_deferred(&self, _: &mut Plugin<()>, _: Value, responder: Responder) {
            let release = self.release.lock().unwrap().take().unwrap();
            let done = self.done.clone();
            std::thread::spawn(move || {
                let value = release.recv().unwrap();
                responder.respond(Ok(value));
                done.send(()).unwrap();
            });
        }
    }

    #[test]
    fn deferred_requests_do_not_block_the_next_ones() {
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let mut plugin = Plugin::new((), false);
        plugin.add_rpc_method(
            "slow",
            "",
            "",
            Deferred {
                release: Arc::new(std::sync::Mutex::new(Some(release_rx))),
                done: done_tx,
            },
        );
        plugin.add_rpc_method("fast", "", "", Echo("fast"));
        plugin.register_hook("htlc_accepted", None, None, Echo("hook"));

        let input = [
            request(1, "slow"),
            request(2, "fast"),
            request(3, "htlc_accepted"),
            request(4, "missing"),
        ]
        .concat();
        let output = SharedBuf::default();
        // Every request is read and the quick ones answered while `slow`
        // is still pending.
        plugin.start_with_io(io::Cursor::new(input), output.clone());
        let responses = output.responses();
        let ids: Vec<_> = responses.iter().map(|r| r["id"].clone()).collect();
        assert_eq!(ids, vec![json!(2), json!(3), json!(4)]);
        assert_eq!(responses[0]["result"], json!({ "from": "fast" }));
        assert_eq!(responses[1]["result"], json!({ "from": "hook" }));
        assert!(responses[2]["error"]["message"]
            .as_str()
            .unwrap()
            .contains("`missing` not found"));

        release_tx.send(json!({ "from": "slow" })).unwrap();
        done_rx.recv().unwrap();
        let responses = output.responses();
        assert_eq!(responses.len(), 4);
        assert_eq!(responses[3]["id"], json!(1));
        assert_eq!(responses[3]["result"], json!({ "from": "slow" }));
    }

    #[test]
    fn concurrent_responses_are_whole_messages() {
        #[derive(Clone)]
        struct Threaded;

        impl RPCCommand<()> for Threaded {
            fn call_deferred(&self, _: &mut Plugin<()>, request: Value, responder: Responder) {
                std::thread::spawn(move || responder.respond(Ok(request)));
            }
        }

        let mut plugin = Plugin::new((), false);
        plugin.add_rpc_method("threaded", "", "", Threaded);
        let count = 200u16;
        let input: String = (0..count)
            .map(|id| {
                let params = json!({ "id": id, "padding": "x".repeat(4096) });
                format!(
                    "{}\n",
                    json!({ "jsonrpc": "2.0", "id": id, "method": "threaded", "params": params })
                )
            })
            .collect();
        let output = SharedBuf::default();
        plugin.start_with_io(io::Cursor::new(input), output.clone());
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while output.responses().len() < count as usize {
            assert!(std::time::Instant::now() < deadline, "missing responses");
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        let mut ids: Vec<_> = output
            .responses()
            .iter()
            .map(|r| {
                assert_eq!(r["result"]["id"], r["id"]);
                r["id"].as_u64().unwrap()
            })
            .collect();
        ids.sort();
        assert_eq!(ids, (0..count as u64).collect::<Vec<_>>());
    }

    #[test]
    fn dropped_responder_answers_with_an_error() {
        #[derive(Clone)]
        struct Forgetful;

        impl RPCCommand<()> for Forgetful {
            fn call_deferred(&self, _: &mut Plugin<()>, _: Value, _responder: Responder) {}
        }

        let mut plugin = Plugin::new((), false);
        plugin.add_rpc_method("forgetful", "", "", Forgetful);
        let output = SharedBuf::default();
        plugin.start_with_io(io::Cursor::new(request(7, "forgetful")), output.clone());
        let responses = output.responses();
        assert_eq!(responses.len(), 1);
        assert_eq!(responses[0]["id"], json!(7));
        assert!(responses[0]["error"]["message"]
            .as_str()
            .unwrap()
            .contains("without answering"));
    }

    #[test]
    fn synchronous_handlers_answer_in_order() {
        let mut plugin = Plugin::new((), false);
        plugin.add_rpc_method("a", "", "", Echo("a"));
        plugin.add_rpc_method("b", "", "", Echo("b"));
        let input = [request(1, "a"), request(2, "b"), request(3, "a")].concat();
        let output = SharedBuf::default();
        plugin.start_with_io(io::Cursor::new(input), output.clone());
        let responses = output.responses();
        let got: Vec<_> = responses
            .iter()
            .map(|r| (r["id"].clone(), r["result"]["from"].clone()))
            .collect();
        assert_eq!(
            got,
            vec![
                (json!(1), json!("a")),
                (json!(2), json!("b")),
                (json!(3), json!("a"))
            ]
        );
    }
}
