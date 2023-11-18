CREATE TABLE IF NOT EXISTS quotes (
    q_pair String,
    q_id String,
    q_median String,
    q_owner String,
    q_timestamp String,
    q_value String
)
ENGINE = MergeTree()
ORDER BY (q_pair)