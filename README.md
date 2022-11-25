# EOS-Nation Substreams monorepo

This repository holds all Substreams, libraries and consumers for the EOS-Nation Substreams run on Antelope chains. Its 
goal is to centralize build tools, documentation and shared code among our Substream implementations. 

It can also be used to discuss new Substream proposals.

For a current list of available endpoints [see here](#available-endpoints).

Further resources:
* [Substreams documentation](https://substreams.streamingfast.io)


## Repository structure

TODO explain repository structure

## Prerequisites

Before starting to work with Substreams, you need to have Rust, the Substreams CLI and buf installed. 

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

## Building Substreams

To build any of the available Substreams in the `./substreams` directory you can use the Makefile by running 
`make build SUBMODULE=<submodule>`. Alternatively you can change into the submodule directory and run 
`cargo build --target wasm32-unknown-unknown --release`.

You can also execute the `./build-all.sh` script in case you want to build all available substreams.

## Running Substreams

To execute a Substream on the server you need to use the CLI and specify the substream.yaml file you want to execute, the
endpoint to execute the Substream on and the store method. 

For example executing the exemplary `blocktime-meta` Substream you need to run:

`substreams run -e waxtest.firehose.eosnation.io:9001 ./submodules/blocktime-meta/substreams.yaml store_blockmeta`

In case you want to execute the Substream on a specific interval you can specify the start block using `-s <start_block_num>`
and end block using `-t <end_block_num>`. The endblock can also be specified as range using `+`. That means using `-t +1000`
will run the substreams for 1000 blocks from the start block.

### Available endpoints

* `waxtest.firehose.eosnation.io:9001`


## Running consumers & sinks

TODO describe how to run substreams from a node/go process and how to run the available sinks (file, MongoDB, graph-node)

## Creating new Substreams

TODO describe how to create new substreams


## Contributing

TODO


## License

[Apache 2.0](LICENSE)
