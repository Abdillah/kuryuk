table! {
    categories (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Text,
        icon -> Text,
        title -> Text,
        description -> Text,
    }
}

table! {
    transaction_category (id) {
        id -> Nullable<Integer>,
        transaction_id -> Integer,
        category_id -> Integer,
    }
}

table! {
    transactions (id) {
        id -> Nullable<Integer>,
        trx_id -> Text,
        title -> Text,
        description -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    transaction_category,
    transactions,
);
