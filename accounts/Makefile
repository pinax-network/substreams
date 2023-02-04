.PHONY: all
all:
	make codegen
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: codegen
codegen:
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
	substreams gui -e eos.firehose.eosnation.io:9001 map_accounts -s 1000000 -t +10000

.PHONY: sink
sink:
	substreams-sink-kv run badger3://badger_data.db eos.firehose.eosnation.io:9001 substreams.yaml kv_out