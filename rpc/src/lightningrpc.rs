//! High-level interface to c-lightning RPC
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use clightningrpc_common::client;
use clightningrpc_common::errors::Error;

use crate::requests;
use crate::responses;
use crate::types::RouteItem;

/// Structure providing a high-level interface to the c-lightning daemon RPC
#[derive(Debug)]
pub struct LightningRPC {
    client: client::Client,
}

/// Optional arguments for pay() request
#[derive(Debug, Clone, Default)]
pub struct PayOptions<'f> {
    /// {msatoshi} (if and only if {bolt11} does not have amount)
    pub msatoshi: Option<u64>,
    /// {description} (required if {bolt11} uses description hash)
    pub description: Option<&'f str>,
    /// {riskfactor} (default 1.0)
    pub riskfactor: Option<f64>,
    /// {maxfeepercent} (default 0.5) the maximum acceptable fee as a percentage (e.g. 0.5 => 0.5%)
    pub maxfeepercent: Option<f64>,
    /// {exemptfee} (default 5000 msat) disables the maxfeepercent check for fees below the threshold
    pub exemptfee: Option<u64>,
    /// {retry_for} (default 60) the integer number of seconds before we stop retrying
    pub retry_for: Option<u64>,
    /// {maxdelay} (default 500) the maximum number of blocks we allow the funds to possibly get locked
    pub maxdelay: Option<u64>,
}

impl LightningRPC {
    /// Create a new connection from a UNIX socket path.
    ///
    /// # Arguments
    ///
    /// * `sockpath` - Path of UNIX socket to connect to, by default lightningd will create a
    /// socket named `.lightning/lightning-rpc` in the home directory of the user running
    /// lightningd.
    pub fn new<P: AsRef<Path>>(sockpath: P) -> LightningRPC {
        LightningRPC {
            client: client::Client::new(sockpath),
        }
    }

    /// Get reference to the low-level client connection
    pub fn client(&mut self) -> &mut client::Client {
        &mut self.client
    }

    /// Generic call function for RPC calls.
    pub fn call<T: Serialize, U: DeserializeOwned>(
        &self,
        method: &str,
        input: T,
    ) -> Result<U, Error> {
        self.client
            .send_request(method, input)
            .and_then(|res| res.into_result())
    }

