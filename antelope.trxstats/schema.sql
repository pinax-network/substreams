CREATE TABLE
    IF NOT EXISTS transactions (
        "id" VARCHAR(16) PRIMARY KEY,
        "trx_id" VARCHAR(64),
        "block_num" BIGINT,
        "block_time" TIMESTAMP,
        "status" INT,
        "action_count" INT,
        "cpu_elapsed_us" INT,
        "net_elapsed_bytes" INT,
        "cpu_usage_us" INT,
        "net_usage_bytes" INT
    );

-- Add indexes
CREATE INDEX idx_cpu_elapsed_us ON transactions ("cpu_elapsed_us");

CREATE INDEX idx_cpu_usage_us ON transactions ("cpu_usage_us");

CREATE INDEX idx_action_count ON transactions ("action_count");