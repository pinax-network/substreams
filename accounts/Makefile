.PHONY: codegen
codegen:
	substreams protogen substreams.yaml --exclude-paths sf/antelope,sf/substreams,google

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack substreams.yaml

.PHONY: stream
stream:
	substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml map_accounts -s 1000 -t +10

.PHONY: stream_kv
stream_kv:
	substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml kv_out -s 1000 -t +10

.PHONY: graph
graph:
	substreams graph substreams.yaml

.PHONY: info
info:
	substreams info substreams.yaml