table! {
    order_items (item_id) {
        item_id -> Uuid,
        order_id -> Uuid,
        description -> Varchar,
        qty -> Int4,
        price -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    orders (order_id) {
        order_id -> Uuid,
        user_id -> Uuid,
        note -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

table! {
    users (user_id) {
        user_id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
    }
}

joinable!(order_items -> orders (order_id));
joinable!(orders -> users (user_id));

allow_tables_to_appear_in_same_query!(
    order_items,
    orders,
    users,
);
