use actix_web::{HttpResponse, get, web::{Data, Query}};
use sqlx::PgPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Frame {
    frame: String
}

// GET /update will receive raw data and store it in the database
// using GET for compatibility
#[get("/update")]
pub async fn update(Query(frame): Query<Frame>, pool: Data::<PgPool>) -> HttpResponse {
    let result = 
        sqlx::query("insert into collector (frame) values ($1)")
            .bind(frame.frame)
            .execute(pool.as_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
        
}