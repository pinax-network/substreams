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
	substreams run -e eos.firehose.eosnation.io:9001 prom_out -s 230078212 -t 230088212 -o jsonl --production-mode 

.PHONY: gui
gui:
	substreams gui -e eos.firehose.eosnation.io:9001 prom_out -s 230078212 -t 230088212 --production-mode 

.PHONY: sink
sink:
	substreams-sink-prometheus run eos.firehose.eosnation.io:9001
