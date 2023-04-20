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
	substreams run -e telos.firehose.eosnation.io:9001 prom_out -s 273976005 -o jsonl

.PHONY: gui
gui:
	substreams gui -e telos.firehose.eosnation.io:9001 map_schedule_change -s 273976005 -t +10

.PHONY: sink
sink:
	substreams-sink-prometheus run -e eos.firehose.eosnation.io:9001 https://github.com/pinax-network/substreams/releases/download/eosmechanics-v0.3.5/eosmechanics-v0.3.5.spkg -s 292103130

