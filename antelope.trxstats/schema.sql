CREATE TABLE
    IF NOT EXISTS transactions (
        "id" VARCHAR(16) PRIMARY KEY,
        "trx_id" VARCHAR(64),
        "block_num" BIGINT,
        "block_time" TIMESTAMP,
        "status" INT,
        "action_count" INT,
        "elapsed" INT,
        "net_usage" INT,
        "cpu_usage_micro_seconds" INT,
        "net_usage_words" INT
    );

-- Add indexes
CREATE INDEX
    idx_cpu_usage ON transactions ("cpu_usage_micro_seconds");

CREATE INDEX idx_net_usage_words ON transactions ("net_usage_words");

CREATE INDEX idx_action_count ON transactions ("action_count");

CREATE INDEX idx_trx_id ON transactions ("trx_id");