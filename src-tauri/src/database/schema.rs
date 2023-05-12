// @generated automatically by Diesel CLI.

diesel::table! {
    model (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    payment (id) {
        id -> Nullable<Integer>,
        date -> Integer,
        value -> Integer,
        net_received -> Integer,
        promoter_id -> Integer,
    }
}

diesel::table! {
    promoter (id) {
        id -> Nullable<Integer>,
        name -> Text,
        bank_id -> Nullable<Text>,
        store_id -> Integer,
        active -> Integer,
    }
}

diesel::table! {
    promotion (id) {
        id -> Nullable<Integer>,
        date -> Integer,
        paid -> Integer,
        model_id -> Integer,
        promoter_id -> Integer,
    }
}

diesel::table! {
    purchase (id) {
        id -> Nullable<Integer>,
        date -> Integer,
        description -> Text,
        value -> Integer,
        debt -> Integer,
        promoter_id -> Integer,
    }
}

diesel::table! {
    store (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::joinable!(payment -> promoter (promoter_id));
diesel::joinable!(promoter -> store (store_id));
diesel::joinable!(promotion -> model (model_id));
diesel::joinable!(promotion -> promoter (promoter_id));
diesel::joinable!(purchase -> promoter (promoter_id));

diesel::allow_tables_to_appear_in_same_query!(
    model,
    payment,
    promoter,
    promotion,
    purchase,
    store,
);
