.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e api-unstable.streamingfast.io:443 map_params -t +10

.PHONY: gui
gui:
	substreams gui -e api-unstable.streamingfast.io:443 map_params -t +10
