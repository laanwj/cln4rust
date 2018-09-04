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

    pub fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        let params = Json::from_serialize(requests::GetInfo {})?;
        let request = self.client.build_request("getinfo".to_string(), params);
        self.client.send_request(&request).and_then(|res| res.into_result::<responses::GetInfo>())
    }

    pub fn feerates(&mut self, style: &str) -> Result<responses::FeeRates, Error> {
        let params = Json::from_serialize(requests::FeeRates { style: style.to_string() })?;
        let request = self.client.build_request("feerates".to_string(), params);
        self.client.send_request(&request).and_then(|res| res.into_result::<responses::FeeRates>())
    }
}

