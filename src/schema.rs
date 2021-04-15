table! {
    item_ (id) {
        id -> Text,
        icon_link -> Nullable<Text>,
        wiki_link -> Nullable<Text>,
        image_link -> Nullable<Text>,
    }
}

table! {
    price_data_ (item_id, timestamp) {
        item_id -> Text,
        timestamp -> Integer,
        base_price -> Integer,
        avg_24h_price -> Nullable<Integer>,
    }
}

table! {
    resource_ (key) {
        key -> Text,
        value -> Text,
    }
}

table! {
    trader_ (id) {
        id -> Text,
        name -> Text,
    }
}

table! {
    trader_price_data_ (item_id, trader_id, timestamp) {
        item_id -> Text,
        trader_id -> Text,
        timestamp -> Integer,
        price -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    item_,
    price_data_,
    resource_,
    trader_,
    trader_price_data_,
);
