.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

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
	substreams run -e eos.firehose.eosnation.io:9001 map_action_traces -s 292442484 -t +100000 -o jsonl

.PHONY: gui
gui:
	substreams gui -e eos.firehose.eosnation.io:9001 map_action_traces -s 292103130 -t +100000
