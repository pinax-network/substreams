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
	substreams run -e eos.substreams.pinax.network:443 map_deferred -s 1005179 -t +1

.PHONY: csv
csv:
	substreams run -e eos.substreams.pinax.network:443 csv_out -s 1005179 -t +1

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 map_deferred -s 1000000 -t +100000
