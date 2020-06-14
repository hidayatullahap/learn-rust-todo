use crate::models::Status;
use crate::repo;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("error conect db");

    let res = repo::get_todos(&client).await;
    match res {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
