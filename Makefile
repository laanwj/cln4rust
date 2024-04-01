CC=cargo
FMT=fmt

# Please keep me update with the https://github.com/ElementsProject/lightning/blob/master/Makefile#L29
BOLTDIR := /tmp/
OPTIONS := ""

default: fmt
	$(CC) build --all-features
	@make example

fmt:
	$(CC) fmt --all

check:
	@make default
	$(CC) test -- --show-output

example:
	$(CC) build --example foo_plugin
	$(CC) build --example macros_ex

clean:
	$(CC) clean

# FIXME: we should apply the diff over the csv file.
genfile: check_compiler
	cd "$(BOLTDIR)" && git clone https://github.com/lightning/bolts.git
	cd /tmp && git clone https://github.com/ElementsProject/lightning.git
	cd $(BOLTDIR)/bolts && tools/extract-formats.py 07-routing-gossip.md > bolt7.csv
	cp /tmp/lightning/gossipd/gossip_store_wire.csv gossip_map/spec/gossip_store_wire.csv
	cp /tmp/bolts/bolt7.csv gossip_map/spec/bolt7.csv
	@make update-genfile

update-genfile:
	lncodegen-cli -l rust generate -b gossip_map/spec/gossip_store_wire.csv gossip_map/src/gossip_stor_wiregen.rs
	lncodegen-cli -l rust generate -b gossip_map/spec/bolt7.csv gossip_map/src/bolt7.rs


check_compiler:
	@command -v lncodegen-cli --help >/dev/null 2>&1 || (echo "`lncodegen-cli` not found, installing..." && cargo install lncodegen-cli --git https://github.com/lnspec-tools/lncodegen.git)
