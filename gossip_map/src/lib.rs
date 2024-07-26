#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufReader, Error, ErrorKind};

use fundamentals::core::FromWire;
use fundamentals::types::ShortChannelId;
// Reexport types
pub use fundamentals::*;

mod bolt7;
mod flags;
mod gossip_stor_wiregen;
mod gossip_types;
mod peekable_stream;

use crate::bolt7::{ChannelAnnouncement, ChannelUpdate, NodeAnnouncement};
use crate::flags::{
    GOSSIP_STORE_MAJOR_VERSION, GOSSIP_STORE_MAJOR_VERSION_MASK, WIRE_GOSSIP_STORE_CHANNEL_AMOUNT,
    WIRE_GOSSIP_STORE_DELETE_CHAN, WIRE_GOSSIP_STORE_ENDED, WIRE_GOSSIP_STORE_PRIVATE_CHANNEL,
    WIRE_GOSSIP_STORE_PRIVATE_UPDATE,
};
use crate::gossip_stor_wiregen::{
    GossipStoreChannelAmount, GossipStoreDeleteChan, GossipStoreEnded,
};
use crate::gossip_types::{GossipChannel, GossipNode, GossipNodeId, GossipStoredHeader};

/// Gossip map implementation, that allow you to manage the gossip_store
/// written by core lightning.
#[derive(Debug)]
struct GossipMap {
    // FIXME: make this optional
    path: String,
    version: u8,
    stream: Option<BufReader<File>>,
    nodes: HashMap<GossipNodeId, GossipNode>,
    channels: HashMap<ShortChannelId, GossipChannel>,
    orphan_channel_updates: HashMap<ShortChannelId, ChannelUpdate>,
}

impl GossipMap {
    // Create a new instance of the gossip map.
    pub fn new(version: u8) -> Self {
        log::debug!("gossip map version `{version}`");
        GossipMap {
            path: "".to_owned(),
            version,
            stream: None,
            nodes: HashMap::new(),
            channels: HashMap::new(),
            orphan_channel_updates: HashMap::new(),
        }
    }

    pub fn from_file(file_name: &str) -> anyhow::Result<Self> {
        log::debug!("Loading gossip map from file `{file_name}`");
        let gossip_store = File::open(file_name)?;
        let stream = BufReader::new(gossip_store);
        let mut gossip_map = GossipMap {
            path: file_name.to_owned(),
            version: 0,
            stream: Some(stream),
            nodes: HashMap::new(),
            channels: HashMap::new(),
            orphan_channel_updates: HashMap::new(),
        };
        gossip_map.refresh()?;
        Ok(gossip_map)
    }

    pub fn get_channel(&self, short_chananel_id: &str) -> Option<&GossipChannel> {
        self.channels.get(short_chananel_id.as_bytes())
    }

    pub fn get_node(&self, node_id: &str) -> Option<&GossipNode> {
        let node_id = GossipNodeId::from(node_id);
        self.nodes.get(&node_id)
    }

    /// add a node announcement message inside the gossip map
    fn add_node_announcement(&mut self, node_announce: NodeAnnouncement) {}

    /// add a channel announcement message inside the gossip map.
    fn add_channel_announcement(&mut self, channel_announce: ChannelAnnouncement) {}

