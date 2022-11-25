SUBMODULE := *

.PHONY: build
build:
	cargo build --manifest-path=./submodules/$(SUBMODULE)/Cargo.toml --target wasm32-unknown-unknown --release

.PHONY: codegen
codegen:
	substreams protogen ./submodules/$(SUBMODULE)/substreams.yaml --exclude-paths="sf/substreams,google"
