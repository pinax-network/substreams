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
	substreams run -e eos.firehose.eosnation.io:9001 prom_out -s 292442484 -t +100000 -o jsonl

.PHONY: gui
gui:
	substreams gui -e eos.firehose.eosnation.io:9001 prom_out -s 292103130 -t +100000

.PHONY: sink
sink:
	substreams-sink-prometheus run -e eos.firehose.eosnation.io:9001 https://github.com/pinax-network/substreams/releases/download/eosmechanics-v0.3.5/eosmechanics-v0.3.5.spkg -s 292103130

