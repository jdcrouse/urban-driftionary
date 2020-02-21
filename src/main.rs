#![feature(proc_macro_hygiene, decl_macro)]
#![warn(unused_extern_crates)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use dotenv::dotenv;
use rocket::{Rocket, config::{Config, Environment}};
use rocket_contrib::json::{Json, JsonValue};
use std::env;

mod db;
use db::*;

mod cors;
use cors::CORS;

fn main() {
    dotenv().ok();
    let _res = rocket()
        .mount("/", routes![define, add, request, options])
        .attach(CORS())
        .launch();
}

fn rocket() -> Rocket {
    let is_in_production = env::var("ROCKET_ENV").unwrap() == "production";
    if is_in_production {
        let port = get_port_from_env_or_default(8000);
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

#[options("/define/<_term>")]
async fn options(_term: String) -> Result<(), ()> {
    Ok(())
}

#[get("/define/<term>")]
async fn define(term: String) -> Result<Json<DefinitionsResult>, JsonValue> {
    match get_definition(term).await {
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
