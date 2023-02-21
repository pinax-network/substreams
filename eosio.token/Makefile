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
	substreams run -e eos.firehose.eosnation.io:9001 map_transfers -s 292442484 -t +1000000 -o jsonl

.PHONY: map_accounts
map_accounts:
	substreams gui -e eos.firehose.eosnation.io:9001 map_accounts -s 292442484 -t +1000000

.PHONY: map_stat
map_stat:
	substreams gui -e eos.firehose.eosnation.io:9001 map_stat -s 292442484 -t +1000000

.PHONY: gui
gui:
	substreams gui -e eos.firehose.eosnation.io:9001 map_transfers -s 292103130 -t +1000000

.PHONY: sink
sink:
	substreams run -e eos.firehose.eosnation.io:9001 log_out -s 292442484 -t +1000000 -o jsonl

.PHONY: db_out
db_out:
	substreams gui -e eos.firehose.eosnation.io:9001 db_out -s 292442484 -t +1000000