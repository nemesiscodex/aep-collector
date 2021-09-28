use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Activation {
    pub frame: String
}

#[derive(Deserialize, Debug)]
pub struct Frame {
    pub satellite: String
}

#[derive(Deserialize, Debug)]
pub struct SensorData {
    pub precipitation: f32,
    pub wind_velocity: f32,
    pub wind_direction: f32,
    pub humidity: f32,
    pub exterior_temperature: f32,
    pub interior_temperature: f32,
    pub pressure: f32,
}