//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{get,  post, web,  Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{ Serialize};
use uuid::Uuid;

use crate::token_utils;
use crate::models;
use crate::actions;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &conn))
        .await
        .map_err(|e| ErrorInternalServerError("Something unexpected happened. Please retry"))?;

    let user = user.ok_or(ErrorNotFound(format!(
        "No user found with uid: {}",
        user_uid
    )))?;

    Ok(HttpResponse::Ok().json(user))
}

/// Inserts new user with name defined in form.
#[post("/api/v1/auth/register")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        actions::insert_new_user(
            &form.first_name,
            &form.last_name,
            &form.email,
            &form.password,
            &conn,
        )
    })
    .await
    .map_err(|e| { ErrorInternalServerError("Something unexpected happened. Please retry")})?;

    let token_str = token_utils::generate_jwt(user.user_id);
    #[derive(Debug, Clone, Serialize)]
    struct JWTResponse {
        token: String,
    }

    Ok(HttpResponse::Ok().json(JWTResponse { token: token_str }))
}