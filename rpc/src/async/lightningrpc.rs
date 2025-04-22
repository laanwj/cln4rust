//! High-level async interface to c-lightning RPC
use std::path::Path;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::types::RouteItem;
use clightningrpc_common::r#async::Client;

use crate::errors::Error;
use crate::lightningrpc::PayOptions;
use crate::requests;
use crate::responses;

/// Structure providing a high-level async interface to the c-lightning daemon RPC
#[derive(Debug)]
pub struct LightningRPC {
    client: Client,
}

impl LightningRPC {
    /// Create a new async connection from a UNIX socket path.
    ///
    /// # Arguments
    ///
    /// * `sockpath` - Path of UNIX socket to connect to, by default lightningd will create a
    /// socket named `.lightning/lightning-rpc` in the home directory of the user running
    /// lightningd.
    pub fn new<P: AsRef<Path>>(sockpath: P) -> LightningRPC {
        LightningRPC {
            client: Client::new(sockpath),
        }
    }

    /// Set an optional timeout for requests
    pub fn set_timeout(&mut self, timeout: Option<Duration>) {
        self.client.set_timeout(timeout);
    }

    pub fn client(&mut self) -> &mut Client {
        &mut self.client
    }

    /// Generic call function for async RPC calls.
    pub async fn call<T: Serialize, U: DeserializeOwned>(
        &self,
        method: &str,
        input: T,
    ) -> Result<U, Error> {
        let response = self.client.send_request(method, input).await?;
        response.into_result()
    }

