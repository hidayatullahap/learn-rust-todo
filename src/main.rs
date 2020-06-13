mod models;

use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}
