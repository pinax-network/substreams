CREATE TABLE
    IF NOT EXISTS transactions (
        "id" SERIAL PRIMARY KEY,
        "trx_id" VARCHAR(64) UNIQUE,
        "block_num" BIGINT UNSIGNED,
        "timestamp" TIMESTAMP,
        "action_count" INT UNSIGNED,
        "cpu" INT,
        "net" INT UNSIGNED
    );