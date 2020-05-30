table! {
    categories (id) {
        id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Text,
        icon -> Text,
        title -> Text,
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
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    transaction_category,
    transactions,
);
