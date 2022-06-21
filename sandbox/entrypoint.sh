#!/bin/bash
./run-bitcoin.sh
./generate-block-bitcoin.sh
cd code || exit 1
make
cd .. || exit 1
./run-clightning.sh
sleep 0.5m # Give the time to c-lightning to sync the network
cd code || exit 1
echo "Running test with Cargo"
cargo test --workspace