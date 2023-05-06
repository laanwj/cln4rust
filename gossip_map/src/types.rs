//! Gossip map types implementations.
use std::{collections::HashMap, io::BufRead, str::Bytes, vec::Vec};

use byteorder::{BigEndian, ReadBytesExt};

use crate::flags::{GOSSIP_STORE_LEN_DELETED_BIT, GOSSIP_STORE_LEN_MASK};

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

impl GossipNodeId {
    pub(crate) fn from_bytes(buff: &[u8; 33]) -> std::io::Result<Self> {
        Ok(GossipNodeId {
            node_id: String::from_utf8(buff.to_vec()).map_err(|err| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", err))
            })?,
        })
    }
}

impl GossipType for GossipNodeId {
    fn decode(stream: &mut dyn BufRead) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let mut node_id: String = String::new();
        stream.read_to_string(&mut node_id)?;

        // FIXME: missed sanity check!
        let res = GossipNodeId { node_id };
        Ok(res)
    }

    fn encode(&self) -> Bytes {
        todo!()
    }
}

pub struct GossipNode<'a> {
    node_id: GossipNodeId,
    announce_fileds: Option<HashMap<String, String>>,
    announce_offset: Option<u32>,
    channels: Vec<&'a GossipChannel<'a>>,
}

impl GossipNode<'_> {
    pub fn new(node_id: GossipNodeId) -> Self {
        Self {
            node_id,
            announce_fileds: None,
            announce_offset: None,
            channels: vec![],
        }
    }
}

impl<'a> GossipNode<'a> {
    /// add a gossip channel inside the gossip map.
    pub fn add_channel(&'a mut self, channel: &'a GossipChannel) {
        self.channels.push(channel);
    }
}

impl GossipType for GossipNode<'_> {
    fn decode(stream: &mut dyn BufRead) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let mut buff = [0; 33];
        stream.read_exact(&mut buff)?;
        let node_id = GossipNodeId::from_bytes(&buff)?;
        Ok(Self {
            node_id,
            announce_fileds: None,
            announce_offset: None,
            channels: vec![],
        })
    }

    fn encode(&self) -> Bytes {
        todo!()
    }
}

/// Channel Information stored inside the Gossip Map.
pub struct GossipChannel<'a> {
    fileds: HashMap<String, String>,
    annound_offset: u32,
    scid: String,
    node_one: &'a GossipNode<'a>,
    node_two: &'a GossipNode<'a>,
    update_fields: Vec<HashMap<String, String>>,
    update_offset: Vec<u32>,
    satoshi: Option<u64>,
    half_channels: Vec<&'a GossipPartialChannel>,
}

impl GossipType for GossipChannel<'_> {
    fn decode(stream: &mut dyn BufRead) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        todo!()
    }

    fn encode(&self) -> Bytes {
        todo!()
    }
}

/// One direction gossip map channel
pub struct GossipPartialChannel {}

/// Gossip map header, that contains the version
/// of the gossip map.
pub struct GossipStoredHeader {
    flags: bool,
    len: u16,
    crc: u32,
    timestamp: u32,
}

impl GossipType for GossipStoredHeader {
    fn decode(stream: &mut dyn BufRead) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        let len = stream.read_u16::<BigEndian>()?;
        let crc = stream.read_u32::<BigEndian>()?;
        let timestamp = stream.read_u32::<BigEndian>()?;

        Ok(GossipStoredHeader {
            timestamp,
            crc,
            flags: (len as u32 & GOSSIP_STORE_LEN_DELETED_BIT) != 0,
            len: (len & GOSSIP_STORE_LEN_MASK),
        })
    }

    fn encode(&self) -> Bytes {
        todo!()
    }
}
