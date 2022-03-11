table! {
    products (id) {
        id -> Int4,
        title -> Varchar,
        stock -> Float8,
        rating -> Nullable<Float8>,
        price -> Nullable<Int4>,
        description -> Nullable<Varchar>,
        text_searchable_product_col -> Tsvector,
        product_rank -> Nullable<Numeric>,
        user_id -> Int4,
    }
}

table! {
    users (email) {
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users_table (id) {
        id -> Int4,
    }
}

joinable!(products -> users_table (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    users,
    users_table,
);
