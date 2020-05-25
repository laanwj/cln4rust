//! High-level async interface to c-lightning RPC
use futures::io::{AsyncRead, AsyncWrite};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::aio::client;
use crate::common;
use crate::error::Error;
use crate::requests;
use crate::responses;
use crate::lightningrpc::PayOptions;

/// Structure providing a high-level interface to the c-lightning daemon RPC
#[derive(Debug)]
pub struct LightningRPC<U: AsyncRead + AsyncWrite> {
    client: client::Client<U>,
}

impl<U> LightningRPC<U>
where
    U: AsyncRead + AsyncWrite + Unpin,
{
    /// Create a new connection from a UNIX socket stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - TODO
    pub fn new(stream: U) -> Self
    {
        LightningRPC {
            client: client::Client::new(stream),
        }
    }

    /// Get reference to the low-level client connection
    pub fn client(&mut self) -> &mut client::Client<U> {
        &mut self.client
    }

    /// Generic call function for RPC calls.
    async fn call<T: Serialize, D: DeserializeOwned>(&mut self, method: &str, input: T) -> Result<D, Error> {
        self.client
            .send_request(method, input)
            .await
            .and_then(|res| res.into_result())
    }

    /// Show information about this node.
    pub async fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {}).await
    }

    /// Return feerate estimates, either satoshi-per-kw ({style} perkw) or satoshi-per-kb ({style}
    /// perkb).
    pub async fn feerates(&mut self, style: &str) -> Result<responses::FeeRates, Error> {
        self.call("feerates", requests::FeeRates { style: style }).await
    }

    /// Show node {id} (or all, if no {id}), in our local network view.
    pub async fn listnodes(&mut self, id: Option<&str>) -> Result<responses::ListNodes, Error> {
        self.call("listnodes", requests::ListNodes { id }).await
    }

    /// Show channel {short_channel_id} (or all known channels, if no {short_channel_id}).
    pub async fn listchannels(
        &mut self,
        short_channel_id: Option<&str>,
    ) -> Result<responses::ListChannels, Error> {
        self.call("listchannels", requests::ListChannels { short_channel_id }).await
    }

    /// List available commands, or give verbose help on one command.
    pub async fn help(&mut self, command: Option<&str>) -> Result<responses::Help, Error> {
        self.call("help", requests::Help { command }).await
    }

    /// Show logs, with optional log {level} (info|unusual|debug|io).
    pub async fn getlog(&mut self, level: Option<&str>) -> Result<responses::GetLog, Error> {
        self.call("getlog", requests::GetLog { level }).await
    }

    /// List all configuration options, or with [config], just that one.
    /// Because of the dynamic nature of the returned object, unlike the other methods, this
    /// returns a HashMap (from &str to Json) instead of a structure.
    pub async fn listconfigs(&mut self, config: Option<&str>) -> Result<responses::ListConfigs, Error> {
        self.call("listconfigs", requests::ListConfigs { config }).await
    }

    /// Show current peers, if {level} is set, include {log}s.
    pub async fn listpeers(
        &mut self,
        id: Option<&str>,
        level: Option<&str>,
    ) -> Result<responses::ListPeers, Error> {
        self.call("listpeers", requests::ListPeers { id, level }).await
    }

    /// Show invoice {label} (or all, if no {label)).
    pub async fn listinvoices(&mut self, label: Option<&str>) -> Result<responses::ListInvoices, Error> {
        self.call("listinvoices", requests::ListInvoices { label }).await
    }

    /// Create an invoice for {msatoshi} with {label} and {description} with
    /// optional {expiry} seconds (default 1 hour).
    pub async fn invoice(
        &mut self,
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
        ).await
    }

    /// Create an invoice for {msatoshi} with {label} and {description} with
    /// optional {expiry} seconds (default 1 hour).
    pub async fn delinvoice(&mut self, label: &str, status: &str) -> Result<responses::DelInvoice, Error> {
        self.call("delinvoice", requests::DelInvoice { label, status }).await
    }

    /// Delete all expired invoices that expired as of given {maxexpirytime} (a UNIX epoch time),
    /// or all expired invoices if not specified.
    pub async fn delexpiredinvoice(
        &mut self,
        maxexpirytime: Option<u64>,
    ) -> Result<responses::DelExpiredInvoice, Error> {
        self.call(
            "delexpiredinvoice",
            requests::DelExpiredInvoice { maxexpirytime },
        ).await
    }

    /// Set up autoclean of expired invoices. Perform cleanup every {cycle_seconds} (default 3600),
    /// or disable autoclean if 0. Clean up expired invoices that have expired for {expired_by}
    /// seconds (default 86400).
    pub async fn autocleaninvoice(
        &mut self,
        cycle_seconds: Option<u64>,
        expired_by: Option<u64>,
    ) -> Result<responses::AutoCleanInvoice, Error> {
        self.call(
            "autocleaninvoice",
            requests::AutoCleanInvoice {
                cycle_seconds,
                expired_by,
            },
        ).await
    }

    /// Wait for the next invoice to be paid, after {lastpay_index}.
    /// (if supplied)
    pub async fn waitanyinvoice(
        &mut self,
        lastpay_index: Option<u64>,
    ) -> Result<responses::WaitAnyInvoice, Error> {
        self.call("waitanyinvoice", requests::WaitAnyInvoice { lastpay_index }).await
    }

    /// Wait for an incoming payment matching the invoice with {label}.
    pub async fn waitinvoice(&mut self, label: &str) -> Result<responses::WaitInvoice, Error> {
        self.call("waitinvoice", requests::WaitInvoice { label }).await
    }

    /// Send a lightning payment.
    ///
    /// # Arguments
    ///
    /// * `bolt11` - A string that holds the payment information in bolt11 format.
    /// * `options` - Options for this payment. Use `Default::default()` to not pass any options.
    pub async fn pay<'a>(&mut self, bolt11: &str, options: PayOptions<'a>) -> Result<responses::Pay, Error> {
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
        ).await
    }

    /// Send along {route} in return for preimage of {payment_hash}, with optional {description}.
    pub async fn sendpay(
        &mut self,
        route: Vec<common::RouteItem>,
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
        ).await
    }

    /// Wait for payment attempt on {payment_hash} to succeed or fail, but only up to {timeout} seconds.
    pub async fn waitsendpay(
        &mut self,
        payment_hash: &str,
        timeout: u64,
    ) -> Result<responses::WaitSendPay, Error> {
        self.call(
            "waitsendpay",
            requests::WaitSendPay {
                payment_hash,
                timeout,
            },
        ).await
    }

    /// Show outgoing payments.
    pub async fn listsendpays(
        &mut self,
        bolt11: Option<&str>,
        payment_hash: Option<&str>,
    ) -> Result<responses::ListSendPays, Error> {
        self.call(
            "listsendpays",
            requests::ListSendPays {
                bolt11,
                payment_hash,
            },
        ).await
    }

    /// Decode {bolt11}, using {description} if necessary.
    pub async fn decodepay(
        &mut self,
        bolt11: &str,
        description: Option<&str>,
    ) -> Result<responses::DecodePay, Error> {
        self.call(
            "decodepay",
            requests::DecodePay {
                bolt11,
                description,
            },
        ).await
    }

    /// Show route to {id} for {msatoshi}, using {riskfactor} and optional {cltv} (default 9). If
    /// specified search from {fromid} otherwise use this node as source. Randomize the route with
    /// up to {fuzzpercent} (0.0 -> 100.0, default 5.0) using {seed} as an arbitrary-size string
    /// seed.
    pub async fn getroute(
        &mut self,
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
        ).await
    }

    /// Connect to {id} at {host} (which can end in ':port' if not default). {id} can also be of
    /// the form id@host.
    pub async fn connect(&mut self, id: &str, host: Option<&str>) -> Result<responses::Connect, Error> {
        self.call("connect", requests::Connect { id, host }).await
    }

    /// Disconnect from peer with {peer_id}.
    pub async fn disconnect(&mut self, id: &str) -> Result<responses::Disconnect, Error> {
        self.call("disconnect", requests::Disconnect { id }).await
    }

    /// Fund a new channel with another lightning node.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of node to fund a channel to
    /// * `satoshi` - either `AmountOrAll::Amount(n)` for a given amount in satoshi units, or
    /// `AmountOrAll::All` to spend all available funds
    /// * `feerate` - optional feerate to use for Bitcoin transaction
    pub async fn fundchannel(
        &mut self,
        id: &str,
        satoshi: requests::AmountOrAll,
        feerate: Option<u64>,
    ) -> Result<responses::FundChannel, Error> {
        self.call(
            "fundchannel",
            requests::FundChannel {
                id,
                satoshi,
                feerate,
            },
        ).await
    }

    /// Close the channel with {id} (either peer ID, channel ID, or short channel ID). If {force}
    /// (default false) is true, force a unilateral close after {timeout} seconds (default 30),
    /// otherwise just schedule a mutual close later and fail after timing out.
    pub async fn close(
        &mut self,
        id: &str,
        force: Option<bool>,
        timeout: Option<u64>,
    ) -> Result<responses::Close, Error> {
        self.call("close", requests::Close { id, force, timeout }).await
    }

    /// Send {peerid} a ping of length {len} (default 128) asking for {pongbytes} (default 128).
    pub async fn ping(
        &mut self,
        id: &str,
        len: Option<u64>,
        pongbytes: Option<u64>,
    ) -> Result<responses::Ping, Error> {
        self.call("ping", requests::Ping { id, len, pongbytes }).await
    }

    /// Show available funds from the internal wallet.
    pub async fn listfunds(&mut self) -> Result<responses::ListFunds, Error> {
        self.call("listfunds", requests::ListFunds {}).await
    }

    /// Send to destination address via Bitcoin transaction.
    ///
    /// # Arguments
    ///
    /// * `destination` - Bitcoin address to send to
    /// * `satoshi` - either `AmountOrAll::Amount(n)` for a given amount in satoshi units, or
    /// `AmountOrAll::All` to spend all available funds
    /// * `feerate` - optional feerate to use for Bitcoin transaction
    pub async fn withdraw(
        &mut self,
        destination: &str,
        satoshi: requests::AmountOrAll,
        feerate: Option<u64>,
    ) -> Result<responses::Withdraw, Error> {
        self.call(
            "withdraw",
            requests::Withdraw {
                destination,
                satoshi,
                feerate,
            },
        ).await
    }

    /// Get a new {bech32, p2sh-segwit} address to fund a channel (default is bech32).
    pub async fn newaddr(&mut self, addresstype: Option<&str>) -> Result<responses::NewAddr, Error> {
        self.call("newaddr", requests::NewAddr { addresstype }).await
    }

    /// Shut down the lightningd process.
    pub async fn stop(&mut self) -> Result<responses::Stop, Error> {
        self.call("stop", requests::Stop {}).await
    }
}
