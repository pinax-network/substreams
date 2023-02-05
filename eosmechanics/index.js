import { Substreams, download } from "substreams";
import { gauges, register, listen } from "./src/metrics.js"

// Substreams using live data
const spkg = "https://github.com/pinax-network/substreams/releases/download/eosmechanics-v0.2.0/eosmechanics-v0.2.0.spkg";
const outputModule = "prom_out";
const startBlockNum = "292442484";
const host = "eos.firehose.eosnation.io:9001"

// Initialize Substreams
const substreams = new Substreams(outputModule, {
    host,
    startBlockNum,
    authorization: process.env.STREAMINGFAST_KEY // or SUBSTREAMS_API_TOKEN
});

// Prometheus Exporter
listen(9102).then(async () => {
    // download Substream from IPFS
    const { modules, registry } = await download(spkg);

    // Find Protobuf message types from registry
    const PrometheusOperations = registry.findMessage("pinax.substreams.sink.prometheus.v1.PrometheusOperations");
    if (!PrometheusOperations) throw new Error("Could not find PrometheusOperations message type");

    let block_num = 0;
    substreams.on("block", block => {
        block_num = Number(block.clock.number);
    });

    substreams.on("mapOutput", output => {
        if (output.name !== "prom_out") return;
        const decoded = PrometheusOperations.fromBinary(output.data.mapOutput.value);
        
        // Prometheus metrics
        for ( const { labels, value, type } of decoded.operations ) {
            console.log(block_num, { labels, value, type });
            // SET
            if (type === 1) gauges.producer_usage.labels(labels[0]).set(value);
            // RESET
            if (type === 2) gauges.producers_usage.reset(labels[0]);
        }
    });

    // start streaming Substream
    await substreams.start(modules);
});