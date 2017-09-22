use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;

use iron::{typemap, BeforeMiddleware, Request, IronResult};

pub type DieselConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
pub type DieselPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct DieselMiddleware {
    pool: DieselPool,
}

impl DieselMiddleware {
    pub fn new() -> DieselMiddleware {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let config = r2d2::Config::default();
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool");

        DieselMiddleware { pool: pool }
    }
}

pub struct Value(DieselPool);

impl typemap::Key for DieselMiddleware {
    type Value = Value;
}

impl BeforeMiddleware for DieselMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DieselMiddleware>(
            Value(self.pool.clone()),
        );
        Ok(())
    }
}

pub trait DieselReqExt {
    fn get_db_conn(&self) -> DieselConnection;
}

impl<'a, 'b> DieselReqExt for Request<'a, 'b> {
    fn get_db_conn(&self) -> DieselConnection {
        let &Value(ref pool) = self.extensions.get::<DieselMiddleware>().unwrap();

        return pool.get().expect("Failed to get a db connection");
    }
}
