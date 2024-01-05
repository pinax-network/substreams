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
	substreams run -e eos.substreams.pinax.network:443 map_logs -s 350410392 -t +300

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 map_logs -s 350410392 -t +300

.PHONY: sql
sql:
	substreams gui -e eos.substreams.pinax.network:443 db_out -s 350410392 -t +300

