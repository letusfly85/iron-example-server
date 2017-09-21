extern crate iron;
extern crate router;
extern crate bodyparser;
#[macro_use]
extern crate serde_derive;

extern crate serde;

#[macro_use]
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

fn find_employee(req: &mut Request) -> IronResult<Response> {
    //TODO validate query parameter
    let employee_id: i32 = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .unwrap()
        .parse()
        .unwrap();

    use self::schema::employee::dsl::*;

    //TODO remove connection from method, use something like connection pool
    let connection = establish_connection();
    let results = employee
        .filter(id.eq(employee_id))
        .limit(5)
        .load::<Employee>(&connection)
        .expect("Error loading employee");

    let mut return_emp = json!({});
    for emp in results {
        println!("{}", emp.id);
        println!("{}", emp.name);
        println!("{}", emp.department_id);

        return_emp = json!({
            "id": emp.id,
            "name": emp.id,
            "department_id": emp.department_id
        });
        println!("{}", return_emp);
    }

    //TODO contruct structure of json response
    Ok(Response::with((status::Ok, return_emp.to_string())))
}

fn main() {


    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/log", log_body, "log");
    router.get("/employees/:id", find_employee, "id");

    let mut chain = Chain::new(router);
    chain.link_before(AppBeforeMiddleware);

    Iron::new(chain).http("localhost:3001").unwrap();
}
