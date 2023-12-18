CREATE TABLE
    IF NOT EXISTS transactions (
        "trx_id" VARCHAR(64) PRIMARY KEY,
        "block_num" BIGINT,
        "block_time" TIMESTAMP,
        "status" INT,
        "action_count" INT,
        "elapsed" INT,
        "net_usage" INT,
        "cpu_usage_micro_seconds" INT,
        "net_usage_words" INT
    );