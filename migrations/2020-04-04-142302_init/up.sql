CREATE TABLE resource (
  key   TEXT PRIMARY KEY NOT NULL,
  value TEXT NOT NULL
);

CREATE TABLE market_item (
  uid        TEXT PRIMARY KEY NOT NULL,
  slots      INTEGER NOT NULL,
  wiki_link  TEXT NOT NULL,
  img_link   TEXT NOT NULL
);

CREATE TABLE market_item_name (
  uid        TEXT NOT NULL,
  lang       TEXT NOT NULL,
  name       TEXT NOT NULL,
  short_name TEXT NOT NULL,
  
  PRIMARY KEY (uid, lang)
);

CREATE TABLE price_data (
  uid             TEXT NOT NULl,
  timestamp       INTEGER NOT NULL,
  price           INTEGER NOT NULL,
  avg_24h_price   INTEGER NOT NULL,
  avg_7d_price    INTEGER NOT NULL,
  trader_name     TEXT NOT NULL,
  trader_price    INTEGER NOT NULL,
  trader_currency TEXT NOT NULL,

  PRIMARY KEY (uid, timestamp)
);