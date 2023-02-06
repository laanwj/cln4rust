use bitcoin::{Address, Amount, Txid};
use bitcoincore_rpc::{Client, RpcApi};
use bitcoincore_rpc_json::EstimateMode;
use serde::Deserialize;

// Get 50 BTC to the node wallet
// In Regtest Mode a block must have 100 confirmations before that reward can be spent,
// so we generate 101 blocks to get access to the coinbase transaction from block #1.
// https://developer.bitcoin.org/examples/testing.html#regtest-mode
pub fn fund_wallet(client: &Client) {
    let mature_blocks = 100;
    mine_blocks(client, mature_blocks + 1);
}

// Mine some blocks to self wallet
pub fn mine_blocks(client: &Client, amount: u64) {
    let address = new_address(client);
    client
        .generate_to_address(amount, &address)
        .expect("mined blocks");
}

// Get a fresh address from the node
pub fn new_address(client: &Client) -> Address {
    client.get_new_address(None, None).expect("new address")
}

// Send mined btc to given address
pub fn send_funds(client: &Client, address: &Address, amount: Amount) -> Txid {
    client
        .send_to_address(address, amount, None, None, None, Some(true), None, None)
        .expect("funds sent")
}

// Abandones given transaction from wallet (and mempool)
pub fn bumpfee(
    client: &Client,
    txid: &Txid,
    confirmation_target: Option<u32>,
    total_fee: Option<Amount>,
    replaceable: Option<bool>,
    estimate_mode: Option<EstimateMode>,
) -> bitcoincore_rpc::Result<BumpFeeResult> {
    let mut args = [
        serde_json::to_value(txid.to_string()).unwrap(),
        opt_into_json(confirmation_target),
        opt_into_json(total_fee.map(|v| v.as_btc())),
        opt_into_json(replaceable),
        opt_into_json(estimate_mode),
    ];
    client.call(
        "bumpfee",
        handle_defaults(
            &mut args,
            &["".into(), 6.into(), null(), true.into(), null()],
        ),
    )
}

#[derive(Deserialize)]
pub struct BumpFeeResult {
    pub txid: Txid,
    pub origfee: f64,
    pub fee: f64,
}

/// Shorthand for converting an Option into an Option<serde_json::Value>.
fn opt_into_json<T>(opt: Option<T>) -> serde_json::Value
where
    T: serde::ser::Serialize,
{
    match opt {
        Some(val) => serde_json::to_value(val).unwrap(),
        None => serde_json::Value::Null,
    }
}

/// Shorthand for `serde_json::Value::Null`.
fn null() -> serde_json::Value {
    serde_json::Value::Null
}

/// Handle default values in the argument list
///
/// Substitute `Value::Null`s with corresponding values from `defaults` table,
/// except when they are trailing, in which case just skip them altogether
/// in returned list.
///
/// Note, that `defaults` corresponds to the last elements of `args`.
///
/// ```norust
/// arg1 arg2 arg3 arg4
///           def1 def2
/// ```
///
/// Elements of `args` without corresponding `defaults` value, won't
/// be substituted, because they are required.
fn handle_defaults<'a, 'b>(
    args: &'a mut [serde_json::Value],
    defaults: &'b [serde_json::Value],
) -> &'a [serde_json::Value] {
    assert!(args.len() >= defaults.len());

    // Pass over the optional arguments in backwards order, filling in defaults after the first
    // non-null optional argument has been observed.
    let mut first_non_null_optional_idx = None;
    for i in 0..defaults.len() {
        let args_i = args.len() - 1 - i;
        let defaults_i = defaults.len() - 1 - i;
        if args[args_i] == serde_json::Value::Null {
            if first_non_null_optional_idx.is_some() {
                if defaults[defaults_i] == serde_json::Value::Null {
                    panic!("Missing `default` for argument idx {}", args_i);
                }
                args[args_i] = defaults[defaults_i].clone();
            }
        } else if first_non_null_optional_idx.is_none() {
            first_non_null_optional_idx = Some(args_i);
        }
    }

    let required_num = args.len() - defaults.len();

    if let Some(i) = first_non_null_optional_idx {
        &args[..i + 1]
    } else {
        &args[..required_num]
    }
}
