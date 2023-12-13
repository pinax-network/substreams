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
	substreams protogen --exclude-paths sf/substreams,google,sf/antelope/type

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
	substreams run -e eos.substreams.pinax.network:443 map_action_traces -s -1

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 map_action_traces -s -1

.PHONY: params
params:
	substreams run -e eos.substreams.pinax.network:443 map_action_traces -s -1 -p map_action_traces=receiver=eosio.token

.PHONY: sink
sink:
	substreams gui -e eos.substreams.pinax.network:443 kv_out -s -1