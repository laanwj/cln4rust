pub mod cln;

pub mod prelude {
    pub use port_selector as port;
    pub use tempfile;

    pub use clightningrpc;
    pub use corepc_node as btc;
    pub use corepc_node::client;
    pub use corepc_node::client::bitcoin;

    pub use crate::check_dir_or_make_if_missing;
    pub use crate::macros::*;
}

pub mod macros {
    #[macro_export]
    macro_rules! wait_for {
        ($callback:expr, $timeout:expr) => {
            use log;
            use tokio::time::{sleep, Duration};

            for wait in 0..$timeout {
                if let Err(_) = $callback.await {
                    sleep(Duration::from_millis(wait)).await;
                    continue;
                }
                log::info!("callback completed in {wait} milliseconds");
                break;
            }
        };
        ($callback:expr) => {
            use crate::DEFAULT_TIMEOUT;

            $crate::wait_for!($callback, DEFAULT_TIMEOUT);
        };
    }

    pub use wait_for;
}

static DEFAULT_TIMEOUT: u64 = 100;

pub async fn check_dir_or_make_if_missing(path: String) -> anyhow::Result<()> {
    use std::path::Path;
    use tokio::fs::create_dir;

    if !Path::exists(Path::new(&path.to_owned())) {
        create_dir(path.clone()).await?;
        log::debug!("created dir {path}");
    }
    Ok(())
}
