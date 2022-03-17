use anyhow::{
    Context,
    Result,
};
use diesel::{
    PgConnection,
    r2d2::ConnectionManager,
};
use r2d2::Pool;
use std::env;

mod schema;
pub mod users;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn new_pool() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL")?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(15)
        .build(manager)
        .context("failed to build database pool.")
}
