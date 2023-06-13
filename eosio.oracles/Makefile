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
	substreams run -e eos.firehose.eosnation.io:9001 map_pairs -s -1

.PHONY: delphioracle
delphioracle:
	substreams gui -e eos.firehose.eosnation.io:9001 map_datapoints -s -1

.PHONY: oracle_defi
oracle_defi:
	substreams gui -e eos.firehose.eosnation.io:9001 map_prices -s -1
