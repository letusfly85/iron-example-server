extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate persistent;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use persistent::Read;
use iron::status;
use iron::prelude::*;
use router::Router;


#[derive(Debug, Clone, Deserialize)]
pub struct MyStructure {
    name: String,
    message: Option<String>,
}

fn log_body(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => println!("Read body:\n{}", body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err),
    }

    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => println!("Parsed body:\n{:?}", json_body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err),
    }

    let struct_body = req.get::<bodyparser::Struct<MyStructure>>();
    match struct_body {
        Ok(Some(struct_body)) => println!("Parsed body:\n{:?}", struct_body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err),
    }

    Ok(Response::with(status::Ok))
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");

    let mut chain = Chain::new(log_body);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    //Iron::new(chain).http("localhost:3000").unwrap();
    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        println!("return ok");
        Ok(Response::with((status::Ok, *query)))
    }
}