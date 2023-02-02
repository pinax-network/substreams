# Antelope `eosmechanics` Substream

> Block Producer Benchmarks created by [AlohaEOS](https://www.alohaeos.com/tools/benchmarks).

### Quickstart

```
$ substreams run -e eos.firehose.eosnation.io:9001 map_block_stats -t +10
```

### Mermaid graph

```mermaid
graph TD;
  map_block_stats[map: map_block_stats]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_block_stats
```

<img width="832" alt="image" src="https://user-images.githubusercontent.com/550895/216176638-cea94a43-f95e-4eb6-ae00-527a2cb02ab7.png">

<img width="841" alt="image" src="https://user-images.githubusercontent.com/550895/216177257-6dab708d-870f-4296-9d72-456e6b2f2b77.png">

### Modules

```yaml
Package name: eosmechanics
Version: v0.1.0
Doc: Block Producer Benchmarks
Modules:
  ----
Name: map_block_stats
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.BlockStats
Hash: e71870fcf747d120a130501211e1ad8770cc44b8
```