//! High-level interface to c-lightning RPC
use serde::de::DeserializeOwned;
use serde::Serialize;
use strason::Json;

use client;
use error::Error;
use requests;
use responses;

/// Structure providing a high-level interface to the c-lightning daemon RPC
pub struct LightningRPC {
    client: client::Client,
}

impl LightningRPC {
    /// Create a new connection from a UNIX socket path
    pub fn new(sockname: String) -> LightningRPC {
        LightningRPC {
            client: client::Client::new(sockname),
        }
    }

    /// Generic call function for RPC calls
    fn call<T: Serialize, U: DeserializeOwned>(
        &mut self,
        method: &str,
        input: T,
    ) -> Result<U, Error> {
        let params = Json::from_serialize(input)?;
        let request = self.client.build_request(method.to_string(), params);
        self.client
            .send_request(&request)
            .and_then(|res| res.into_result::<U>())
    }

    /// Show information about this node
    pub fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {})
    }

    /// Supply feerate estimates manually.
    pub fn feerates(&mut self, style: &str) -> Result<responses::FeeRates, Error> {
        self.call(
            "feerates",
            requests::FeeRates {
                style: style.to_string(),
            },
        )
    }

    /// Show current peers, if {level} is set, include {log}s"
    pub fn listpeers(
        &mut self,
        id: Option<String>,
        level: Option<String>,
    ) -> Result<responses::ListPeers, Error> {
        self.call("listpeers", requests::ListPeers { id, level })
    }
}
