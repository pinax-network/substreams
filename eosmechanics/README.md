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
Version: v0.3.4
Doc: Block Producer Benchmarks
Modules:
----
Name: map_producer_usage
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.ProducerUsage
Hash: daaafefb647b0238139a48f2e54c1ef423b42406

Name: map_schedule_change
Initial block: 0
Kind: map
Output Type: proto:eosmechanics.v1.ScheduleChange
Hash: b9deac907bccc0f249dddd00614109e7b8fc834b

Name: prom_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.prometheus.v1.PrometheusOperations
Hash: fe1c9145a7a314dc668e94896ef148d0dd434945
```