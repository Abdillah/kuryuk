table! {
    categories (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        icon -> Text,
        title -> Text,
        description -> Text,
    }
}

table! {
    transaction_category (id) {
        id -> Integer,
        transaction_id -> Integer,
        category_id -> Integer,
    }
}

table! {
    transactions (id) {
        id -> Integer,
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
