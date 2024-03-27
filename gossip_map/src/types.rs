//! Gossip map types implementations.
use std::io::Read;
use std::{collections::HashMap, io::BufRead, str::Bytes, vec::Vec};

use bitcoin::PublicKey;
use fundamentals::core::FromWire;
use fundamentals::prelude::bolt7::{ChannelAnnouncement, ChannelUpdate, NodeAnnouncement};
use fundamentals::types::{Point, ShortChannelId};
use fundamentals_derive::DecodeWire;

use crate::gossip_store_msg::GossipStoreChannelAmount;

trait GossipType {
    /// Decode the gossip message from a sequence of bytes.
    fn decode(stream: &mut dyn BufRead) -> Result<Self, std::io::Error>
    where
        Self: Sized;

    /// Encode the gossip message in a sequence of bytes.
    fn encode(&self) -> Bytes;
}

/// Node Id encoded for the gossip map
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct GossipNodeId {
    pub(crate) node_id: String,
}

impl From<&str> for GossipNodeId {
    fn from(value: &str) -> Self {
        Self {
            node_id: value.to_owned(),
        }
    }
}

impl GossipNodeId {
    pub(crate) fn from_bytes(buff: &Point) -> std::io::Result<Self> {
        Ok(GossipNodeId {
            node_id: PublicKey::from_slice(buff).unwrap().to_string(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct GossipNode {
    node_id: GossipNodeId,
    announced: bool,
    raw_message: Option<NodeAnnouncement>,
    channels: Vec<GossipChannel>,
}

impl GossipNode {
    pub fn new(node_id: GossipNodeId, inner: Option<NodeAnnouncement>) -> Self {
        Self {
            node_id,
            // FIXME: this can be optional right? for
            // private channel we do not have one.
            raw_message: inner,
            channels: vec![],
            announced: true,
        }
    }
}

impl GossipNode {
    /// add a gossip channel inside the gossip map.
    pub fn add_channel(&mut self, channel: &GossipChannel) {
        self.channels.push(channel.clone());
    }
}

/// Channel Information stored inside the Gossip Map.
#[derive(Clone, Debug)]
pub struct GossipChannel {
    inner: ChannelAnnouncement,
    annound_offset: u32,
    scid: ShortChannelId,
    node_one: GossipNodeId,
    node_two: GossipNodeId,
    update_fields: Vec<HashMap<String, String>>,
    update_offset: Vec<u32>,
    satoshi: Option<u64>,
    half_channels: HashMap<u8, GossipPartialChannel>,
    private: bool,
}

impl GossipChannel {
    pub fn new(
        inner: ChannelAnnouncement,
        node_one: &GossipNodeId,
        node_two: &GossipNodeId,
    ) -> Self {
        GossipChannel {
            inner: inner.clone(),
            annound_offset: 0,
            scid: inner.short_channel_id,
            // FIXME: I can store only the ID?
            node_one: node_one.clone(),
            node_two: node_two.clone(),
            update_fields: vec![],
            update_offset: vec![],
            satoshi: None,
            half_channels: HashMap::new(),
            private: false,
        }
    }

    pub fn channel_update(&mut self, channel_update: &ChannelUpdate) {
        // FIXME: check how to normalize the BitFlag
        let direction = 1;
        self.half_channels.insert(
            direction,
            GossipPartialChannel::new(channel_update.to_owned()),
        );
    }

    pub fn set_amount(&mut self, amount: GossipStoreChannelAmount) {
        self.satoshi = Some(amount.satoshis.into());
    }

    pub fn set_private(&mut self, private: bool) {
        self.private = private;
    }
}

/// One direction gossip map channel
#[derive(Debug, Clone)]
pub struct GossipPartialChannel {
    pub inner: ChannelUpdate,
}

impl GossipPartialChannel {
    pub fn new(inner: ChannelUpdate) -> Self {
        Self { inner }
    }
}

/// Gossip map header, that contains the version
/// of the gossip map.
#[derive(DecodeWire, Debug)]
pub struct GossipStoredHeader {
    flags: u16,
    pub len: u16,
    crc: u32,
    timestamp: u32,
}

impl GossipStoredHeader {
    pub fn flag(&self) -> u16 {
        self.flags
    }
}
