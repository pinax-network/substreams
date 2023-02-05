import client from "prom-client";
import http from "node:http";

// Prometheus Exporter
const register = new client.Registry();
const collectDefaultMetrics = client.collectDefaultMetrics;
collectDefaultMetrics({ register });

const action_count = new client.Gauge({
    name: "action_count",
    help: "Total [eosmechanics] CPU actions",
});
register.registerMetric(action_count);

const schedule_version = new client.Gauge({
    name: "schedule_version",
    help: "Schedule version",
});
register.registerMetric(schedule_version);

const producer_usage = new client.Gauge({
    name: "producer_usage",
    help: "Producer CPU usage",
    labelNames: ["producer"]
});
register.registerMetric(producer_usage);

export const gauges = {
    action_count,
    schedule_version,
    producer_usage
}

// Create a local server to serve Prometheus gauges
const server = http.createServer(async (req, res) => {
    res.writeHead(200, { 'Content-Type': register.contentType });
    res.end(await register.metrics());
});

export async function listen(port) {
    return new Promise(resolve => {
        server.listen(port, "0.0.0.0", () => {
            console.log('starting prometheus metrics server', { port });
            resolve();
        });
    })
}