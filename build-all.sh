#!/bin/bash

cargo build --target wasm32-unknown-unknown --release

# Pack substreams
substreams pack ./substreams/blocktivity/substreams.yaml
substreams pack ./substreams/blocktime-meta/substreams.yaml
substreams pack ./substreams/tokens/substreams.yaml
