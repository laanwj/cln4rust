#!/bin/bash

DIR=/workdir

lightningd --lightning-dir=$DIR/lightning_dir_one --log-file=$DIR/lightning_dir_two/log.txt --daemon
lightningd --lightning-dir=$DIR/lightning_dir_two --log-file=$DIR/lightning_dir_two/log.txt --daemon

## Useful to take information in some case from the other node.
# lightning-cli --lightning-dir=$DIR/lightning_dir_one getinfo > node_one.info
# lightning-cli --lightning-dir=$DIR/lightning_dir_two getinfo > node_two.info

for run in {1..2}; do
  address_two="$(lightning-cli --lightning-dir=$DIR/lightning_dir_two newaddr | jq -r '.bech32')"
  bitcoin-cli -datadir=$DIR/bitcoin_dir generatetoaddress 50 "${address_two}" > /dev/null
done

for run in {1..4}; do
  address="$(bitcoin-cli -datadir=$DIR/bitcoin_dir getnewaddress)"
  bitcoin-cli -datadir=$DIR/bitcoin_dir generatetoaddress 50 "${address}" > /dev/null
done

#lightning-cli --lightning-dir=$DIR/lightning_dir_one listfunds
#lightning-cli --lightning-dir=$DIR/lightning_dir_two listfunds
