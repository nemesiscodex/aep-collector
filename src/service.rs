use sqlx::{PgPool, Result};
use crate::models::{Activation, SensorData, Frame};

pub struct DbService<'a>(pub &'a PgPool);

impl <'b> DbService<'b>  {
    pub async fn save_activation(&self, activation: &'b Activation) -> Result<()> {
        sqlx::query("insert into collector_activation (frame) values ($1)")
            .bind(&activation.frame)
            .execute(self.0).await
            .map(|_| ())
    }

    pub async fn save_frame(&self, frame: &'b Frame) -> Result<()> {
        sqlx::query("insert into collector_frames (satellite) values ($1)")
            .bind(&frame.satellite)
            .execute(self.0).await
            .map(|_| ())
    }

    pub async fn save_sensor(&self, sensor: &'b SensorData) -> Result<()> {
        sqlx::query("insert into collector_sensors (precipitation, wind_velocity, wind_direction, humidity, exterior_temperature, \
            interior_temperature, pressure) values ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&sensor.precipitation)
            .bind(&sensor.wind_velocity)
            .bind(&sensor.wind_direction)
            .bind(&sensor.humidity)
            .bind(&sensor.exterior_temperature)
            .bind(&sensor.interior_temperature)
            .bind(&sensor.pressure)
            .execute(self.0).await
            .map(|_| ())
    }
}