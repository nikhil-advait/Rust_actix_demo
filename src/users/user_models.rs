use serde::{Deserialize, Serialize};

use crate::schema::users;


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