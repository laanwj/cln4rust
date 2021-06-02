#!/bin/bash
./run-bitcoin.sh
./generate-block-bitcoin.sh
./run-clightning.sh
cd code
echo "Running test with Cargo"
RESULT_TEST=$(cargo test)
cd ..
./stop-clightning.sh
./stop-bitcoin.sh
rm -rf **/*.pid **/regtest

if [ "$RESULT_TEST" -eq "0" ] ; then
    exit 0
else
    exit 1
fi
