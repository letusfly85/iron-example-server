extern crate iron;
extern crate router;
extern crate bodyparser;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use iron::status;
use iron::prelude::*;
use iron::middleware::*;
use router::Router;

extern crate iron_example_server;
extern crate diesel;
use iron_example_server::*;
use iron_example_server::models::employee::*;
use diesel::prelude::*;


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

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions
        .get::<Router>()
        .unwrap()
        .find("query")
        .unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

struct AppBeforeMiddleware;

impl BeforeMiddleware for AppBeforeMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("before handline request");
        Ok(())
    }
}

//TODO implement
fn get_employee(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

fn main() {
    use self::schema::employee::dsl::*;

    let connection = establish_connection();
    let results = employee
        .filter(department_id.eq(0))
        .limit(5)
        .load::<Employee>(&connection)
        .expect("Error loading employee");

    println!("Displaying {:?} employees", results.len());
    for emp in results {
        println!("{}", emp.id);
    }

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/log", log_body, "log");
    router.get("/employee", get_employee, "employee");

    let mut chain = Chain::new(router);
    chain.link_before(AppBeforeMiddleware);

    Iron::new(chain).http("localhost:3000").unwrap();
}
