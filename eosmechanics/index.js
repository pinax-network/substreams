import { Substreams, download } from "substreams";
import { gauges, listen } from "./src/metrics.js"

// Substreams using live data
const version = "v0.3.0";
const spkg = `https://github.com/pinax-network/substreams/releases/download/eosmechanics-${version}/eosmechanics-${version}.spkg`;
const outputModule = "prom_out";
const startBlockNum = "288786307";
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
        for ( const  { metric, name, operation, value, labels } of decoded.operations ) {
            console.log({metric, operation, name, value, labels });
            // SET
            if (operation === 1) {
                if ( labels.length ) gauges[name].labels({producer: labels[0]}).set(value);
                else gauges[name].set(value);
            }
            // INC
            if (operation === 2) gauges[name].inc();
            // // RESET
            // if (type === 2) gauges[metric].inc(label);
        }
    });

    // start streaming Substream
    await substreams.start(modules);
});