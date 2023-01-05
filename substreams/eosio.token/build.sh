#!/bin/bash

substreams protogen ./substreams.yaml --exclude-paths="sf/antelope,sf/substreams,google"
cargo build --target wasm32-unknown-unknown --release
substreams pack ./substreams.yaml
substreams info $(ls *.spkg)
substreams graph $(ls *.spkg)