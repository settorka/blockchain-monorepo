-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Table: block_builds  (used by Builder → DBPublisher)
CREATE TABLE IF NOT EXISTS block_builds (
    id SERIAL PRIMARY KEY,
    block_hash TEXT NOT NULL,
    included BOOLEAN,
    inclusion_reason TEXT,
    inclusion_latency_ms BIGINT,
    timestamp TIMESTAMPTZ DEFAULT NOW()
);

SELECT create_hypertable('block_builds', 'timestamp', if_not_exists => TRUE);

-- Table: bundles  (used by Relay to log searcher submissions)
CREATE TABLE IF NOT EXISTS bundles (
    id SERIAL PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    searcher TEXT,
    tx_count INT,
    target_block TEXT,
    arrival_time TIMESTAMPTZ DEFAULT NOW()
);

SELECT create_hypertable('bundles', 'arrival_time', if_not_exists => TRUE);

-- Table: simulations  (used by Simulator to record bundle outcomes)
CREATE TABLE IF NOT EXISTS simulations (
    id SERIAL PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    profit_eth DOUBLE PRECISION,
    latency_ms BIGINT,
    success BOOLEAN,
    reason TEXT,
    simulated_at TIMESTAMPTZ DEFAULT NOW()
);

SELECT create_hypertable('simulations', 'simulated_at', if_not_exists => TRUE);

-- Table: metrics (aggregated KPIs)
CREATE TABLE IF NOT EXISTS metrics (
    id SERIAL PRIMARY KEY,
    metric_name TEXT NOT NULL,
    metric_value DOUBLE PRECISION,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
);
SELECT create_hypertable('metrics', 'recorded_at', if_not_exists => TRUE);
