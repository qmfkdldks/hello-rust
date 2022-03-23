#[macro_use]
extern crate diesel;

// src/main.rs
use actix_web::{web, App, HttpServer};
// dependencies here

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// module declaration here
mod errors;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/posts", web::get().to(handlers::get_posts))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}