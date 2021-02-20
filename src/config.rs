use serde::Deserialize;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_key: String,
    pub database_url: String
}

impl Config {
    
    pub fn from_env() -> Config {
        dotenv().ok();

        let mut c = config::Config::new();
        
        c.merge(config::Environment::default())
            .expect("Error loading configurations from environment");

        c.try_into()
            .expect("Error parsing configurations from environment")
    }

    pub async fn db_pool(&self) -> PgPool {
        PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(30))
            .connect(&self.database_url)
            .await
            .expect("Error creating database connection pool")
    }
}
