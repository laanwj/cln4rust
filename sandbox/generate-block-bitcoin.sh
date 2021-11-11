#!/bin/bash
DIR=/workdir
address="$(bitcoin-cli -datadir=$DIR/bitcoin_dir getnewaddress)"
bitcoin-cli -datadir=$DIR/bitcoin_dir generatetoaddress 50 "${address}" > /dev/null
