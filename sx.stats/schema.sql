CREATE TABLE
    IF NOT EXISTS trades (
        "id" VARCHAR(16) PRIMARY KEY,
        "trx_id" VARCHAR(64),
        "block_num" BIGINT,
        "block_time" TIMESTAMP,
        "elapsed" INT,
        "producer" VARCHAR(16),
        "contract" VARCHAR(16),
        "executor" VARCHAR(16),
        "borrow" VARCHAR(24),
        "profit" VARCHAR(24),
        "profit_symbol" VARCHAR(8),
        "profit_value" DECIMAL(8, 4),
        "codes" VARCHAR(256),
        "path" VARCHAR(256),
        "hops" INT,
    );

-- Add indexes
CREATE INDEX idx_profit ON trades ("profit");