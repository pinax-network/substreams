#!/bin/bash

ANTELOPE_SPKG="${ANTELOPE_SPKG:-https://github.com/EOS-Nation/firehose-antelope/releases/download/v0.0.1/antelope-v0.0.1.spkg}"

echo "Generating Antelope Protobuf using $ANTELOPE_SPKG"
substreams protogen "$ANTELOPE_SPKG" --exclude-paths="sf/substreams/v1,google/" --output-path="./core/src/pb"
