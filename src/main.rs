#[macro_use]
extern crate validator_derive;
mod config;
mod handlers;
mod models;

use crate::config::Config;
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use handlers::app_config;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server config");

    let pool = Result = config.db_pool().await.expect("Database config");

    info!("Starting server at: http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
