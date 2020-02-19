#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::config::{Config, Environment};
use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use std::env;

mod elastic;
use elastic::*;

fn main() {
    rocket().launch();
}

fn rocket() -> Rocket {
    // TODO: add environment configs

    let port = get_port_from_env_or_default(8000);
    let config = Config::build(Environment::Staging).port(port).unwrap();

    rocket::custom(config)
        .mount("/define", routes![define])
        .mount("/add", routes![add])
        .register(catchers![not_found])
}

fn get_port_from_env_or_default(default: u16) -> u16 {
    let port_string = match env::var("PORT") {
        Ok(n) => n,
        _ => return default,
    };
    match port_string.parse() {
        Ok(n) => n,
        _ => default,
    }
}

#[get("/<term>")]
fn define(term: String) -> Option<Json<DefinitionsResult>> {
    Some(Json(DefinitionsResult {
        term: String::from("Drift"),
        definitions: vec![DefinitionDetail {
            definition: String::from("A wonderful place to work"),
            example_sentence: String::from("I sure do enjoy working at Drift"),
            tags: vec![
                String::from("company"),
                String::from("marketing"),
                String::from("sales"),
            ],
        }],
    }))
}

#[post("/", format = "json", data = "<definition>")]
fn add(definition: Json<AddDefinition>) -> JsonValue {
    let definition = definition.into_inner();
    match add_definition(definition) {
        Ok(term) => json!({ "status": "ok", "term_defined": term }),
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
