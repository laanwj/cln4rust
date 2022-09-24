//! Flag implementation for the gossip map types

pub static GOSSIP_STORE_MAJOR_VERSION: u16 = 0 << 5;
pub static GOSSIP_STORE_MAJOR_VERSION_MASK: u16 = 0xE0;

/// Deleted fields should be ignored: on restart, they will be removed as the gossip_store is rewritten.
pub static GOSSIP_STORE_LEN_DELETED_BIT: u32 = 0x80000000;
/// The push flag indicates gossip which is generated locally: this is important for gossip timestamp filtering,
/// where peers request gossip and we always send our own gossip messages even if the timestamp wasn't within their
/// request.pub static GOSSIP_STORE_LEN_PUSH_BIT: u32 = 0x40000000;
pub static GOSSIP_STORE_LEN_RATELIMIT_BIT: u32 = 0x20000000;
/// The ratelimit flag indicates that this gossip message came too fast.
/// The message are corded in the gossip map, but don't relay it to peers.
pub static GOSSIP_STORE_LEN_MASK: u16 = 0x0000FFFF;

// These duplicate constants in lightning/gossipd/gossip_store_wiregen.h
pub const WIRE_GOSSIP_STORE_PRIVATE_CHANNEL: u16 = 4104;
pub const WIRE_GOSSIP_STORE_PRIVATE_UPDATE: u16 = 4102;
pub const WIRE_GOSSIP_STORE_DELETE_CHAN: u16 = 4103;
pub const WIRE_GOSSIP_STORE_ENDED: u16 = 4105;
pub const WIRE_GOSSIP_STORE_CHANNEL_AMOUNT: u16 = 4101;
