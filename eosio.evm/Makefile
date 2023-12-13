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
	substreams protogen --exclude-paths sf/antelope/type,sf/substreams,google

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
	substreams run -e eos.substreams.pinax.network:443 map_pushtx -s 311800000 -t +1000000 -o jsonl

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 map_pushtx -s 311800000 -t +1000000
