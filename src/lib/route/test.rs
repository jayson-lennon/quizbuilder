use actix_web::{web, HttpResponse, Responder};
use sqlx::postgres::PgPool;

pub async fn test(db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().body("testing")
}
