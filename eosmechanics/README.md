# Antelope `eosmechanics` Substream

> Block Producer Benchmarks created by [AlohaEOS](https://www.alohaeos.com/tools/benchmarks).

### Quickstart

```
$ substreams run -e eos.firehose.eosnation.io:9001 map_producer_usage -s 292180468 -t +1
```

### Mermaid graph

```mermaid
graph TD;
  map_producer_usage[map: map_producer_usage]
  sf.antelope.type.v2.Block[source: sf.antelope.type.v2.Block] --> map_producer_usage
  map_prom_out[map: map_prom_out]
  map_producer_usage --> map_prom_out
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
Name: map_producer_usage
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.ProducerUsage
Hash: 7c6074773130dc85c3022d6b56fca8f21f06d8cf

Name: map_prom_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sinks.prom.v1.PrometheusMetrics
Hash: a27326168962800fae976825c6173a6539cd8702
```