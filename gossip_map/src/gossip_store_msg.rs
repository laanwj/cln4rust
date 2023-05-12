// code generated with the lngen, please not edit this file.
use std::io::{Read, Write};

use fundamentals_derive::{DecodeWire, EncodeWire};

use fundamentals::core::{FromWire, ToWire};
use fundamentals::prelude::*;

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct GossipStoreChanDying {
    #[warn(dead_code)]
    #[msg_type = 4106]
    ty: u16,
    scid: ShortChannelId,
    blockheight: u32,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct GossipStoreChannelAmount {
    #[warn(dead_code)]
    #[msg_type = 4101]
    ty: u16,
    pub satoshis: u64,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct GossipStoreDeleteChan {
    #[warn(dead_code)]
    #[msg_type = 4103]
    ty: u16,
    scid: ShortChannelId,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct GossipStoreEnded {
    #[warn(dead_code)]
    #[msg_type = 4105]
    ty: u16,
    equivalent_offset: u64,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct GossipStorePrivateChannel {
    #[warn(dead_code)]
    #[msg_type = 4104]
    ty: u16,
    satoshis: u16,
    announcement: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct GossipStorePrivateUpdate {
    #[warn(dead_code)]
    #[msg_type = 4102]
    ty: u16,
    update: BitFlag,
}
