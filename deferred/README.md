# Antelope `eosio.deferred` Substream

> Antelope deferred transactions

## Usage

To dump all transactions into files use [substreams-sink-files](https://github.com/streamingfast/substreams-sink-files) sink, i.e.
```bash
substreams-sink-files run eos.substreams.pinax.network:443 substreams.yaml csv_out ./data 328_000_000:328_620_000 --encoder=lines -c=10000
```