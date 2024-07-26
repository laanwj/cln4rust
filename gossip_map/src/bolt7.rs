// code generated with the lngen, please not edit this file.
use std::io::{Read, Write};

use fundamentals_derive::{DecodeWire, EncodeWire};

use crate::core::{FromWire, ToWire};
use crate::prelude::*;

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct AnnouncementSignatures {
    #[msg_type = 259]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub short_channel_id: ShortChannelId,
    pub node_signature: Signature,
    pub bitcoin_signature: Signature,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct ChannelAnnouncement {
    #[msg_type = 256]
    pub ty: u16,
    pub node_signature_1: Signature,
    pub node_signature_2: Signature,
    pub bitcoin_signature_1: Signature,
    pub bitcoin_signature_2: Signature,
    pub features: BitFlag,
    pub chain_hash: ChainHash,
    pub short_channel_id: ShortChannelId,
    pub node_id_1: Point,
    pub node_id_2: Point,
    pub bitcoin_key_1: Point,
    pub bitcoin_key_2: Point,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct ChannelUpdate {
    #[msg_type = 258]
    pub ty: u16,
    pub signature: Signature,
    pub chain_hash: ChainHash,
    pub short_channel_id: ShortChannelId,
    pub timestamp: u32,
    // FIXME: these are u8 but the codegen will decode it to BitFlag
    pub message_flags: u8,
    pub channel_flags: u8,
    pub cltv_expiry_delta: u16,
    pub htlc_minimum_msat: u64,
    pub fee_base_msat: u32,
    pub fee_proportional_millionths: u32,
    pub htlc_maximum_msat: u64,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct GossipTimestampFilter {
    #[msg_type = 265]
    pub ty: u16,
    pub chain_hash: ChainHash,
    pub first_timestamp: u32,
    pub timestamp_range: u32,
}

macro_rules! to_wire_type_with_size {
    ($ty: ty, $size: expr) => {
        impl ToWire for $ty {
            fn to_wire<W: Write>(&self, buff: &mut W) -> std::io::Result<()> {
                buff.write_all(self)
            }
        }

        impl FromWire for $ty {
            fn from_wire<R: Read>(reader: &mut R) -> std::io::Result<Self> {
                let mut buff = [0; $size];
                reader.read_exact(&mut buff)?;
                Ok(buff)
            }
        }
    };
}

pub type Alias = [u8; 32];
pub type Rgb = [u8; 3];

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct NodeAnnouncement {
    #[msg_type = 257]
    pub ty: u16,
    pub signature: Signature,
    pub features: BitFlag,
    pub timestamp: u32,
    pub node_id: Point,
    pub rgb_color: Rgb,
    pub alias: Alias,
    pub addresses: BitFlag,
    pub node_ann_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct QueryChannelRange {
    #[msg_type = 263]
    pub ty: u16,
    pub chain_hash: ChainHash,
    pub first_blocknum: u32,
    pub number_of_blocks: u32,
    pub query_channel_range_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct QueryShortChannelIds {
    #[msg_type = 261]
    pub ty: u16,
    pub chain_hash: ChainHash,
    pub encoded_short_ids: BitFlag,
    pub query_short_channel_ids_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct ReplyChannelRange {
    #[msg_type = 264]
    pub ty: u16,
    pub chain_hash: ChainHash,
    pub first_blocknum: u32,
    pub number_of_blocks: u32,
    pub sync_complete: BitFlag,
    pub encoded_short_ids: BitFlag,
    pub reply_channel_range_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct ReplyShortChannelIdsEnd {
    #[msg_type = 262]
    pub ty: u16,
    pub chain_hash: ChainHash,
    pub full_information: BitFlag,
}
