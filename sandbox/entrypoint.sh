#!/bin/bash
./run-bitcoin.sh
./generate-block-bitcoin.sh
./run-clightning.sh
sleep 0.5m # Give the time to c-lightning to sync the network
cd code
echo "Running test with Cargo"
cargo test
cd ..
./stop-clightning.sh
./stop-bitcoin.sh
rm -rf **/*.pid **/regtest
