mod controller;
mod db;
mod error;
mod model;
mod routes;
mod schema;

use dotenvy::dotenv;
use ntex::web::{App, HttpServer};
use std::io::Result;

#[ntex::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = db::create_pool();

    HttpServer::new(move || App::new().state(pool.clone()).configure(routes::configure))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
