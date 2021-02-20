mod config;
mod handler;
mod middleware;
mod service;

use actix_web::{App, HttpServer};
use crate::middleware::ApiKey;
use crate::config::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let config = Config::from_env();
    let db_pool = config.db_pool().await;
    let key: ApiKey = ApiKey::new(config.api_key);

    let address = format!("{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(key.clone())
            .service(handler::update)
    })
    .bind(address)?
    .run()
    .await

}
