# `AtomicMarket` Substream

> Metrics for [AtomicHub Market](https://eos.atomichub.io/).

### [Latest Releases](https://github.com/pinax-network/substreams/releases)

### Sinks
- [Prometheus](https://github.com/pinax-network/substreams-sink-prometheus.rs)

### Quickstart

```bash
$ make
$ make run
$ make sink
```

### Mermaid graph

```mermaid
graph TD;
  map_actions[map: map_actions]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_actions
  prom_out[map: prom_out]
  map_actions --> prom_out
```

### Modules

```yaml
Package name: atomicmarket
Version: v0.1.1
Doc: Metrics for AtomicHub Market
Modules:
----
Name: map_actions
Initial block: 0
Kind: map
Output Type: proto:sf.antelope.type.v1.ActionTraces
Hash: 5949d1d368f6d8d7eb254fce5c209b42556303a2

Name: prom_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.prometheus.v1.PrometheusOperations
Hash: 9e13c218d347bc1017b4c59a2783182c86d97fbe
```