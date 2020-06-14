mod config;
mod handlers;
mod models;
mod repo;

use crate::config::Config;
use crate::handlers::*;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use slog::info;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let log = Config::configure_log();
    let pool = config.db.create_pool(NoTls).unwrap();

    info!(
        log,
        "Starting server at http://{}:{}", config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
