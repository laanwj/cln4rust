//! Bitcoin Testing framework.
//!
use std::cell::RefCell;

use bitcoincore_rpc::{Auth, Client, RpcApi};
use port::Port;
use port_selector as port;
use tempfile::TempDir;

pub mod macros {
    #[macro_export]
    macro_rules! bitcoind {
        ($dir:expr, $port:expr, $opt_args:expr) => {
            async {
                use std::process::Stdio;

                use log;
                use tokio::process::Command;

                let opt_args = format!($opt_args);
                let args = opt_args.trim();
                let args_tok: Vec<&str> = args.split(" ").collect();
                log::debug!("additional args: {:?}", args_tok);
                let mut command = Command::new("bitcoind");
                command
                    .args(&args_tok)
                    .arg(format!("-port={}", $port + 1))
                    .arg(format!("-rpcport={}", $port))
                    .arg(format!("-datadir={}", $dir.path().to_str().unwrap()))
                    .stdout(Stdio::null())
                    .spawn()
            }
            .await
        };
        ($dir:expr, $port:expr) => {
            $crate::bitcoind!($dir, $port, "")
        };
    }

    pub use bitcoind;
}

pub struct BtcNode {
    inner: Client,
    pub user: String,
    pub pass: String,
    pub port: Port,
    root_path: TempDir,
    process: RefCell<Vec<tokio::process::Child>>,
}

impl Drop for BtcNode {
    fn drop(&mut self) {
        for process in self.process.borrow().iter() {
            let Some(child) = process.id() else {
                continue;
            };
            let Ok(mut kill) = std::process::Command::new("kill")
                .args(["-s", "SIGKILL", &child.to_string()])
                .spawn()
            else {
                continue;
            };
            let _ = kill.wait();
        }

        let result = std::fs::remove_dir_all(self.root_path.path());
        log::debug!(target: "btc", "clean up function {:?}", result);
    }
}

impl BtcNode {
    pub async fn tmp(network: &str) -> anyhow::Result<Self> {
        let dir = tempfile::tempdir()?;
        let user = "crab".to_owned();
        let pass = "crab".to_owned();
        let port = port::random_free_port().unwrap();
        let process = macros::bitcoind!(
            dir,
            port,
            "-server -{network} -rpcuser={user} -rpcpassword={pass}"
        )?;
        let rpc = Client::new(
            &format!("http://localhost:{port}"),
            Auth::UserPass(user.clone(), pass.clone()),
        )?;
        let bg_process = vec![process];
        Ok(Self {
            inner: rpc,
            root_path: dir,
            user,
            pass,
            port,
            process: bg_process.into(),
        })
    }

    pub fn rpc(&self) -> &Client {
        &self.inner
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        log::info!("stop bitcoin node");
        self.inner.stop()?;
        for process in self.process.borrow_mut().iter_mut() {
            process.kill().await?;
            let _ = process.wait().await?;
            log::debug!("process killed");
        }
        Ok(())
    }
}
