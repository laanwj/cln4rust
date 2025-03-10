//! Integration testing library for core lightning
use std::sync::Arc;

use port_selector as port;
use tempfile::TempDir;

use clightningrpc::LightningRPC;

use crate::btc::BtcNode;
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
                log::info!("core lightning home {path}");
                check_dir_or_make_if_missing(path.clone()).await.unwrap();
                let mut command = Command::new("lightningd");
                command
                    .args(&args_tok)
                    .arg(format!("--addr=127.0.0.1:{}", $port))
                    .arg(format!("--bind-addr=127.0.0.1:{}", $port + 1))
                    .arg(format!("--lightning-dir={path}"))
                    .arg("--developer")
                    .arg("--dev-fast-gossip")
                    .arg("--funding-confirms=1")
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
    root_path: Arc<TempDir>,
    cln_dir: String,
    bitcoin: Arc<BtcNode>,
    process: Vec<tokio::process::Child>,
}

impl Drop for Node {
    fn drop(&mut self) {
        let _ = self.rpc().stop();
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

        let result = std::fs::remove_dir_all(self.root_path.path());
        log::debug!(target: "cln", "clean up function {:?}", result);
    }
}

impl Node {
    pub async fn tmp(network: &str) -> anyhow::Result<Self> {
        Self::with_params("", network).await
    }

    pub async fn with_params(params: &str, network: &str) -> anyhow::Result<Self> {
        let btc = BtcNode::tmp(network).await?;
        let btc = Arc::new(btc);
        Self::with_btc_and_params(btc.clone(), params, network).await
    }

    pub async fn with_btc_and_params(
        btc: Arc<BtcNode>,
        params: &str,
        network: &str,
    ) -> anyhow::Result<Self> {
        let dir = tempfile::tempdir()?;

        let cln_path = format!("{}/.lightning", dir.path().to_str().unwrap());
        let port = port::random_free_port().unwrap();
        let process = macros::lightningd!(
            dir,
            port,
            "--network={} --log-level=debug --dev-bitcoind-poll=1 --bitcoin-rpcuser={} --bitcoin-rpcpassword={} --bitcoin-rpcport={} {}",
            network,
            btc.user,
            btc.pass,
            btc.port,
            params,
        )?;

        let rpc = LightningRPC::new(
            dir.path()
                .join(format!(".lightning/{}", network))
                .join("lightning-rpc"),
        );
        let rpc = Arc::new(rpc);
        wait_for!(async { rpc.getinfo() });

        Ok(Self {
            inner: rpc,
            root_path: dir.into(),
            bitcoin: btc,
            port,
            process: vec![process],
            cln_dir: cln_path,
        })
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
        self.inner.stop()?;
        for process in self.process.iter_mut() {
            process.kill().await?;
            let _ = process.wait().await?;
            log::debug!("killing process");
        }
        self.bitcoin.stop().await?;
        Ok(())
    }
}
