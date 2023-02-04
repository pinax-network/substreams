# Antelope `accounts` Substream

> Newly created accounts on Antelope

### Releases

[v0.1.0](https://github.com/pinax-network/substreams/releases/tag/accounts-v0.1.0)

### Quickstart

```
substreams gui -e eos.firehose.eosnation.io:9001 https://github.com/pinax-network/substreams/releases/download/accounts-v0.1.0/accounts-v0.1.0.spkg map_accounts -s 1000 -t +10000
```

### Build, Run & Sink from source

```bash
$ make
$ make run
$ make sink
```

### Graph

```mermaid
graph TD;
  map_accounts[map: map_accounts]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_accounts
  kv_out[map: kv_out]
  map_accounts --> kv_out
```

### Modules

```yaml
Package name: accounts
Version: v0.1.0
Doc: Newly created accounts on Antelope
Modules:
----
Name: map_accounts
Initial block: 0
Kind: map
Output Type: proto:antelope.accounts.v1.Accounts
Hash: c2913de037b623e79746988daef50c7a97b8b546

Name: kv_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.kv.v1.KVOperations
Hash: 1eed02da1523ba80a21a56fdb3f137e41f366530
```
