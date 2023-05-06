use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Error, ErrorKind},
};

use fundamentals::{
    bolt::bolt7::ChannelAnnouncement,
    prelude::bolt7::{ChannelUpdate, NodeAnnouncement},
};
use fundamentals::{core::FromWire, types::ShortChannelId};

mod flags;
mod gossip_store_msg;
mod types;

use flags::{
    GOSSIP_STORE_MAJOR_VERSION, GOSSIP_STORE_MAJOR_VERSION_MASK, WIRE_GOSSIP_STORE_CHANNEL_AMOUNT,
    WIRE_GOSSIP_STORE_DELETE_CHAN, WIRE_GOSSIP_STORE_ENDED, WIRE_GOSSIP_STORE_PRIVATE_CHANNEL,
    WIRE_GOSSIP_STORE_PRIVATE_UPDATE,
};
use gossip_store_msg::*;
use types::{GossipChannel, GossipNode, GossipNodeId};

/// Gossip map implementation, that allow you to manage the gossip_store
/// written by core lightning.
struct GossipMap<'a> {
    version: u8,
    stream: Option<BufReader<File>>,
    nodes: HashMap<GossipNodeId, GossipNode<'a>>,
    channels: HashMap<ShortChannelId, GossipChannel<'a>>,
    orphan_channel_updates: HashSet<ChannelUpdate>,
}

impl GossipMap<'_> {
    // Create a new instance of the gossip map.
    pub fn new(version: u8) -> Self {
        GossipMap {
            version,
            stream: None,
            nodes: HashMap::new(),
            channels: HashMap::new(),
            orphan_channel_updates: HashSet::new(),
        }
    }

    pub fn from_file(file_name: &str) -> Result<Self, std::io::Error> {
        let gossip_store = File::open(file_name)?;
        let stream = BufReader::new(gossip_store);
        let mut gossip_map = GossipMap {
            version: 0,
            stream: Some(stream),
            nodes: HashMap::new(),
            channels: HashMap::new(),
            orphan_channel_updates: HashSet::new(),
        };
        gossip_map.refresh()?;
        Ok(gossip_map)
    }

    pub fn get_channel(&self, short_chananel_id: &str) -> Option<&'static GossipChannel> {
        self.channels.get(short_chananel_id.as_bytes())
    }

    pub fn get_node(&self, node_id: &str) -> Option<&'static GossipNode> {
        // FIXME: store the node as String?
        self.nodes.get(&GossipNodeId {
            node_id: node_id.to_owned(),
        })
    }

    fn refresh(&mut self) -> std::io::Result<()> {
        let mut stream = self.stream.as_mut().unwrap();
        let version = u8::from_wire(&mut stream)? as u16;
        if (version & GOSSIP_STORE_MAJOR_VERSION_MASK) != GOSSIP_STORE_MAJOR_VERSION {
            return Err(Error::new(
                ErrorKind::Other,
                "Invalid gossip store version {version}",
            ));
        }
        self.version = version as u8;

        while let Ok(chunk) = u8::from_wire(&mut stream) {
            match chunk as u16 {
                // channel announcement!
                256 => {
                    let channel_announcement = ChannelAnnouncement::from_wire(&mut stream)?;
                }
                WIRE_GOSSIP_STORE_PRIVATE_CHANNEL => {
                    let private_channel = GossipStorePrivateChannel::from_wire(&mut stream)?;
                }
                WIRE_GOSSIP_STORE_CHANNEL_AMOUNT => {
                    let channel_amount = GossipStoreChannelAmount::from_wire(&mut stream)?;
                }
                WIRE_GOSSIP_STORE_PRIVATE_UPDATE => {
                    let private_update = GossipStorePrivateUpdate::from_wire(&mut stream)?;
                }
                WIRE_GOSSIP_STORE_DELETE_CHAN => {
                    let del_chan = GossipStoreDeleteChan::from_wire(&mut stream)?;
                }
                WIRE_GOSSIP_STORE_ENDED => {
                    let eof = GossipStoreEnded::from_wire(&mut stream)?;
                }
                257 => {
                    let node_announcement = NodeAnnouncement::from_wire(&mut stream)?;
                }
                258 => {
                    let channel_update = ChannelUpdate::from_wire(&mut stream)?;
                }
                _ => continue,
            }
        }

        Ok(())
    }
}
