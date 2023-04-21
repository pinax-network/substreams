ENDPOINT ?= mainnet.eth.streamingfast.io:443
STOP_BLOCK ?= +10

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream_graph
stream_graph: build
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -t $(STOP_BLOCK)

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONE: package
package: build
	substreams pack -o substreams.spkg substreams.yaml
