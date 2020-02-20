#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate dotenv;

use dotenv::dotenv;
use rocket::config::{Config, Environment};
use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use std::env;

mod db;
use db::*;

fn main() {
    dotenv().ok();
    rocket()
        .mount("/", routes![define, add, request])
        .register(catchers![not_found])
        .launch();
}

fn rocket() -> Rocket {
    let is_in_production = env::var("ROCKET_ENV").unwrap() == "production";
    if is_in_production {
        let port = get_port_from_env_or_default(8001);
        let config = Config::build(Environment::Production).port(port).unwrap();
        rocket::custom(config)
    } else {
        rocket::ignite()
    }
}

fn get_port_from_env_or_default(default: u16) -> u16 {
    match env::var("PORT") {
        Ok(port_string) => match port_string.parse() {
            Ok(num) => num,
            _ => default,
        },
        _ => default,
    }
}

#[get("/define/<term>")]
fn define(term: String) -> Result<Json<DefinitionsResult>, JsonValue> {
    match get_definition(term) {
        Some(defn) => Ok(Json(defn)),
        _ => Err(json!({"status": "error", "reason": "Term is not defined."})),
    }
}

#[post("/add", format = "json", data = "<definition>")]
fn add(definition: Json<AddDefinition>) -> JsonValue {
    let definition = definition.into_inner();
    match add_definition(definition) {
        Ok(term) => json!({ "status": "ok", "term_defined": term }),
        Err(e) => json!({"status": "error", "reason": e}),
    }
}

#[post("/request", format = "json", data = "<tbd>")]
fn request(tbd: Json<ToBeDefined>) -> JsonValue {
    let tbd = tbd.into_inner();
    match request_to_be_defined(tbd) {
        Ok(term) => json!({ "status": "ok", "term_to_be_defined": term }),
        Err(e) => json!({"status": "error", "reason": e}),
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
