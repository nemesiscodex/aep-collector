use actix_web::{HttpResponse, get, web::{Data, Query}};
use sqlx::PgPool;
use log::{info, error};
use crate::models::{Activation, SensorData, Frame};
use crate::service::DbService;

// GET /update will receive raw data and store it in the database
// using GET for compatibility
#[get("/update")]
pub async fn activations(Query(activation): Query<Activation>, pool: Data::<PgPool>) -> HttpResponse {

    let result = DbService(&pool).save_activation(&activation).await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .body("ACK\n"),
        Err(e) => {
            error!("Error saving activation data: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
          
    }
        
}

// GET /sensors will receive sensor data
// using GET for compatibility
#[get("/sensors")]
pub async fn sensors(Query(sensor_data): Query<SensorData>, pool: Data::<PgPool>) -> HttpResponse {
    info!("Storing sensor data {:?}", sensor_data);
    let result = DbService(&pool).save_sensor(&sensor_data).await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .body("ACK\n"),
        Err(e) => {
            error!("Error saving sensor data: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
        
}

// GET /frames will receive raw frames
// using GET for compatibility
#[get("/frames")]
pub async fn frames(Query(frame): Query<Frame>, pool: Data::<PgPool>) -> HttpResponse {
    let result = DbService(&pool).save_frame(&frame).await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .body("ACK\n"),
        Err(e) => {
            error!("Error saving frame data: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
        
}