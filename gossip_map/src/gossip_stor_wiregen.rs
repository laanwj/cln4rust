// code generated with the lngen, please not edit this file.
use std::io::{Read, Write};

use fundamentals_derive::{DecodeWire, EncodeWire};

use crate::core::{FromWire, ToWire};
use crate::prelude::*;

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct GossipStoreChanDying {
    #[msg_type = 4106]
    pub ty: u16,
    pub scid: ShortChannelId,
    pub blockheight: u32,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct GossipStoreChannelAmount {
    #[msg_type = 4101]
    pub ty: u16,
    pub satoshis: u64,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct GossipStoreDeleteChan {
    #[msg_type = 4103]
    pub ty: u16,
    pub scid: ShortChannelId,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct GossipStoreEnded {
    #[msg_type = 4105]
    pub ty: u16,
    pub equivalent_offset: u64,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct GossipStorePrivateChannelObs {
    #[msg_type = 4104]
    pub ty: u16,
    pub satoshis: u64,
    pub announcement: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct GossipStorePrivateUpdateObs {
    #[msg_type = 4102]
    pub ty: u16,
    pub update: BitFlag,
}
