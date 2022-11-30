SUBSTREAM := *

.PHONY: build
build:
	cargo build --manifest-path=./substreams/$(SUBSTREAM)/Cargo.toml --target wasm32-unknown-unknown --release

.PHONY: codegen
codegen:
	substreams protogen ./substreams/$(SUBSTREAM)/substreams.yaml --exclude-paths="sf/antelope,sf/substreams,google"

.PHONY: pack
pack:
	substreams pack ./substreams/$(SUBSTREAM)/substreams.yaml
