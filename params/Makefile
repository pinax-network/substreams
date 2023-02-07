.PHONY: all
all:
	make build
	make pack

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack

.PHONY: run
run:
	substreams run -e api-unstable.streamingfast.io:443 map_params -s 2 -t +2
