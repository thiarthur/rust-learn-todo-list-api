use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use r2d2::Pool;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    return pool;
}
