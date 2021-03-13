//! Diesel does not support async operations, i.e. diesel operations are blocking, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

use actix_web::error::{BlockingError, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

use crate::actions;
use crate::models;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Inserts new user with name defined in body.
#[post("/api/v1/orders")]
pub async fn create_order(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    body: web::Json<models::NewOrder>,
) -> Result<HttpResponse, Error> {
    let conn = pool
        .get()
        .map_err(|_| ErrorInternalServerError("couldn't get db connection from pool. Please retry."))?;

    let order_id = Uuid::new_v4();
    let note_option = body.note.clone();

    let jwt_header = req.headers().get("access_token").cloned();

    // use web::block to offload blocking Diesel code without blocking server thread
    let order = web::block(move || {
        // Todo: Convert authenticate_request function to actix middleware.
        let user_id = actions::authenticate_request(jwt_header, &conn)?;
        let order = actions::insert_new_order(order_id.clone(), user_id, note_option, &conn);
        actions::insert_new_order_items(order_id, &body.items, &conn)?;
        order
    })
    .await
    .map_err(|e| {
        match e {
            BlockingError::Error(StatusCode::UNAUTHORIZED) => {
                ErrorUnauthorized("Provide proper access token")
            }
            BlockingError::Error(StatusCode::NOT_FOUND) => {
                ErrorNotFound("User in access_token is not found in db to create new order.")
            }
            _ => ErrorInternalServerError("Something unexpected happened. Please retry"),
        }
    })?;

    Ok(HttpResponse::Ok().json(order))
}

#[get("/api/v1/orders")]
pub async fn get_order_details_for_user(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().map_err(|_| ErrorInternalServerError("couldn't get db connection from pool. Please retry."))?;

    let jwt_header = req.headers().get("access_token").cloned();

    // use web::block to offload blocking Diesel code without blocking server thread
    let order_details = web::block(move || {
        let user_id = actions::authenticate_request(jwt_header, &conn)?;

        actions::find_all_orders_for_user(user_id, &conn)
    })
    .await
    .map_err(|e| match e {
        BlockingError::Error(StatusCode::UNAUTHORIZED) => {
            ErrorUnauthorized("Provide proper access token")
        }
        _ => ErrorInternalServerError("Something unexpected happened. Please retry"),
    })?;

    Ok(HttpResponse::Ok().json(order_details))
}

/// Finds user by UID.
#[get("/api/v1/orders/{order_id}")]
pub async fn get_order(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    order_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().map_err(|_| ErrorInternalServerError("couldn't get db connection from pool. Please retry."))?;

    let order_id = order_uid.into_inner();
    let jwt_header = req.headers().get("access_token").cloned();

    // use web::block to offload blocking Diesel code without blocking server thread
    let order = web::block(move || {
        // Todo: Convert authenticate_request function to actix middleware.
        let user_id = actions::authenticate_request(jwt_header, &conn)?;

        actions::find_order_by_id(user_id, order_id, &conn)
    })
    .await
    .map_err(|e| match e {
        BlockingError::Error(StatusCode::UNAUTHORIZED) => {
            ErrorUnauthorized("Provide proper access token")
        }
        BlockingError::Error(StatusCode::NOT_FOUND) => {
            ErrorNotFound("Order id not correct(or not present) for the user in access_token.")
        }
        _ => ErrorInternalServerError("Something unexpected happened. Please retry"),
    })?;

    Ok(HttpResponse::Ok().json(order))
}
