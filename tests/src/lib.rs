use std::sync::Once;

#[cfg(all(test, not(feature = "async")))]
mod test_plugin;
#[cfg(all(test, not(feature = "async")))]
mod test_rpc;

#[cfg(all(test, feature = "async"))]
mod test_rpc_async;

static INIT: Once = Once::new();

fn init() {
    // ignore error
    INIT.call_once(|| {
        env_logger::init();
    });
}

#[macro_export]
macro_rules! async_run {
    ($rt:expr, $expr:expr) => {{
        $rt.block_on($expr)
    }};
    ($expr:expr) => {{
        let rt = tokio::runtime::Runtime::new().unwrap();
        async_run!(rt, $expr)
    }};
}

pub mod fixtures {

    use clightning_testing::cln;
    use rstest::*;

    use super::{async_run, init};

    #[fixture]
    pub fn lightningd() -> cln::Node {
        init();
        let pwd = std::env::var("PWD").unwrap();
        log::info!("pwd: {}", pwd);
        let cln = async_run!(cln::Node::with_params(&format!("--developer --allow-deprecated-apis=true --plugin={pwd}/target/debug/examples/foo_plugin --plugin={pwd}/target/debug/examples/macros_ex"))).unwrap();
        cln
    }

    #[fixture]
    pub fn lightningd_second() -> cln::Node {
        init();
        let cln = async_run!(cln::Node::with_params(
            "--developer --allow-deprecated-apis=true",
        ))
        .unwrap();
        cln
    }
}
