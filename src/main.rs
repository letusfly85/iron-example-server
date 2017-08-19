extern crate iron;
extern crate bodyparser;
extern crate persistent;
#[macro_use]
extern crate serde_derive;

use persistent::Read;
use iron::status;
use iron::prelude::*;

fn log_body(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>();

    Ok(Response::with(status::Ok))
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    let mut chain = Chain::new(log_body);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();
}
