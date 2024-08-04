//! Gossip map types implementations.
use std::fmt::Debug;
use std::io::Read;
use std::{collections::HashMap, io::BufRead, str::Bytes, vec::Vec};

use bitcoin::PublicKey;
use fundamentals::core::FromWire;
use fundamentals::types::ShortChannelId;
use fundamentals_derive::DecodeWire;

use crate::bolt7::{ChannelAnnouncement, ChannelUpdate, NodeAnnouncement};
use crate::gossip_stor_wiregen::GossipStoreChannelAmount;

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
    pub(crate) fn from_bytes(buff: &[u8]) -> std::io::Result<Self> {
        Ok(GossipNodeId {
            node_id: PublicKey::from_slice(buff).unwrap().to_string(),
        })
    }
}

#[derive(Clone)]
pub struct GossipNode {
    node_id: GossipNodeId,
    announced: bool,
    raw_message: Option<NodeAnnouncement>,
    channels: Vec<GossipChannel>,
}

impl Debug for GossipNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "node_id: {:?}", self.node_id)?;
        writeln!(f, "announced: {:?}", self.announced)
    }
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
#[derive(Clone)]
pub struct GossipChannel {
    pub inner: ChannelAnnouncement,
    pub annound_offset: u32,
    pub scid: ShortChannelId,
    pub node_one: GossipNodeId,
    pub node_two: GossipNodeId,
    pub update_fields: Vec<HashMap<String, String>>,
    pub update_offset: Vec<u32>,
    pub satoshi: Option<u64>,
    pub half_channels: HashMap<u8, GossipPartialChannel>,
    pub private: bool,
}

impl Debug for GossipChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "node_id_1: {:?}", self.node_one)?;
        writeln!(f, "node_id_2: {:?}", self.node_two)
    }
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
