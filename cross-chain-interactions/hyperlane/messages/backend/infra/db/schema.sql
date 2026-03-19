
\connect crosschain;

CREATE TABLE IF NOT EXISTS chains (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  chain_id BIGINT NOT NULL,
  rpc_url TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS messages (
  id SERIAL PRIMARY KEY,
  from_chain TEXT NOT NULL,
  to_chain TEXT NOT NULL,
  payload TEXT NOT NULL,
  tx_hash TEXT,
  timestamp TIMESTAMPTZ DEFAULT NOW()
);
