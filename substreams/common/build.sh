#!/bin/bash

cargo build --target wasm32-unknown-unknown --release
substreams pack ./substreams.yaml
substreams info $(ls *.spkg)
substreams graph $(ls *.spkg)