    /// Show information about this node.
    pub fn getinfo(&self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {})
    }

    /// Return feerate estimates, either satoshi-per-kw ({style} perkw) or satoshi-per-kb ({style}
    /// perkb).
    pub fn feerates(&self, style: &str) -> Result<responses::FeeRates, Error> {
        self.call("feerates", requests::FeeRates { style: style })
    }

    /// Show node {id} (or all, if no {id}), in our local network view.
    pub fn listnodes(&self, id: Option<&str>) -> Result<responses::ListNodes, Error> {
        self.call("listnodes", requests::ListNodes { id })
    }

    /// Show channel {short_channel_id} (or all known channels, if no {short_channel_id}).
    pub fn listchannels(
        &self,
        short_channel_id: Option<&str>,
    ) -> Result<responses::ListChannels, Error> {
        self.call("listchannels", requests::ListChannels { short_channel_id })
    }

    /// List available commands, or give verbose help on one command.
    pub fn help(&self, command: Option<&str>) -> Result<responses::Help, Error> {
        self.call("help", requests::Help { command })
    }

    /// Show logs, with optional log {level} (info|unusual|debug|io).
    pub fn getlog(&self, level: Option<&str>) -> Result<responses::GetLog, Error> {
        self.call("getlog", requests::GetLog { level })
    }

    /// List all configuration options, or with [config], just that one.
    /// Because of the dynamic nature of the returned object, unlike the other methods, this
    /// returns a HashMap (from &str to Json) instead of a structure.
    pub fn listconfigs(&self, config: Option<&str>) -> Result<responses::ListConfigs, Error> {
        self.call("listconfigs", requests::ListConfigs { config })
    }

    /// Show current peers, if {level} is set, include {log}s.
    pub fn listpeers(
        &self,
        id: Option<&str>,
        level: Option<&str>,
    ) -> Result<responses::ListPeers, Error> {
        self.call("listpeers", requests::ListPeers { id, level })
    }

    /// Show invoice {label} (or all, if no {label)).
    pub fn listinvoices(
        &self,
        label: Option<&str>,
        invstring: Option<&str>,
        payment_hash: Option<&str>,
        offer_id: Option<&str>,
    ) -> Result<responses::ListInvoices, Error> {
        self.call(
            "listinvoices",
            requests::ListInvoices {
                label,
                invstring,
                payment_hash,
                offer_id,
            },
        )
    }

    /// Create an invoice for {msatoshi} with {label} and {description} with
    /// optional {expiry} seconds (default 1 hour).
    pub fn invoice(
        &self,
        msatoshi: u64,
        label: &str,
        description: &str,
        expiry: Option<u64>,
    ) -> Result<responses::Invoice, Error> {
        self.call(
            "invoice",
            requests::Invoice {
                msatoshi,
                label,
                description,
                expiry,
            },
        )
    }

    /// Create an invoice for {msatoshi} with {label} and {description} with
    /// optional {expiry} seconds (default 1 hour).
    pub fn delinvoice(&self, label: &str, status: &str) -> Result<responses::DelInvoice, Error> {
        self.call("delinvoice", requests::DelInvoice { label, status })
    }

    /// Delete all expired invoices that expired as of given {maxexpirytime} (a UNIX epoch time),
    /// or all expired invoices if not specified.
    pub fn delexpiredinvoice(
        &self,
        maxexpirytime: Option<u64>,
    ) -> Result<responses::DelExpiredInvoice, Error> {
        self.call(
            "delexpiredinvoice",
            requests::DelExpiredInvoice { maxexpirytime },
        )
    }

    /// Set up autoclean of expired invoices. Perform cleanup every {cycle_seconds} (default 3600),
    /// or disable autoclean if 0. Clean up expired invoices that have expired for {expired_by}
    /// seconds (default 86400).
    pub fn autocleaninvoice(
        &self,
        cycle_seconds: Option<u64>,
        expired_by: Option<u64>,
    ) -> Result<responses::AutoCleanInvoice, Error> {
        self.call(
            "autocleaninvoice",
            requests::AutoCleanInvoice {
                cycle_seconds,
                expired_by,
            },
        )
    }

    /// Wait for the next invoice to be paid, after {lastpay_index}.
    /// (if supplied)
    pub fn waitanyinvoice(
        &self,
        lastpay_index: Option<u64>,
    ) -> Result<responses::WaitAnyInvoice, Error> {
        self.call("waitanyinvoice", requests::WaitAnyInvoice { lastpay_index })
    }

    /// Wait for an incoming payment matching the invoice with {label}.
    pub fn waitinvoice(&self, label: &str) -> Result<responses::WaitInvoice, Error> {
        self.call("waitinvoice", requests::WaitInvoice { label })
    }

    /// Send a lightning payment.
    ///
    /// # Arguments
    ///
    /// * `bolt11` - A string that holds the payment information in bolt11 format.
    /// * `options` - Options for this payment. Use `Default::default()` to not pass any options.
    pub fn pay(&self, bolt11: &str, options: PayOptions) -> Result<responses::Pay, Error> {
        self.call(
            "pay",
            requests::Pay {
                bolt11: bolt11,
                msatoshi: options.msatoshi,
                description: options.description,
                riskfactor: options.riskfactor,
                maxfeepercent: options.maxfeepercent,
                exemptfee: options.exemptfee,
                retry_for: options.retry_for,
                maxdelay: options.maxdelay,
            },
        )
    }

    /// Send along {route} in return for preimage of {payment_hash}, with optional {description}.
    pub fn sendpay(
        &self,
        route: Vec<RouteItem>,
        payment_hash: &str,
        description: Option<&str>,
        msatoshi: Option<u64>,
    ) -> Result<responses::SendPay, Error> {
        self.call(
            "sendpay",
            requests::SendPay {
                route,
                payment_hash,
                description,
                msatoshi,
            },
        )
    }

    /// Wait for payment attempt on {payment_hash} to succeed or fail, but only up to {timeout} seconds.
    pub fn waitsendpay(
        &self,
        payment_hash: &str,
        timeout: u64,
    ) -> Result<responses::WaitSendPay, Error> {
        self.call(
            "waitsendpay",
            requests::WaitSendPay {
                payment_hash,
                timeout,
            },
        )
    }

    /// Show outgoing payments.
    pub fn listsendpays(
        &self,
        bolt11: Option<&str>,
        payment_hash: Option<&str>,
    ) -> Result<responses::ListSendPays, Error> {
        self.call(
            "listsendpays",
            requests::ListSendPays {
                bolt11,
                payment_hash,
            },
        )
    }

    /// Decode {bolt11}, using {description} if necessary.
    pub fn decodepay(
        &self,
        bolt11: &str,
        description: Option<&str>,
    ) -> Result<responses::DecodePay, Error> {
        self.call(
            "decodepay",
            requests::DecodePay {
                bolt11,
                description,
            },
        )
    }

    /// Show route to {id} for {msatoshi}, using {riskfactor} and optional {cltv} (default 9). If
    /// specified search from {fromid} otherwise use this node as source. Randomize the route with
    /// up to {fuzzpercent} (0.0 -> 100.0, default 5.0) using {seed} as an arbitrary-size string
    /// seed.
    pub fn getroute(
        &self,
        id: &str,
        msatoshi: u64,
        riskfactor: f64,
        cltv: Option<u64>,
        fromid: Option<&str>,
        fuzzpercent: Option<f64>,
        seed: Option<&str>,
    ) -> Result<responses::GetRoute, Error> {
        self.call(
            "getroute",
            requests::GetRoute {
                id,
                msatoshi,
                riskfactor,
                cltv,
                fromid,
                fuzzpercent,
                seed,
            },
        )
    }

    /// Connect to {id} at {host} (which can end in ':port' if not default). {id} can also be of
    /// the form id@host.
    pub fn connect(&self, id: &str, host: Option<&str>) -> Result<responses::Connect, Error> {
        self.call("connect", requests::Connect { id, host })
    }

    /// Disconnect from peer with {peer_id}.
    pub fn disconnect(&self, id: &str) -> Result<responses::Disconnect, Error> {
        self.call("disconnect", requests::Disconnect { id })
    }

    /// Fund a new channel with another lightning node.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of node to fund a channel to
    /// * `amount` - either `AmountOrAll::Amount(n)` for a given amount in satoshi units, or
    /// `AmountOrAll::All` to spend all available funds
    /// * `feerate` - optional feerate to use for Bitcoin transaction
    pub fn fundchannel(
        &self,
        id: &str,
        amount: requests::AmountOrAll,
        feerate: Option<u64>,
    ) -> Result<responses::FundChannel, Error> {
        self.call(
            "fundchannel",
            requests::FundChannel {
                id,
                amount,
                feerate,
            },
        )
    }

    /// Close the channel with {id} (either peer ID, channel ID, or short channel ID). If {force}
    /// (default false) is true, force a unilateral close after {timeout} seconds (default 30),
    /// otherwise just schedule a mutual close later and fail after timing out.
    pub fn close(
        &self,
        id: &str,
        force: Option<bool>,
        timeout: Option<u64>,
    ) -> Result<responses::Close, Error> {
        self.call("close", requests::Close { id, force, timeout })
    }

    /// Send {peerid} a ping of length {len} (default 128) asking for {pongbytes} (default 128).
    pub fn ping(
        &self,
        id: &str,
        len: Option<u64>,
        pongbytes: Option<u64>,
    ) -> Result<responses::Ping, Error> {
        self.call("ping", requests::Ping { id, len, pongbytes })
    }

    /// Show available funds from the internal wallet.
    pub fn listfunds(&self) -> Result<responses::ListFunds, Error> {
        self.call("listfunds", requests::ListFunds {})
    }

    /// Send to destination address via Bitcoin transaction.
    ///
    /// # Arguments
    ///
    /// * `destination` - Bitcoin address to send to
    /// * `amount` - either `AmountOrAll::Amount(n)` for a given amount in satoshi units, or
    /// `AmountOrAll::All` to spend all available funds
    /// * `feerate` - optional feerate to use for Bitcoin transaction
    pub fn withdraw(
        &self,
        destination: &str,
        amount: requests::AmountOrAll,
        feerate: Option<u64>,
    ) -> Result<responses::Withdraw, Error> {
        self.call(
            "withdraw",
            requests::Withdraw {
                destination,
                amount,
                feerate,
            },
        )
    }

    /// Get a new {bech32, p2sh-segwit} address to fund a channel (default is bech32).
    pub fn newaddr(&self, addresstype: Option<&str>) -> Result<responses::NewAddr, Error> {
        self.call("newaddr", requests::NewAddr { addresstype })
    }

    /// Shut down the lightningd process.
    pub fn stop(&self) -> Result<responses::Stop, Error> {
        self.call("stop", requests::Stop {})
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn set_timeout() {
        use crate::LightningRPC;
        use std::time::Duration;

        let mut lightning = LightningRPC::new("/test");
        lightning
            .client()
            .set_timeout(Some(Duration::from_millis(100)));
    }
}
