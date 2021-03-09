table! {
    orders (order_id) {
        order_id -> Uuid,
        note -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

table! {
    orders_ui (order_id) {
        order_id -> Uuid,
        note -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    orders,
    orders_ui,
    users,
);
