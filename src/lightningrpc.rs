use serde::de::DeserializeOwned;
use serde::Serialize;
use strason::Json;

use client;
use error::Error;
use requests;
use responses;

pub struct LightningRPC {
    client: client::Client,
}

impl LightningRPC {
    pub fn new(sockname: String) -> LightningRPC {
        LightningRPC {
            client: client::Client::new(sockname),
        }
    }

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

    pub fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {})
    }

    pub fn feerates(&mut self, style: &str) -> Result<responses::FeeRates, Error> {
        self.call(
            "feerates",
            requests::FeeRates {
                style: style.to_string(),
            },
        )
    }

    pub fn listpeers(
        &mut self,
        id: Option<String>,
        level: Option<String>,
    ) -> Result<responses::ListPeers, Error> {
        self.call("listpeers", requests::ListPeers { id, level })
    }
}