    /// Show information about this node.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn getinfo(&self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {}).await
    }

    /// Return feerate estimates, either satoshi-per-kw ({style} perkw) or satoshi-per-kb ({style}
    /// perkb).
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn feerates(&self, style: &str) -> Result<responses::FeeRates, Error> {
        self.call("feerates", requests::FeeRates { style }).await
    }

    /// Show node {id} (or all, if no {id}), in our local network view.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn listnodes(&self, id: Option<&str>) -> Result<responses::ListNodes, Error> {
        self.call("listnodes", requests::ListNodes { id }).await
    }

    /// Show channel {short_channel_id} (or all known channels, if no {short_channel_id}).
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn listchannels(
        &self,
        short_channel_id: Option<&str>,
        source: Option<&str>,
        destination: Option<&str>,
    ) -> Result<responses::ListChannels, Error> {
        self.call(
            "listchannels",
            requests::ListChannels {
                short_channel_id,
                source,
                destination,
            },
        )
        .await
    }

    /// List available commands, or give verbose help on one command.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn help(&self, command: Option<&str>) -> Result<responses::Help, Error> {
        self.call("help", requests::Help { command }).await
    }

    /// Show logs, with optional log {level} (info|unusual|debug|io).
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn getlog(&self, level: Option<&str>) -> Result<responses::GetLog, Error> {
        self.call("getlog", requests::GetLog { level }).await
    }

    /// List all configuration options, or with [config], just that one.
    /// Because of the dynamic nature of the returned object, unlike the other methods, this
    /// returns a HashMap (from &str to Json) instead of a structure.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn listconfigs(&self, config: Option<&str>) -> Result<responses::ListConfigs, Error> {
        self.call("listconfigs", requests::ListConfigs { config })
            .await
    }

    /// Show current peers, if {level} is set, include {log}s.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn listpeers(
        &self,
        id: Option<&str>,
        level: Option<&str>,
    ) -> Result<responses::ListPeers, Error> {
        self.call("listpeers", requests::ListPeers { id, level })
            .await
    }

    /// Show invoice {label} (or all, if no {label)).
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn listinvoices(
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
        .await
    }

    /// Create an invoice for {msatoshi} with {label} and {description} with
    /// optional {expiry} seconds (default 1 hour).
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn invoice(
        &self,
        amount_msat: Option<u64>,
        label: &str,
        description: &str,
        preimage: Option<&str>,
        expiry: Option<u64>,
        deschashonly: Option<bool>,
    ) -> Result<responses::Invoice, Error> {
        match amount_msat {
            None => {
                self.call(
                    "invoice",
                    requests::AnyInvoice {
                        amount_msat: "any",
                        label,
                        description,
                        preimage,
                        expiry,
                        deschashonly,
                    },
                )
                .await
            }
            Some(amount_msat) => {
                self.call(
                    "invoice",
                    requests::Invoice {
                        amount_msat,
                        label,
                        description,
                        preimage,
                        expiry,
                        deschashonly,
                    },
                )
                .await
            }
        }
    }

    /// Lowlevel command to sign and create invoice {invstring}, resolved with {preimage},
    /// using unique {label}
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn createinvoice(
        &self,
        invstring: &str,
        label: &str,
        preimage: &str,
    ) -> Result<responses::Invoice, Error> {
        self.call(
            "createinvoice",
            requests::CreateInvoice {
                invstring,
                label,
                preimage,
            },
        )
        .await
    }

    /// Delete unpaid invoice {label} with {status}
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn delinvoice(
        &self,
        label: &str,
        status: &str,
    ) -> Result<responses::DelInvoice, Error> {
        self.call("delinvoice", requests::DelInvoice { label, status })
            .await
    }

    /// Delete all expired invoices that expired as of given {maxexpirytime} (a UNIX epoch time),
    /// or all expired invoices if not specified.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn delexpiredinvoice(
        &self,
        maxexpirytime: Option<u64>,
    ) -> Result<responses::DelExpiredInvoice, Error> {
        self.call(
            "delexpiredinvoice",
            requests::DelExpiredInvoice { maxexpirytime },
        )
        .await
    }

    /// Set up autoclean of expired invoices. Perform cleanup every {cycle_seconds} (default 3600),
    /// or disable autoclean if 0. Clean up expired invoices that have expired for {expired_by}
    /// seconds (default 86400).
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn autocleaninvoice(
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
        .await
    }

    /// Wait for the next invoice to be paid, after {lastpay_index}.
    /// (if supplied)
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn waitanyinvoice(
        &self,
        lastpay_index: Option<u64>,
    ) -> Result<responses::WaitAnyInvoice, Error> {
        self.call("waitanyinvoice", requests::WaitAnyInvoice { lastpay_index })
            .await
    }

    /// Wait for an incoming payment matching the invoice with {label}.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn waitinvoice(&self, label: &str) -> Result<responses::WaitInvoice, Error> {
        self.call("waitinvoice", requests::WaitInvoice { label })
            .await
    }

    /// Send a lightning payment.
    ///
    /// # Arguments
    ///
    /// * `bolt11` - A string that holds the payment information in bolt11 format.
    /// * `options` - Options for this payment. Use `Default::default()` to not pass any options.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn pay(
        &self,
        bolt11: &str,
        options: PayOptions<'_>,
    ) -> Result<responses::Pay, Error> {
        self.call(
            "pay",
            requests::Pay {
                bolt11,
                msatoshi: options.msatoshi,
                description: options.description,
                riskfactor: options.riskfactor,
                maxfeepercent: options.maxfeepercent,
                exemptfee: options.exemptfee,
                retry_for: options.retry_for,
                maxdelay: options.maxdelay,
            },
        )
        .await
    }

    /// Send along {route} in return for preimage of {payment_hash}, with optional {description}.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn sendpay(
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
        .await
    }

    /// Wait for payment attempt on {payment_hash} to succeed or fail, but only up to {timeout} seconds.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn waitsendpay(
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
        .await
    }

    /// Show outgoing payments.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn listsendpays(
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
        .await
    }

    /// Decode {bolt11}, using {description} if necessary.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn decodepay(
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
        .await
    }

    /// Show route to {id} for {msatoshi}, using {riskfactor} and optional {cltv} (default 9). If
    /// specified search from {fromid} otherwise use this node as source. Randomize the route with
    /// up to {fuzzpercent} (0.0 -> 100.0, default 5.0) using {seed} as an arbitrary-size string
    /// seed.
    #[allow(clippy::too_many_arguments)]
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn getroute(
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
        .await
    }

    /// Connect to {id} at {host} (which can end in ':port' if not default). {id} can also be of
    /// the form id@host.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn connect(&self, id: &str, host: Option<&str>) -> Result<responses::Connect, Error> {
        self.call("connect", requests::Connect { id, host }).await
    }

    /// Disconnect from peer with {peer_id}.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn disconnect(&self, id: &str) -> Result<responses::Disconnect, Error> {
        self.call("disconnect", requests::Disconnect { id }).await
    }

    /// Fund a new channel with another lightning node.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of node to fund a channel to
    /// * `amount` - either `AmountOrAll::Amount(n)` for a given amount in satoshi units, or
    /// `AmountOrAll::All` to spend all available funds
    /// * `feerate` - optional feerate to use for Bitcoin transaction
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn fundchannel(
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
        .await
    }

    /// Close the channel with {id} (either peer ID, channel ID, or short channel ID). If {force}
    /// (default false) is true, force a unilateral close after {timeout} seconds (default 30),
    /// otherwise just schedule a mutual close later and fail after timing out.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn close(
        &self,
        id: &str,
        force: Option<bool>,
        timeout: Option<u64>,
    ) -> Result<responses::Close, Error> {
        self.call("close", requests::Close { id, force, timeout })
            .await
    }

    /// Send {peerid} a ping of length {len} (default 128) asking for {pongbytes} (default 128).
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn ping(
        &self,
        id: &str,
        len: Option<u64>,
        pongbytes: Option<u64>,
    ) -> Result<responses::Ping, Error> {
        self.call("ping", requests::Ping { id, len, pongbytes })
            .await
    }

    /// Show available funds from the internal wallet.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn listfunds(&self) -> Result<responses::ListFunds, Error> {
        self.call("listfunds", requests::ListFunds {}).await
    }

    /// Send to destination address via Bitcoin transaction.
    ///
    /// # Arguments
    ///
    /// * `destination` - Bitcoin address to send to
    /// * `amount` - either `AmountOrAll::Amount(n)` for a given amount in satoshi units, or
    /// `AmountOrAll::All` to spend all available funds
    /// * `feerate` - optional feerate to use for Bitcoin transaction
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn withdraw(
        &self,
        destination: &str,
        satoshi: requests::AmountOrAll,
        feerate: Option<u64>,
        minconf: Option<u32>,
    ) -> Result<responses::Withdraw, Error> {
        self.call(
            "withdraw",
            requests::Withdraw {
                destination,
                satoshi,
                feerate,
                minconf,
            },
        )
        .await
    }

    /// Get a new {bech32, p2sh-segwit} address to fund a channel (default is bech32).
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn newaddr(&self, addresstype: Option<&str>) -> Result<responses::NewAddr, Error> {
        self.call("newaddr", requests::NewAddr { addresstype })
            .await
    }

    /// Shut down the lightningd process.
    #[deprecated(
        since = "0.1.0",
        note = "Core Lightning API changes frequently, making strongly typed methods hard to maintain. Use the generic `call` method with serde_json until a compiler is shipped or the API stabilizes."
    )]
    pub async fn stop(&self) -> Result<responses::Stop, Error> {
        self.call("stop", requests::Stop {}).await
    }
}
