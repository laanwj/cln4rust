use byteorder::{BigEndian, ReadBytesExt};
use flags::{
    GOSSIP_STORE_MAJOR_VERSION, GOSSIP_STORE_MAJOR_VERSION_MASK, WIRE_GOSSIP_STORE_CHANNEL_AMOUNT,
    WIRE_GOSSIP_STORE_DELETE_CHAN, WIRE_GOSSIP_STORE_ENDED, WIRE_GOSSIP_STORE_PRIVATE_CHANNEL,
    WIRE_GOSSIP_STORE_PRIVATE_UPDATE,
};
use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind},
};
use types::{GossipChannel, GossipNode};

mod flags;
mod types;

/// Gossip map implementation, that allow you to manage the gossip_store
/// written by core lightning.
struct GossipMap {
    version: u8,
    stream: Option<BufReader<File>>,
}

impl GossipMap {
    // Create a new instance of the gossip map.
    pub fn new(version: u8) -> Self {
        GossipMap {
            version,
            stream: None,
        }
    }

    pub fn from_file(file_name: &str) -> Result<Self, std::io::Error> {
        let gossip_store = File::open(file_name)?;
        let stream = BufReader::new(gossip_store);
        let mut gossip_map = GossipMap {
            version: 0,
            stream: Some(stream),
        };
        gossip_map.refresh()?;
        Ok(gossip_map)
    }

    pub fn get_channel(short_chananel_id: &str) -> Result<&'static GossipChannel, ()> {
        todo!()
    }

    pub fn get_node(node_id: &str) -> Result<&'static GossipNode, ()> {
        todo!()
    }

    fn refresh(&mut self) -> Result<(), std::io::Error> {
        let version = self.stream.as_mut().unwrap().read_u8()? as u16;
        if (version & GOSSIP_STORE_MAJOR_VERSION_MASK) != GOSSIP_STORE_MAJOR_VERSION {
            return Err(Error::new(
                ErrorKind::Other,
                "Invalid gossip tore version {version}",
            ));
        }
        self.version = version as u8;

        while let Ok(chunk) = self.stream.as_mut().unwrap().read_u8() {
            match chunk as u16 {
                // channel announcement!
                256 => todo!(),
                WIRE_GOSSIP_STORE_PRIVATE_CHANNEL => todo!("parsing the private channel"),
                WIRE_GOSSIP_STORE_CHANNEL_AMOUNT => todo!("channel ammount"),
                WIRE_GOSSIP_STORE_PRIVATE_UPDATE => todo!("private update"),
                WIRE_GOSSIP_STORE_DELETE_CHAN => todo!("channel deleted"),
                WIRE_GOSSIP_STORE_ENDED => todo!("need to be reimplemented the open store"),
                257 => todo!("node announcment"),
                258 => todo!("channel update"),
                _ => continue,
            }
        }

        Ok(())
    }
}
