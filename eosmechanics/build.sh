#!/bin/bash

substreams protogen --exclude-paths="google,sf/substreams"
cargo build --target wasm32-unknown-unknown --release
substreams pack
substreams graph
substreams info
