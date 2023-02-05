# Antelope `eosmechanics` Substream

> Block Producer Benchmarks created by [AlohaEOS](https://www.alohaeos.com/tools/benchmarks).

### Quickstart - **Substreams**

```

```

### Quickstart

```
substreams gui -e eos.firehose.eosnation.io:9001 https://github.com/pinax-network/substreams/releases/download/eosmechanics-v0.2.0/eosmechanics-v0.2.0.spkg map_producer_usage -s 292103130 -t +100000
```

### Build, Run & Sink from source

```bash
$ make
$ make run
$ make sink
```

### **Prometheus** Sink

1. Start streaming: `$ npm start`
2. Open the browser at [http://localhost:9102/metrics](http://localhost:9102/metrics)

### Mermaid graph

```mermaid
graph TD;
  map_producer_usage[map: map_producer_usage]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_producer_usage
  map_prom_out[map: map_prom_out]
  map_producer_usage --> map_prom_out
```

<img width="832" alt="image" src="https://user-images.githubusercontent.com/550895/216176638-cea94a43-f95e-4eb6-ae00-527a2cb02ab7.png">

<img width="841" alt="image" src="https://user-images.githubusercontent.com/550895/216177257-6dab708d-870f-4296-9d72-456e6b2f2b77.png">

### Modules

```yaml
Package name: eosmechanics
Version: v0.2.0
Doc: Block Producer Benchmarks
Modules:
----
Name: map_producer_usage
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.ProducerUsage
Hash: 0a50cc9be5aa717d6624dbe45e4138d3b794ecfb

Name: prom_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.prometheus.v1.PrometheusOperations
Hash: ecc2b0e082cc1975392b79cff38ca5bb16931e6d
```