    fn refresh(&mut self) -> anyhow::Result<()> {
        let gossip_store = File::open(self.path.clone())?;
        let stream = BufReader::new(gossip_store);
        let mut stream = peekable_stream::PeekableStream::new(stream);

        let version = u8::from_wire(&mut stream)? as u16;
        if (version & GOSSIP_STORE_MAJOR_VERSION_MASK) != GOSSIP_STORE_MAJOR_VERSION {
            anyhow::bail!("Invalid gossip store version {version}");
        }
        self.version = version as u8;

        let mut last_short_channel_id: Option<ShortChannelId> = None;
        while let Ok(header) = GossipStoredHeader::from_wire(&mut stream) {
            log::debug!("header {:?}", header);
            if (header.flag() & flags::GOSSIP_STORE_LEN_DELETED_BIT) != 0 {
                log::debug!("flags::GOSSIP_STORE_LEN_DELETED_BIT");
                continue;
            }
            let typmsg = stream.peek_msgtype()?;
            log::info!("type: {typmsg}");
            match typmsg {
                // channel announcement!
                256 => {
                    let channel_announcement = ChannelAnnouncement::from_wire(&mut stream)?;
                    log::trace!("{:?}", channel_announcement);
                    let node_one =
                        GossipNodeId::from_bytes(&channel_announcement.node_id_1.to_vec()).unwrap();
                    let node_two =
                        GossipNodeId::from_bytes(&channel_announcement.node_id_2.to_vec()).unwrap();
                    if !self.nodes.contains_key(&node_one) {
                        let node = GossipNode::new(node_one.clone(), None);
                        self.nodes.insert(node_one.clone(), node);
                    }

                    if !self.nodes.contains_key(&node_two) {
                        let node = GossipNode::new(node_two.clone(), None);
                        self.nodes.insert(node_two.clone(), node);
                    }
                    last_short_channel_id = Some(channel_announcement.short_channel_id);
                    let channel = GossipChannel::new(channel_announcement, &node_one, &node_two);
                    // SAFETY: It is safe to unwrap because the node is always present, due the
                    // previous checks.
                    let node_one = self.nodes.get_mut(&node_one).unwrap();
                    node_one.add_channel(&channel.clone());
                    let node_two = self.nodes.get_mut(&node_two).unwrap();
                    node_two.add_channel(&channel.clone());
                    self.channels
                        .insert(last_short_channel_id.unwrap(), channel);
                }
                WIRE_GOSSIP_STORE_PRIVATE_CHANNEL => {
                    log::info!("private channel announcement");
                    unimplemented!();
                }
                WIRE_GOSSIP_STORE_CHANNEL_AMOUNT => {
                    let channel_amount = GossipStoreChannelAmount::from_wire(&mut stream)?;
                    log::trace!("{:?}", channel_amount);
                    //FIXME: remove the unwrap().
                    let channel = self
                        .channels
                        .get_mut(&last_short_channel_id.unwrap())
                        .unwrap();
                    channel.set_amount(channel_amount);
                }
                WIRE_GOSSIP_STORE_PRIVATE_UPDATE => {
                    log::info!("private update for channel");
                    unimplemented!()
                }
                WIRE_GOSSIP_STORE_DELETE_CHAN => {
                    log::info!("delte channel from gossip");
                    let _ = GossipStoreDeleteChan::from_wire(&mut stream)?;
                    unimplemented!()
                }
                WIRE_GOSSIP_STORE_ENDED => {
                    log::info!("end of the gossip store");
                    let _ = GossipStoreEnded::from_wire(&mut stream)?;
                    break;
                }
                257 => {
                    let node_announcement = NodeAnnouncement::from_wire(&mut stream).unwrap();
                    log::trace!("{:?}", node_announcement);
                    let node_id = GossipNodeId::from_bytes(&node_announcement.node_id)?;
                    if !self.nodes.contains_key(&node_id) {
                        let node = GossipNode::new(node_id.clone(), Some(node_announcement));
                        self.nodes.insert(node_id, node);
                    }
                }
                258 => {
                    log::info!("found channel update");
                    let channel_update = ChannelUpdate::from_wire(&mut stream)?;
                    if let Some(channel) = self.channels.get_mut(&channel_update.short_channel_id) {
                        log::info!(
                            "found channel with short id `{}`",
                            hex::encode(channel_update.short_channel_id)
                        );
                        channel.channel_update(&channel_update)
                    } else {
                        self.orphan_channel_updates
                            .insert(channel_update.short_channel_id, channel_update);
                    }
                }
                _ => anyhow::bail!("Unexpected message with type `{typmsg}`"),
            }
        }
        log::info!("{:#?}", self.nodes);
        log::info!("{:#?}", self.channels);
        Ok(())
    }
}

#[cfg(test)]
mod logger;

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::*;

    static INIT: Once = Once::new();

    fn init() {
        INIT.call_once(|| {
            logger::init(log::Level::Trace).expect("initializing logger for the first time");
        });
    }

    #[test]
    fn read_gossipmap_from_file() {
        init();
        let path = "/run/media/vincent/VincentSSD/.lightning/signet/gossip_store";
        let pubkey = "03b39d1ddf13ce486de74e9e44e0538f960401a9ec75534ba9cfe4100d65426880";
        let map = GossipMap::from_file(path);
        assert!(map.is_ok(), "{:?}", map);
        let map = map.unwrap();
        assert!(
            map.get_node(pubkey).is_some(),
            "node with id `{pubkey}` not found!"
        );
    }
}
