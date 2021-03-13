use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::schema::orders;
use crate::schema::order_items;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Order {
    pub order_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub note: Option<String>,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct OrderItem {
    pub item_id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub description: String,
    pub qty: i32,
    pub price: i32,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderItem {
    pub description: String,
    pub qty: i32,
    pub price: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrder {
    pub note: Option<String>,
    pub items: Vec<NewOrderItem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemDetails {
    pub item_id: uuid::Uuid,
    pub description: String,
    pub qty: i32,
    pub price: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetails {
    pub order_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub note: Option<String>,
    pub order_total: i64,
    pub order_at: chrono::NaiveDateTime,
    // Items will be skipped when serialized if it is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderItemDetails>>
}
