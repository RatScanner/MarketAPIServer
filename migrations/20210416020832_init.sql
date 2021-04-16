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
  id         TEXT PRIMARY KEY NOT NULL,
  name       TEXT NOT NULL
);

CREATE TABLE price_data_ (
  item_id         TEXT NOT NULL,
  timestamp       INTEGER NOT NULL,

  base_price      INTEGER NOT NULL,
  avg_24h_price   INTEGER,

  PRIMARY KEY (item_id, timestamp)
);

CREATE TABLE trader_price_data_ (
  item_id         TEXT NOT NULL,
  trader_id       TEXT NOT NULL,
  timestamp       INTEGER NOT NULL,

  price           INTEGER NOT NULL,

  PRIMARY KEY (item_id, trader_id, timestamp)
);

CREATE TABLE file_ (
  name       TEXT PRIMARY KEY NOT NULL,
  data       BLOB NOT NULL
);
