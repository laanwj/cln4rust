# gossip map

Core Lightning Gossip Map parser to access the gossip map with rust.

## Example

``` rust
fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../contrib/gossip_store");
    let pubkey = "03e2408a49f07d2f4083a47344138ef89e7617e63919202c92aa8d49b574a560ae";
    let map = GossipMap::from_file(path);
    assert!(map.is_ok(), "{:?}", map);
    let map = map.unwrap();
    assert!(map.get_node(pubkey).is_some(), "node with id `{pubkey}` not found!");
}
```

## Reading the gossip store incrementally

`GossipStoreReader` returns the raw records of the gossip store one at
the time. When it has read all the records, it returns `None`, and the
next call returns the records that `gossipd` appended in the meantime.
It also follows the store when `gossipd` rewrites it on startup.

``` rust
fn main() -> anyhow::Result<()> {
    let mut reader = GossipStoreReader::new("/home/user/.lightning/bitcoin/gossip_store");
    while let Some(record) = reader.next_record()? {
        if record.is_deleted() {
            continue;
        }
        // `record.msg` is the raw message, starting with its type.
        println!("message of type {} at offset {}", record.msg_type(), record.offset);
    }
    Ok(())
}
```
