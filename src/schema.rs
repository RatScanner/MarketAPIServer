table! {
    market_item (uid) {
        uid -> Text,
        slots -> Integer,
        wiki_link -> Text,
        img_link -> Text,
    }
}

table! {
    market_item_name (uid, lang) {
        uid -> Text,
        lang -> Text,
        name -> Text,
        short_name -> Text,
    }
}

table! {
    price_data (uid, timestamp) {
        uid -> Text,
        timestamp -> Integer,
        price -> Integer,
        avg_24h_price -> Integer,
        avg_7d_price -> Integer,
        trader_name -> Text,
        trader_price -> Integer,
        trader_currency -> Text,
    }
}

table! {
    resource (key) {
        key -> Text,
        value -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    market_item,
    market_item_name,
    price_data,
    resource,
);
