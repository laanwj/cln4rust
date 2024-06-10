CC=cargo
FMT=fmt

# Please keep me update with the https://github.com/ElementsProject/lightning/blob/master/Makefile#L29
BOLTDIR := /tmp/
OPTIONS := ""

default: fmt
	$(CC) build --all-features
	@make example

fmt: ## Format the file
	$(CC) fmt --all

check: ## Run all the tests inside the workspace
	@make default
	$(CC) test -- --show-output

example: # build the examples
	$(CC) build --example foo_plugin
	$(CC) build --example macros_ex

clean: ## Clean the file
	$(CC) clean

# FIXME: we should apply the diff over the csv file.
genfile: check_compiler ## Generate the file from a new main version of the spec, in addition apply potential patch
	cd "$(BOLTDIR)" && git clone https://github.com/lightning/bolts.git
	cd /tmp && git clone https://github.com/ElementsProject/lightning.git
	cd $(BOLTDIR)/bolts && tools/extract-formats.py 07-routing-gossip.md > bolt7.csv
	cp /tmp/lightning/gossipd/gossip_store_wire.csv gossip_map/spec/gossip_store_wire.csv
	cp /tmp/bolts/bolt7.csv gossip_map/spec/bolt7.csv
	patch gossip_map/spec/bolt7.csv gossip_map/spec/bolt7.diff
	@make update-genfile

update-genfile: ## update the local generated file without fetch a new specification, just the generated file.
	lncodegen-cli -l rust generate -b gossip_map/spec/gossip_store_wire.csv gossip_map/src/gossip_stor_wiregen.rs
	lncodegen-cli -l rust generate -b gossip_map/spec/bolt7.csv gossip_map/src/bolt7.rs


check_compiler: ## Check if the lncodegen exist or need to be installed (required the rust toolchain).
	@command -v lncodegen-cli --help >/dev/null 2>&1 || (echo "`lncodegen-cli` not found, installing..." && cargo install lncodegen-cli --git https://github.com/lnspec-tools/lncodegen.git)

help: ## Show Help
	@grep --no-filename -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-15s\033[0m %s\n", $$1, $$2}'
