#!/bin/bash

substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google" 
cargo build --target wasm32-unknown-unknown --release
substreams pack
substreams graph
substreams info
