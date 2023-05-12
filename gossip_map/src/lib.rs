#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

use fundamentals::core::FromWire;
use fundamentals::prelude::bolt7::{ChannelAnnouncement, ChannelUpdate, NodeAnnouncement};
use fundamentals::types::ShortChannelId;

mod flags;
mod gossip_store_msg;
mod types;

use flags::{
    GOSSIP_STORE_MAJOR_VERSION, GOSSIP_STORE_MAJOR_VERSION_MASK, WIRE_GOSSIP_STORE_CHANNEL_AMOUNT,
    WIRE_GOSSIP_STORE_DELETE_CHAN, WIRE_GOSSIP_STORE_ENDED, WIRE_GOSSIP_STORE_PRIVATE_CHANNEL,
    WIRE_GOSSIP_STORE_PRIVATE_UPDATE,
};
use gossip_store_msg::*;
use types::{GossipChannel, GossipNode, GossipNodeId, GossipStoredHeader};

/// Gossip map implementation, that allow you to manage the gossip_store
/// written by core lightning.
#[derive(Debug)]
struct GossipMap {
    version: u8,
    stream: Option<BufReader<File>>,
    nodes: HashMap<GossipNodeId, GossipNode>,
    channels: HashMap<ShortChannelId, GossipChannel>,
    orphan_channel_updates: HashMap<ShortChannelId, ChannelUpdate>,
}

impl GossipMap {
    // Create a new instance of the gossip map.
    pub fn new(version: u8) -> Self {
        GossipMap {
            version,
            stream: None,
            nodes: HashMap::new(),
            channels: HashMap::new(),
            orphan_channel_updates: HashMap::new(),
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
        println!("version {version}");

        let mut last_short_channel_id: Option<ShortChannelId> = None;
        loop {
            let Ok(header) = GossipStoredHeader::from_wire(&mut stream) else {
                break; // EOF?
            };
            match header.flag() {
                flags::GOSSIP_STORE_LEN_DELETED_BIT | flags::GOSSIP_STORE_LEN_RATELIMIT_BIT => {
                    continue
                }
                _ => {}
            }

            println!("header {:?}", header);
            let chunk = u16::from_wire(&mut stream)?;
            println!("chunk {chunk}");
            match chunk {
                // channel announcement!
                256 => {
                    let channel_announcement = ChannelAnnouncement::from_wire(&mut stream)?;
                    println!("{:?}", channel_announcement);
                    let node_one = GossipNodeId::from_bytes(&channel_announcement.node_id_1)?;
                    let node_two = GossipNodeId::from_bytes(&channel_announcement.node_id_2)?;
                    if !self.nodes.contains_key(&node_one) {
                        let node = GossipNode::new(node_one.clone(), None);
                        self.nodes.insert(node_one.clone(), node);
                    }

                    if !self.nodes.contains_key(&node_two) {
                        let node = GossipNode::new(node_two.clone(), None);
                        self.nodes.insert(node_two.clone(), node);
                    }
                    println!("{:?}", self.nodes);
                    last_short_channel_id = Some(channel_announcement.short_channel_id);
                    let channel = GossipChannel::new(channel_announcement, &node_one, &node_two);
                    // SAFETY: this is sage because the node is always present, due the
                    // previous checks.
                    let node_one = self.nodes.get_mut(&node_one).unwrap();
                    node_one.add_channel(&channel.clone());
                    let node_two = self.nodes.get_mut(&node_two).unwrap();
                    node_two.add_channel(&channel.clone());
                    self.channels
                        .insert(last_short_channel_id.unwrap(), channel);
                }
                WIRE_GOSSIP_STORE_PRIVATE_CHANNEL => {
                    let _ = stream.seek_relative(2 + 8 + 2)?;
                    let channel_announcement = ChannelAnnouncement::from_wire(&mut stream)?;

                    let node_one = GossipNodeId::from_bytes(&channel_announcement.node_id_1)?;
                    let node_two = GossipNodeId::from_bytes(&channel_announcement.node_id_2)?;
                    if !self.nodes.contains_key(&node_one) {
                        let node = GossipNode::new(node_one.clone(), None);
                        self.nodes.insert(node_one.clone(), node);
                    }

                    if !self.nodes.contains_key(&node_two) {
                        let node = GossipNode::new(node_two.clone(), None);
                        self.nodes.insert(node_two.clone(), node);
                    }

                    last_short_channel_id = Some(channel_announcement.short_channel_id);
                    let mut channel =
                        GossipChannel::new(channel_announcement, &node_one, &node_two);
                    // SAFETY: this is sage because the node is always present, due the
                    // previous checks.
                    let node_one = self.nodes.get_mut(&node_one).unwrap();
                    node_one.add_channel(&channel.clone());
                    let node_two = self.nodes.get_mut(&node_two).unwrap();
                    node_two.add_channel(&channel.clone());
                    channel.set_private(true);
                    self.channels
                        .insert(last_short_channel_id.unwrap(), channel);
                }
                WIRE_GOSSIP_STORE_CHANNEL_AMOUNT => {
                    let channel_amount = GossipStoreChannelAmount::from_wire(&mut stream)?;
                    //FIXME: remove the unwrap().
                    assert!(last_short_channel_id.is_some());
                    let channel = self
                        .channels
                        .get_mut(&last_short_channel_id.unwrap())
                        .unwrap();
                    channel.set_amount(channel_amount);
                }
                WIRE_GOSSIP_STORE_PRIVATE_UPDATE => {
                    let private_update = GossipStorePrivateUpdate::from_wire(&mut stream)?;
                    unimplemented!()
                }
                WIRE_GOSSIP_STORE_DELETE_CHAN => {
                    let del_chan = GossipStoreDeleteChan::from_wire(&mut stream)?;
                    unimplemented!()
                }
                WIRE_GOSSIP_STORE_ENDED => {
                    let _ = GossipStoreEnded::from_wire(&mut stream)?;
                    break;
                }
                257 => {
                    let node_announcement = NodeAnnouncement::from_wire(&mut stream)?;
                    let node_id = GossipNodeId::from_bytes(&node_announcement.node_id)?;
                    if !self.nodes.contains_key(&node_id) {
                        let node = GossipNode::new(node_id.clone(), Some(node_announcement));
                        self.nodes.insert(node_id, node);
                    }
                }
                258 => {
                    let channel_update = ChannelUpdate::from_wire(&mut stream)?;
                    if self.channels.contains_key(&channel_update.short_channel_id) {
                        // SAFETY: we check the existence before!
                        let channel = self
                            .channels
                            .get_mut(&channel_update.short_channel_id)
                            .unwrap();
                        channel.channel_update(&channel_update)
                    } else {
                        self.orphan_channel_updates
                            .insert(channel_update.short_channel_id, channel_update);
                    }
                }
                _ => assert!(false),
            }
            println!("----------------------------------------------------");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_gossipmap_from_file() {
        let path = "/run/media/vincent/VincentSSD/.lightning/testnet/gossip_store";
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
