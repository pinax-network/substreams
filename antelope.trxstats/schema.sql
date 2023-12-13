CREATE TABLE
    IF NOT EXISTS transactions (
        "trx_id" VARCHAR(64) PRIMARY KEY,
        "block_num" BIGINT,
        "block_time" TIMESTAMP,
        "action_count" INT,
        "cpu" INT,
        "net" INT
    );