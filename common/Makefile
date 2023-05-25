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
	substreams run -e eos.firehose.eosnation.io:9001 kv_out -s 292442484 -t +100000 -o jsonl --production-mode

.PHONY: gui
gui:
	substreams gui -e eos.firehose.eosnation.io:9001 kv_out -s 292103130 -t +100000 --production-mode
