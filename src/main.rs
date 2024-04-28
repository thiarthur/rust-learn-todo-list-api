mod controller;
mod db;
mod error;
mod model;
mod response;
mod routes;
mod schema;

use dotenvy::dotenv;
use ntex::web::middleware::Logger;
use ntex::web::{App, HttpServer};
use std::io::Result;

use env_logger::Env;

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
