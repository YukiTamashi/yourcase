// @generated automatically by Diesel CLI.

diesel::table! {
    models (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    payments (id) {
        id -> Integer,
        date -> Integer,
        value -> Integer,
        net_received -> Integer,
        promoter_id -> Integer,
    }
}

diesel::table! {
    promoters (id) {
        id -> Integer,
        name -> Text,
        bank_id -> Nullable<Text>,
        store_id -> Integer,
        active -> Integer,
        debt -> Integer,
    }
}

diesel::table! {
    promotions (id) {
        id -> Integer,
        date -> Integer,
        value -> Integer,
        model_id -> Integer,
        promoter_id -> Integer,
    }
}

diesel::table! {
    purchases (id) {
        id -> Integer,
        date -> Integer,
        description -> Text,
        value -> Integer,
        promoter_id -> Integer,
    }
}

diesel::table! {
    stores (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    models,
    payments,
    promoters,
    promotions,
    purchases,
    stores,
);
