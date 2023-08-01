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
	substreams run -e eos.firehose.eosnation.io:9001 map_actions -s 317328003 -t +10 -p "map_actions=d.w.pomelo"

.PHONY: gui
gui:
	substreams gui -e eos.firehose.eosnation.io:9001 map_actions -s 317328003 -t +10000 -p "map_actions=d.w.pomelo"

.PHONY: claim
claim:
	substreams gui -e eos.firehose.eosnation.io:9001 map_actions -s 316013128 -t +10000 -p "map_actions=d.w.pomelo"
