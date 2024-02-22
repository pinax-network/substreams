ENDPOINT ?= goerli-scdm37.mar.eosn.io:10016

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONE: package
package: build
	substreams pack -o substreams.spkg substreams.yaml

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) map_blobs -s -10 --plaintext