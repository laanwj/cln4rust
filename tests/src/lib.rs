use std::sync::Once;

#[cfg(test)]
mod test_plugin;
#[cfg(test)]
mod test_rpc;

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
        
        async_run!(cln::Node::with_params(&format!("--developer --plugin={pwd}/target/debug/examples/foo_plugin --plugin={pwd}/target/debug/examples/macros_ex"), "regtest")).unwrap()
    }

    #[fixture]
    pub fn lightningd_second() -> cln::Node {
        init();
        
        async_run!(cln::Node::with_params("--developer", "regtest")).unwrap()
    }
}
