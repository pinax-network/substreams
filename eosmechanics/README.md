# Antelope `eosmechanics` Substream

> Block Producer Benchmarks created by [AlohaEOS](https://www.alohaeos.com/tools/benchmarks).

### [Latest Releases](https://github.com/pinax-network/substreams/releases)

### Quickstart

```bash
$ make
$ make run
$ make sink
```

### Sinks
- [Prometheus](https://github.com/pinax-network/substreams-sink-prometheus.rs)

### Mermaid graph

```mermaid
graph TD;
  map_producer_usage[map: map_producer_usage]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_producer_usage
  map_schedule_change[map: map_schedule_change]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_schedule_change
  prom_out[map: prom_out]
  map_producer_usage --> prom_out
  map_schedule_change --> prom_out
```

<img width="832" alt="image" src="https://user-images.githubusercontent.com/550895/216176638-cea94a43-f95e-4eb6-ae00-527a2cb02ab7.png">

<img width="841" alt="image" src="https://user-images.githubusercontent.com/550895/216177257-6dab708d-870f-4296-9d72-456e6b2f2b77.png">

### Modules

```yaml
Package name: eosmechanics
Version: v0.3.5
Doc: Block Producer Benchmarks
Modules:
----
Name: map_producer_usage
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.ProducerUsage
Hash: d17b3f70424913ba557036e0c9daaa1f2b0f9719

Name: map_schedule_change
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.ScheduleChange
Hash: 2273b378c36ffd12294911b05f733298ac186556

Name: prom_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.prometheus.v1.PrometheusOperations
Hash: 62397f52f2f2b33041ad6f0a3100d3c718496fa7
```