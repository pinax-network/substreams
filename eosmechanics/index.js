import { Substreams, download } from "substreams";
import { gauges, listen } from "./src/metrics.js"

// Substreams using live data
const spkg = "https://github.com/pinax-network/substreams/releases/download/v0.1.0/eosmechanics-v0.1.0.spkg";
const outputModule = "map_producer_usage";
const startBlockNum = "292352668";
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
    const {modules, registry} = await download(spkg);

    // Find Protobuf message types from registry
    const ProducerUsage = registry.findMessage("eosmechanics.v1.ProducerUsage");
    if ( !ProducerUsage) throw new Error("Could not find ProducerUsage message type");

    substreams.on("mapOutput", output => {
        const decoded = ProducerUsage.fromBinary(output.data.mapOutput.value);
        console.log(decoded);
        gauges.producer_usage.inc(Number(decoded.cpuUsage));
    });

    // start streaming Substream
    await substreams.start(modules);
});