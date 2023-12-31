CREATE TABLE resource_ (
  key   TEXT PRIMARY KEY NOT NULL,
  value TEXT NOT NULL
);

CREATE TABLE item_ (
  id         TEXT PRIMARY KEY NOT NULL,
  icon_link  TEXT,
  wiki_link  TEXT,
  image_link TEXT
);

CREATE TABLE trader_ (
  id   TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE price_data_ (
  item_id       TEXT NOT NULL,
  timestamp     BIGINT NOT NULL,

  base_price    BIGINT NOT NULL,
  last_low_price BIGINT NOT NULL,
  low_24h_price BIGINT NOT NULL,
  avg_24h_price BIGINT NOT NULL,

  PRIMARY KEY (item_id, timestamp)
);

CREATE TABLE trader_price_data_ (
  item_id   TEXT NOT NULL,
  trader_id TEXT NOT NULL,

  price     BIGINT NOT NULL,

  PRIMARY KEY (item_id, trader_id)
);

CREATE TABLE file_ (
  name       TEXT PRIMARY KEY NOT NULL,
  data       BYTEA NOT NULL
);
