use actix_web::{HttpResponse, get, web::{Data, Query}};
use sqlx::PgPool;
use serde::Deserialize;
use log::{info, error};

#[derive(Deserialize, Debug)]
pub struct Activation {
    frame: String
}

#[derive(Deserialize, Debug)]
pub struct Frame {
    satellite: String
}

#[derive(Deserialize, Debug)]
pub struct SensorData {
    precipitation: f32,
    wind_velocity: f32,
    wind_direction: f32,
    humidity: f32,
    exterior_temperature: f32,
    interior_temperature: f32,
    pressure: f32,
}

// GET /update will receive raw data and store it in the database
// using GET for compatibility
#[get("/update")]
pub async fn activations(Query(activation): Query<Activation>, pool: Data::<PgPool>) -> HttpResponse {

    let result = 
        sqlx::query("insert into collector_activation (frame) values ($1)")
            .bind(activation.frame)
            .execute(pool.as_ref()).await;

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
    let result = 
        sqlx::query("insert into collector_sensors (precipitation, wind_velocity, wind_direction, humidity, exterior_temperature, \
            interior_temperature, pressure) values ($1, $2, $3, $4, $5, $6, $7)")
            .bind(sensor_data.precipitation)
            .bind(sensor_data.wind_velocity)
            .bind(sensor_data.wind_direction)
            .bind(sensor_data.humidity)
            .bind(sensor_data.exterior_temperature)
            .bind(sensor_data.interior_temperature)
            .bind(sensor_data.pressure)
            .execute(pool.as_ref()).await;

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
    let result = 
        sqlx::query("insert into collector_frames (satellite) values ($1)")
            .bind(frame.satellite)
            .execute(pool.as_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .body("ACK\n"),
        Err(e) => {
            error!("Error saving frame data: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
        
}