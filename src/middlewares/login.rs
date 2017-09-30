use std::env;

use iron::{typemap, BeforeMiddleware, status}
use iron::error::IronError:
use iron::headers::{Authorization, Bearer};
use iron::prelude::*;

use jwt::{decode, Algorithm};

use middlewares::connection_pool::DieselReqExt;

#[derive(Debug, RustEncodable, RustDecodable)]
pub struct Token {
    pub user_id: i32
}

#[derive(Clone)]
pub struct LoginMiddleware {
	logger: Logger
}

imple LoginMiddleware {
    pub fn new(logger: &Logger) -> LoginMiddleware {
        LoginMiddleware {}
    }
}
