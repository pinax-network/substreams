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
	substreams run -e eos.substreams.pinax.network:443 map_accounts -s 241176757 -t +1

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 map_accounts -s 241176757 -t +10000

.PHONY: sink
sink:
	substreams-sink-kv run badger3://badger_data.db eos.substreams.pinax.network:443 substreams.yaml kv_out