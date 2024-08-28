CREATE TABLE IF NOT EXISTS block_meta (
    "id" VARCHAR(64),
    "number" UInt64,
    "timestamp" VARCHAR(64)
    ) ENGINE = MergeTree PRIMARY KEY ("id");