//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

use actix_web::{
    error::{
        BlockingError, ErrorConflict, ErrorForbidden, ErrorInternalServerError, ErrorNotFound,
    },
    http::StatusCode,
};
use actix_web::{post, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::Serialize;

use crate::actions;
use crate::models;
use crate::token_utils;
#[derive(Debug, Clone, Serialize)]
struct JWTResponse {
    token: String,
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Inserts new user with name defined in form.
#[post("/api/v1/auth/register")]
async fn register_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        // Check if user with email id is already present. If yes, then return error.
        let user_option = actions::find_user_by_email(&form.email, &conn)?;

        if user_option.is_some() {
            return Err(StatusCode::CONFLICT);
        }

        actions::insert_new_user(
            &form.first_name,
            &form.last_name,
            &form.email,
            &form.password,
            &conn,
        )
    })
    .await
    .map_err(|e| match e {
        BlockingError::Error(StatusCode::CONFLICT) => {
            ErrorConflict("User with email already present")
        }
        BlockingError::Error(StatusCode::NOT_FOUND) => {
            ErrorNotFound("User could not be found to create new order.")
        }
        _ => ErrorInternalServerError("Something unexpected happened. Please retry"),
    })?;

    let token_str = token_utils::generate_jwt(user.user_id);

    Ok(HttpResponse::Ok().json(JWTResponse { token: token_str }))
}

/// Inserts new user with name defined in form.
#[post("/api/v1/auth/login")]
async fn login_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::UserLogin>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        // Check if user with email id is already present. If yes, then return error.
        let user_option = actions::find_user_by_email(&form.email, &conn)?;

        if let Some(user) = user_option {
            if user.password != form.password {
                return Err(StatusCode::FORBIDDEN);
            } else {
                return Ok(user);
            }
        } else {
            // User not found for given email
            return Err(StatusCode::FORBIDDEN);
        }
    })
    .await
    .map_err(|e| match e {
        BlockingError::Error(StatusCode::FORBIDDEN) => {
            ErrorForbidden("email and/or password not correct.")
        }
        _ => ErrorInternalServerError("Something unexpected happened. Please retry"),
    })?;

    let token_str = token_utils::generate_jwt(user.user_id);

    Ok(HttpResponse::Ok().json(JWTResponse { token: token_str }))
}
