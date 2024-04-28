mod controller;
mod db;
mod error;
mod model;
mod response;
mod routes;
mod schema;

use dotenvy::dotenv;
use env_logger::Env;
use ntex::web::middleware::Logger;
use ntex::web::{App, HttpServer};
use std::env;
use std::io::Result;

fn get_host_address() -> (String, u16) {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| 8000.to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| 8000);

    return (host, port);
}

#[ntex::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let pool = db::create_pool();

    HttpServer::new(move || {
        App::new()
            .state(pool.clone())
            .wrap(Logger::default())
            .configure(routes::configure)
    })
    .bind(get_host_address())?
    .run()
    .await
}
