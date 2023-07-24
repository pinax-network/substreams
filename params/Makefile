.PHONY: all
all:
	make build
	make pack

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack

.PHONY: run
run:
	substreams run map_params -s 1 -t +1 -p "map_params=custom run params 🔥"

.PHONY: gui
gui:
	substreams gui map_params -s 1 -t +1 -p "map_params=custom gui params 🔥"
