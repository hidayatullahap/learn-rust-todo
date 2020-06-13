mod config;
mod models;

use crate::config::Config;
use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use slog::info;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let log = Config::configure_log();

    info!(
        log,
        "Starting server at http://{}:{}", config.server.host, config.server.port
    );

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}
