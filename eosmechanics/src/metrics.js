import client from "prom-client";
import http from "node:http";

// Prometheus Exporter
const register = new client.Registry();
const collectDefaultMetrics = client.collectDefaultMetrics;
collectDefaultMetrics({ register });

const producers_usage = new client.Gauge({
    name: "producers_usage",
    help: "Producers CPU Usage",
});
register.registerMetric(producers_usage);

const producer_usage = new client.Gauge({
    name: "producer_usage",
    help: "Producer CPU Usage",
    labelNames: ["producer"]
});
register.registerMetric(producer_usage);

export const gauges = {
    producers_usage,
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