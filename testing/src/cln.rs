//! Integration testing library for core lightning
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;

use port_selector as port;
use tempfile::TempDir;
use tokio::fs;

#[cfg(feature = "async")]
use clightningrpc::r#async::LightningRPC;
#[cfg(not(feature = "async"))]
use clightningrpc::LightningRPC;
use corepc_node::{Conf, Node as BtcNode};

use crate::prelude::*;

pub mod macros {
    #[macro_export]
    macro_rules! lightningd {
        ($dir:expr, $port:expr, $($opt_args:tt)*) => {
            async {
                use tokio::process::Command;

                let opt_args = format!($($opt_args)*);
                let args = opt_args.trim();
                let args_tok: Vec<&str> = args.split(" ").collect();

                let path = format!("{}/.lightning", $dir.path().to_str().unwrap());

                // Create and set the write permissions for the lightning directory
                fs::create_dir_all(path.clone()).await.unwrap();
                fs::set_permissions(path.clone(), std::fs::Permissions::from_mode(0o755)).await.unwrap();

                log::info!("core lightning home {path}");
                check_dir_or_make_if_missing(path.clone()).await.unwrap();
                let mut command = Command::new("lightningd");
                command
                    .args(&args_tok)
                    .arg(format!("--bind-addr=127.0.0.1:{}", $port))
                    .arg(format!("--lightning-dir={path}"))
                    .arg("--developer")
                    .arg("--dev-fast-gossip")
                    .arg(format!("--log-file={path}/log.log"))
                    .stdout(std::process::Stdio::null())
                    .spawn()
            }.await
        };
        ($dir:expr, $port:expr) => {
            $crate::lightningd!($dir, $port, "")
        };
    }

    pub use lightningd;
}

pub struct Node {
    inner: Arc<LightningRPC>,
    pub port: u16,
    cln_dir: String,
    // This is unused, but it's used to keep the directory alive,
    // otherwise when the main reference to the node is dropped,
    // the directory is deleted.
    root_path: Arc<TempDir>,
    bitcoin: Arc<BtcNode>,
    process: Vec<tokio::process::Child>,
}

impl Drop for Node {
    fn drop(&mut self) {
        for process in self.process.iter() {
            let Some(child) = process.id() else {
                continue;
            };
            // Read the content of the logs and print on the stdout
            let _ = self.print_logs();
            let Ok(mut kill) = std::process::Command::new("kill")
                .args(["-s", "SIGKILL", &child.to_string()])
                .spawn()
            else {
                continue;
            };
            let _ = kill.wait();
        }
    }
}

impl Node {
    pub async fn tmp() -> anyhow::Result<Self> {
        let mut conf = Conf::default();
        conf.wallet = None;
        let conf = Arc::new(conf);
        Self::with_conf(conf).await
    }

    pub async fn with_conf(conf: Arc<Conf<'static>>) -> anyhow::Result<Self> {
        let conf_clone = conf.clone();
        let btc = tokio::task::spawn_blocking(move || {
            if let Ok(exec_path) = corepc_node::exe_path() {
                let btc = BtcNode::with_conf(exec_path, conf_clone.as_ref())?;
                Ok(btc)
            } else {
                anyhow::bail!("corepc-node exec path not found");
            }
        })
        .await??;
        let btc = Arc::new(btc);
        Self::with_btc_and_params(btc.clone(), None).await
    }

    pub async fn with_btc_and_params(
        btc: Arc<BtcNode>,
        params: Option<String>,
    ) -> anyhow::Result<Self> {
        let dir = tempfile::tempdir()?;

        let cln_path = format!("{}/.lightning", dir.path().to_str().unwrap());
        let port = port::random_free_port().unwrap();

        let addr = btc.params.rpc_socket;
        let cookie_user = btc
            .params
            .get_cookie_values()?
            .ok_or(anyhow::anyhow!("cookie not found"))?;
        let process = macros::lightningd!(
            dir,
            port,
            "--network=regtest --log-level=debug --dev-bitcoind-poll=1 --bitcoin-rpcuser={} --bitcoin-rpcpassword={} --bitcoin-rpcport={} {}",
            cookie_user.user,
            cookie_user.password,
            addr.port(),
            params.unwrap_or_default(),
        )?;

        let rpc_path = dir.path().join(".lightning/regtest").join("lightning-rpc");
        log::info!("rpc_path: {}", rpc_path.to_str().unwrap());
        let rpc = LightningRPC::new(rpc_path);
        let rpc = Arc::new(rpc);

        #[cfg(feature = "async")]
        wait_for!(async { rpc.getinfo().await });
        #[cfg(not(feature = "async"))]
        wait_for!(async { rpc.getinfo() });

        log::info!("rpc is ready");

        let node = Self {
            inner: rpc,
            root_path: Arc::new(dir),
            bitcoin: btc,
            port,
            process: vec![process],
            cln_dir: cln_path,
        };
        log::info!("logs: {}", node.logs().unwrap());
        Ok(node)
    }

    pub async fn with_params(params: &str) -> anyhow::Result<Self> {
        let mut conf = Conf::default();
        conf.wallet = None;

        let conf = Arc::new(conf);
        let btc = tokio::task::spawn_blocking(move || {
            if let Ok(exec_path) = corepc_node::exe_path() {
                let btc = BtcNode::with_conf(exec_path, conf.as_ref())?;
                Ok(btc)
            } else {
                anyhow::bail!("bitcoind exec path not found");
            }
        })
        .await??;
        let btc = Arc::new(btc);
        Self::with_btc_and_params(btc, Some(params.to_string())).await
    }
    pub fn rpc(&self) -> Arc<LightningRPC> {
        self.inner.clone()
    }

    pub fn logs(&self) -> anyhow::Result<String> {
        let content = std::fs::read_to_string(format!("{}/log.log", self.cln_dir))?;
        Ok(content)
    }

    pub fn print_logs(&self) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(format!("{}/log.log", self.cln_dir))?;
        println!("{content}");
        Ok(())
    }

    pub fn btc(&self) -> Arc<BtcNode> {
        self.bitcoin.clone()
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("stop lightning node");

        #[cfg(feature = "async")]
        self.inner.as_ref().stop().await?;
        #[cfg(not(feature = "async"))]
        self.inner.as_ref().stop()?;

        for process in self.process.iter_mut() {
            process.kill().await?;
            let _ = process.wait().await?;
            log::debug!("killing process");
        }
        Ok(())
    }
}
