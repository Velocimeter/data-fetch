CREATE TABLE IF NOT EXISTS o_token_datas (
  chain_id            INTEGER PRIMARY KEY,
  average_lock_time   FLOAT NOT NULL,
  w_native_rewarded  FLOAT NOT NULL
);