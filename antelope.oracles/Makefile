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
	substreams gui -e eos.firehose.eosnation.io:9001 map_quotes -s -1

.PHONY: oracle.defi
oracle.defi:
	substreams gui -e eos.firehose.eosnation.io:9001 map_prices -s -1

.PHONY: kv_out
kv_out:
	substreams gui -e eos.firehose.eosnation.io:9001 kv_out -s -1

.PHONY: db_out
db_out:
	substreams gui -e eos.firehose.eosnation.io:9001 db_out -s -1