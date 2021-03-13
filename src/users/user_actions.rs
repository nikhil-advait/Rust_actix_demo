#[path = "./user_models.rs"] pub mod models;

use actix_web::http::{HeaderValue, StatusCode};
use diesel::prelude::*;

use uuid::Uuid;

#[path = "./token_utils.rs"] mod token_utils;

/// Find user by user_id. If not found then return None.
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::User>, StatusCode> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(user_id.eq(uid))
        .first::<models::User>(conn)
        .optional()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(user)
}

/// Find user by email. If not found then return None.
pub fn find_user_by_email(
    email_str: &str,
    conn: &PgConnection,
) -> Result<Option<models::User>, StatusCode> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let user = users
        .filter(email.eq(email_str))
        .first::<models::User>(conn)
        .optional()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(user)
}

/// Extract user_id from jwt token and verify in db that user by that user_id exists.
/// This should ideally have been authentication middleware of actix. Todo.
pub fn authenticate_request(
    header: Option<HeaderValue>,
    conn: &PgConnection,
) -> Result<uuid::Uuid, StatusCode> {

    let v = header.ok_or(StatusCode::UNAUTHORIZED)?;
    let jwt_str = v.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id =
        token_utils::decode_jwt_and_get_user_id(jwt_str).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_option =
        find_user_by_uid(user_id, conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    user_option.map(|u| u.user_id).ok_or(StatusCode::NOT_FOUND)
}


/// Insert new user in db as part of new user registration.
pub fn insert_new_user(
    first_n: &str,
    last_n: &str,
    email_str: &str,
    passwd: &str,
    conn: &PgConnection,
) -> Result<models::User, StatusCode> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        user_id: Uuid::new_v4(),
        first_name: first_n.to_owned(),
        last_name: last_n.to_owned(),
        email: email_str.to_owned(),
        password: passwd.to_owned(),
        created_at: chrono::offset::Utc::now().naive_utc(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(new_user)
}