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
- [Prometheus](https://github.com/pinax-network/substreams-sink-prometheus)

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
Version: v0.3.3
Doc: Block Producer Benchmarks
Modules:
----
Name: map_producer_usage
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.ProducerUsage
Hash: 4c74215a363f0f40413132a61464bb7a7ebe4e39

Name: map_schedule_change
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.ScheduleChange
Hash: 6e50d4674abdb3c3c0766e394decdb11ebfd2f94

Name: prom_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.prometheus.v1.PrometheusOperations
Hash: b08bc360f9ebceeee0ddc58e7d428c2d88d41aff
```