DROP TABLE trader_price_data_;

CREATE TABLE trader_price_data_ (
  item_id         TEXT NOT NULL,
  trader_id       TEXT NOT NULL,

  price           INTEGER NOT NULL,

  PRIMARY KEY (item_id, trader_id)
);
