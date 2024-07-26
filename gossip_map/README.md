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
