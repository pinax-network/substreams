# EOS-Nation Substreams monorepo

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/EOS-Nation/substreams-monorepo/blob/main/LICENSE)
[![Node.js Substreams](https://github.com/EOS-Nation/substreams-monorepo/actions/workflows/node-consumer.yml/badge.svg)](https://github.com/EOS-Nation/substreams-monorepo/actions/workflows/node-consumer.yml)
[![Rust Substreams](https://github.com/EOS-Nation/substreams-monorepo/actions/workflows/rust-substreams.yml/badge.svg)](https://github.com/EOS-Nation/substreams-monorepo/actions/workflows/rust-substreams.yml)

This repository holds all Substreams, libraries and consumers for the EOS-Nation Substreams run on Antelope chains. Its 
goal is to centralize build tools, documentation and shared code among our Substream implementations. 

It can also be used to discuss new Substream proposals.

### Firsehose V2

| Chain       | Host     |
|-------------|----------|
| EOS         | `eos.firehose.eosnation.io:9001`
| WAX         | `wax.firehose.eosnation.io:9001`
| Ore         | `ore.firehose.eosnation.io:9001`
| Telos       | `telos.firehose.eosnation.io:9001`

### Firsehose V2 (Testnets)

| Chain         | Host     |
|---------------|----------|
| WAX Testnet   | `waxtest.firehose.eosnation.io:9001`
| Jungle 4      | `jungle4.firehose.eosnation.io:9001`
| Kylin         | `kylin.firehose.eosnation.io:9001`
| Ore Stage     | `orestage.firehose.eosnation.io:9001`
| Telos Testnet | `telostest.firehose.eosnation.io:9001`

## Substreams

| Substream | Description |
|-----------|-------------|
| [`tokens`](substreams/tokens)  | Extracts token supply from Antelope `eosio.token` based tokens. |

### Further resources

- [Substreams documentation](https://substreams.streamingfast.io)

## Repository structure

This repository currently holds all substreams as well as the SDK library. 

+ `core` holds the Substream core SDK for Antelope chains. This contains the generated protobuffer for the Antelope blocks
  as well as helper methods to extract and parse block data.
* `substreams` holds all the substreams bundled into their own subfolder.

## Prerequisites

Before starting to work with Substreams, you need to have `Rust` and the `Substreams CLI` installed. To create new substreams
you also need to have `buf` available (to generate Rust code from you proto buffers). In case you want to utilize different
sinks you might also need to have `Go` installed.

### Rust

Install rust via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. This will install the `rustup`
toolchain into `~/.cargo` including tools like cargo (package manager). Run `rustup update` to keep your rust dependencies
up to date. 

To have this properly included into your path directory, add `source $HOME/.cargo/env` to your shell profile (for example
to `~/.zshrc` if you are using zsh).

See the [Rust website](https://www.rust-lang.org/tools/install) for further information about the installation.


### Substreams CLI

To be able to execute Substreams you need to install the Substreams CLI. MacOS users can just do this using Homebrew by
executing `brew install streamingfast/tap/substreams`. 

Other users can get the executable from the Substreams [Github repository](https://github.com/streamingfast/substreams):

```shell
# Use correct binary for your platform
LINK=$(curl -s https://api.github.com/repos/streamingfast/substreams/releases/latest | awk '/download.url.*linux/ {print $2}' | sed 's/"//g')
curl -L  $LINK  | tar zxf -
```

Check the [substreams documentation](https://substreams.streamingfast.io/getting-started/installing-the-cli) for more information about installing the CLI.


### buf

To generate Rust code from protobuffers you need to install buf (this is not required if you only want to build and 
run the available Substreams in this repository). Again MacOS users can install those using Homebrew by executing
`brew install bufbuild/buf/buf`.

Otherwise, you can get the binaries from the [Github repository](https://github.com/bufbuild/buf):

```shell
# Substitute BIN for your bin directory.
# Substitute VERSION for the current released version.
BIN="/usr/local/bin" && \
VERSION="1.9.0" && \
  curl -sSL \
      "https://github.com/bufbuild/buf/releases/download/v${VERSION}/buf-$(uname -s)-$(uname -m)" \
      -o "${BIN}/buf" && \
  chmod +x "${BIN}/buf"
```

For more information about the installation check the [buf website](https://docs.buf.build/installation).

### Go

To run the available sinks from the Streamingfast team you currently need to have Go installed (until they release) 
binaries. 

MacOS users can again use Homebrew to install Go running `brew install go`. Linux and Windows users should follow the 
[official installation](https://go.dev/doc/install).

Make sure you have proper `PATH` variables set in your shell profile (for example `.zshrc` for zsh users):

```shell
export GOPATH=$HOME/go
export GOBIN=$GOPATH/bin
export PATH=$PATH:$GOPATH
export PATH=$PATH:$GOBIN
```

## Building Substreams

To build any of the available Substreams in the `./substreams` directory you can use the Makefile by running 
`make build SUBSTREAM=<substream>`. Alternatively you can change into the Substream directory and run 
`cargo build --target wasm32-unknown-unknown --release`.

You can also execute the `./build-all.sh` script in case you want to build all available substreams.

## Running Substreams

To execute a Substream on the server you need to use the CLI and specify the `substream.yaml` file you want to execute, the
endpoint to execute the Substream on and the store method. 

For example executing the exemplary `blocktime-meta` Substream you need to run:

`substreams run -e waxtest.firehose.eosnation.io:9001 ./substreams/blocktime-meta/substreams.yaml store_blockmeta`

In case you want to execute the Substream on a specific interval you can specify the start block using `-s <start_block_num>`
and end block using `-t <end_block_num>`. The endblock can also be specified as range using `+`. That means using `-t +1000`
will run the substreams for 1000 blocks from the start block.

For a current list of available endpoints [see here](#available-endpoints).

## Packing Substreams

In case you want to run a Substream on a sink you need to pack your substream. This will create a `*.spkg` bundle file.

This can be easily done by running `make pack SUBSTREAM=<mysubstream>`.

## Running consumers & sinks

TODO describe how to run substreams from a node/go process and how to run the available sinks (file, MongoDB, graph-node)

### Running sinks

There are sinks available from the Streamingfast team, currently those are the [file](https://github.com/streamingfast/substreams-sink-files), 
[PostgresSQL](https://github.com/streamingfast/substreams-sink-postgres) and [MongoDB](https://github.com/streamingfast/substreams-sink-mongodb) 
sink.

#### File sink

Install the sink by running `go install github.com/streamingfast/substreams-sink-files/cmd/substreams-sink-files@latest`.

### Writing consumers

TODO describe how to write custom consumers for substreams in different languages.

## Creating new Substreams

This chapter will give you a brief understanding what to do if you want to create a new substream. 

### Setup the codebase

To create a new Substream you need to first create a new code base:

```shell
cargo new substreams/<mysubstream> --lib
```

Next you want to make sure that we will be able to compile the substreams and generate the relevant models for them. For
that you'll need a `substreams.yaml` file. Copy this over from another substream and adapt it to your needs.

### Create the models

You probably want to start now by defining the output models for your maps so you can write the transformation from a 
full antelope Block to the data you actually need within your substreams. These are written as protobuffers and should 
be located within the `proto` folder in your substreams directory. See this [protofile](https://github.com/EOS-Nation/substreams-monorepo/blob/develop/substreams/blocktime-meta/proto/block.proto) 
as an example.

After you have defined your models you want to generate the Rust models from your protobuffers. You can do this by 
executing `make codegen SUBSTREAM=<mysubstream>`. This will generate the Rust code into the `src/pb` folder in your
Substream module. You need to now add a `mod.rs` in that folder to export the code, see [here](https://github.com/EOS-Nation/substreams-monorepo/blob/develop/substreams/blocktime-meta/src/pb/mod.rs) for an example.

### Write the transformers and stores

To write the actual code you first want to create your maps. Maps will transform an input format into some output format.
The first map will receive the full antelope block format (including all block headers and all transactions). Its job 
is it to filter out the relevant data and output it into one of the custom models you defined a step above. 

The stores will then receive the outputs from the maps and store them into one of the predefined KV stores. Those are 
providing different update policies (such as `set` or `add` for example) for different data types. So to add up int64
values on a specific key you would use a `StoreAddInt64`. The available stores can be found [here](https://github.com/streamingfast/substreams-rs/blob/develop/substreams/src/store.rs).

A simple example on how to map and store blocks can be found [here](https://github.com/EOS-Nation/substreams-monorepo/blob/develop/substreams/blocktime-meta/src/lib.rs).
And more information about developing Substreams can be found on the [official documentation](https://substreams.streamingfast.io/developer-guide/overview).

**Note:** Make sure the input / output data types you use in your maps and stores match the ones you have defined in your 
`substreams.yaml` file.

## Contributing

TODO


## License

[Apache 2.0](LICENSE)
