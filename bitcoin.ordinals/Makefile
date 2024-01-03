.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	AR=/usr/local/opt/llvm/bin/llvm-ar CC=/usr/local/opt/llvm/bin/clang cargo build --target wasm32-unknown-unknown --release

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
	substreams run -e bitcoin.substreams.pinax.network:443 map_inscriptions -s 780235 -t +1

.PHONY: gui
gui:
	substreams gui -e bitcoin.substreams.pinax.network:443 map_inscriptions -s 780235 -t +1
