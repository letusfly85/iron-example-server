use diesel::mysql::MysqlConnection
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;

use iron::{typemap, BeforeMiddleware};
use iron::prelude::*;

pub type DieselConnection = r2d2::PoolConnection<ConnectionManager<MysqlConnection>>;
pub type DieselPool = r2d2<ConnectionManager<MysqlConnection>>;

pub struct DieselMiddleware {
    pool: DieselPool
}

impl DieselMiddleware {
    pub fn new() -> DieselMiddleware {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let config = r2d2::Config::default();
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::new(config, namager).expect("Failed to create pool");

        DieselMiddleware {pool: pool}
    }
}
