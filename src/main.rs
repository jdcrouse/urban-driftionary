#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::config::{Config, Environment};
use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::env;

fn main() {
    rocket().launch();
}

fn rocket() -> Rocket {
    // TODO: only use env var if it exists, otherwise default to config
    // TODO: add environment configs
    let port: u16 = match env::var("PORT").unwrap().parse() {
        Ok(n) => n,
        _ => panic!("I need a number"),
    };

    let config = Config::build(Environment::Staging).port(port).unwrap();

    rocket::custom(config)
        .mount("/define", routes![define])
        .mount("/add", routes![add])
        .register(catchers![not_found])
}

#[derive(Serialize, Deserialize)]
struct DefinitionResponse {
    term: String,
    definitions: Vec<Definition>,
}

#[derive(Serialize, Deserialize)]
struct Definition {
    definition: String,
    example_sentence: String,
    tags: Vec<String>,
}

#[get("/<term>")]
fn define(term: String) -> Option<Json<DefinitionResponse>> {
    Some(Json(DefinitionResponse {
        term: String::from("Drift"),
        definitions: vec![Definition {
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

#[derive(Serialize, Deserialize)]
struct NewDefinition {
    term: String,
    definition: String,
    example_sentence: String,
    tags: Vec<String>,
}

#[post("/", format = "json", data = "<definition>")]
fn add(definition: Json<NewDefinition>) -> JsonValue {
    json!({ "status": "ok", "term_defined": definition.term }) // TODO
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
