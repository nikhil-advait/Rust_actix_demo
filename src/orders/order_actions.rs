#[path = "./order_models.rs"] pub mod models;

use actix_web::http::{ StatusCode};
use diesel::prelude::*;
use models::NewOrderItem;
use models::{Order, OrderDetails, OrderItem, OrderItemDetails};
use std::collections::HashMap;
use uuid::Uuid;




/// Find order corresponding to given user_id and order_id.
pub fn find_order_by_id(user_id_arg: Uuid, oid: Uuid, conn: &PgConnection) -> Result<OrderDetails, StatusCode> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::orders::dsl::*;
    use crate::schema::order_items::dsl::*;

    let vec: Vec<(Order, OrderItem)> = orders
        .inner_join(order_items)
        // Only allow to access user’s own order not others
        .filter(crate::schema::orders::dsl::user_id.eq(user_id_arg))
        .filter(crate::schema::orders::dsl::order_id.eq(oid))
        .into_boxed()
        .get_results(conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If vec is empty. Then return not found error.
    if vec.len() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Order is common for all tuples in vector. Hence taking first one.
    let order = vec[0].0.clone();

    let mut ret_value: OrderDetails = OrderDetails {
        order_id: order.order_id,
        user_id: order.user_id,
        note: order.note,
        order_total: 0,
        order_at: order.created_at,
        // Mark items as None initially. This will be set to below again.
        items: None, //vec![]
    };

    let mut order_total: i64 = 0;
    let mut order_item_details_vec: Vec<OrderItemDetails> = vec![];

    // Iterate over all tuples and calcuate order_total. Also collect orter_items.
    vec.iter().for_each(|tup| {
        let order_item = tup.1.clone();
        order_total = order_total + i64::from(order_item.qty * order_item.price);

        order_item_details_vec.push(OrderItemDetails {
            item_id: order_item.item_id,
            description: order_item.description,
            qty: order_item.qty,
            price: order_item.price,
        });
    });

    ret_value.order_total = order_total;
    ret_value.items = Some(order_item_details_vec);

    Ok(ret_value)
}

/// Find all orders for a user_id (from jwt).
pub fn find_all_orders_for_user(uid: Uuid, conn: &PgConnection) -> Result<Vec<OrderDetails>, StatusCode> {
    use crate::schema::order_items::dsl::*;
    use crate::schema::orders::dsl::*;

    let vec: Vec<(Order, OrderItem)> = orders
        .inner_join(order_items)
        .filter(user_id.eq(uid))
        .into_boxed()
        .get_results(conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut dictionary: HashMap<&uuid::Uuid, OrderDetails> = HashMap::new();

    vec.iter().for_each(|tup| {
        let order = &tup.0;
        let order_item = &tup.1;
        match dictionary.get_mut(&order.order_id) {
            // Insert record in hashmap for the first time.
            None => dictionary.insert(
                &order.order_id,
                OrderDetails {
                    order_id: order.order_id,
                    user_id: order.user_id,
                    note: order.note.clone(),
                    order_total: i64::from(order_item.qty * order_item.price),
                    order_at: order.created_at,
                    items: None,
                },
            ),
            // Update order_total for subsequent orders.
            Some(od) => {
                od.order_total = od.order_total + i64::from(order_item.qty + order_item.price);
                None
            }
        };
    });

    let vec_of_order_details: Vec<OrderDetails> = dictionary.values().cloned().collect();

    Ok(vec_of_order_details)
}


pub fn insert_new_order(
    order_id_arg: uuid::Uuid,
    user_id_arg: uuid::Uuid,
    note_arg: Option<String>,
    conn: &PgConnection,
) -> Result<models::Order, StatusCode> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::orders::dsl::*;

    let new_order = models::Order {
        order_id: order_id_arg,
        user_id: user_id_arg,
        note: note_arg,
        created_at: chrono::offset::Utc::now().naive_utc(),
    };

    diesel::insert_into(orders)
        .values(&new_order)
        .execute(conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(new_order)
}

pub fn insert_new_order_items(
    order_id_arg: uuid::Uuid,
    order_items_arg: &Vec<NewOrderItem>,
    conn: &PgConnection,
) -> Result<bool, StatusCode> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::order_items::dsl::*;

    let new_order_items: Vec<OrderItem> = order_items_arg
        .iter()
        .map(|oi| OrderItem {
            item_id: Uuid::new_v4(),
            order_id: order_id_arg,
            description: oi.description.clone(),
            qty: oi.qty,
            price: oi.price,
            created_at: chrono::offset::Utc::now().naive_utc(),
        })
        .collect::<Vec<_>>();

    diesel::insert_into(order_items)
        .values(&new_order_items)
        .execute(conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(true)
}
