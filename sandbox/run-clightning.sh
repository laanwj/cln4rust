#!/bin/bash

echo 'run c-lightning'

DIR=/workdir

lightningd --lightning-dir=$DIR/lightning_dir_one --log-file=$DIR/lightning_dir_two/log.txt --daemon
lightningd --lightning-dir=$DIR/lightning_dir_two --log-file=$DIR/lightning_dir_two/log.txt --daemon

lightning-cli --lightning-dir=$DIR/lightning_dir_one getinfo > node_one.info
#for run in {1..50}; do
#  address_one="$(lightning-cli --lightning-dir=$DIR/lightning_dir_one newaddr | jq -r '.bech32')"
  #echo "${address_one}"
  # From https://bitcoincore.org/en/doc/0.21.0/rpc/wallet/sendtoaddress/
  #bitcoin-cli -datadir=$DIR/bitcoin_dir sendtoaddress "${address}" 50 "drinks" "room77" true true null "unset" null 1.1
#  bitcoin-cli -datadir=$DIR/bitcoin_dir -named sendtoaddress address="${address_one}" amount=1 fee_rate=1 > /dev/null
#  address="$(bitcoin-cli -datadir=$DIR/bitcoin_dir getnewaddress)"
#  bitcoin-cli -datadir=$DIR/bitcoin_dir generatetoaddress 6 "${address}" > /dev/null
#done


lightning-cli --lightning-dir=$DIR/lightning_dir_two getinfo > node_two.info
for run in {1..50}; do
  address_two="$(lightning-cli --lightning-dir=$DIR/lightning_dir_two newaddr | jq -r '.bech32')"
  #echo "${address_two}"
  # From https://bitcoincore.org/en/doc/0.21.0/rpc/wallet/sendtoaddress/
  #bitcoin-cli -datadir=$DIR/bitcoin_dir sendtoaddress "${address}" 50 "drinks" "room77" true true null "unset" null 1.1
  bitcoin-cli -datadir=$DIR/bitcoin_dir -named sendtoaddress address="${address_two}" amount=1 fee_rate=1 > /dev/null
  address="$(bitcoin-cli -datadir=$DIR/bitcoin_dir getnewaddress)"
  bitcoin-cli -datadir=$DIR/bitcoin_dir generatetoaddress 50 "${address}" > /dev/null
done

## achieve the confirmation channels for sure
for run in {1..50}; do
  address="$(bitcoin-cli -datadir=$DIR/bitcoin_dir getnewaddress)"
  bitcoin-cli -datadir=$DIR/bitcoin_dir generatetoaddress 50 "${address}" > /dev/null
done

lightning-cli --lightning-dir=$DIR/lightning_dir_one listfunds
lightning-cli --lightning-dir=$DIR/lightning_dir_two listfunds