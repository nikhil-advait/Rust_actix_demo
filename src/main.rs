//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

#[macro_use]
extern crate diesel;

use actix_web::error::{BlockingError, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod actions;
mod models;
mod order_handlers;
mod schema;
mod token_utils;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(add_user)
            .service(get_user)
            .service(order_handlers::get_order)
            .service(order_handlers::create_order)
            .service(order_handlers::get_order_details_for_user)
    })
    .bind(&bind)?
    .run()
    .await
}
