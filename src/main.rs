#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

fn main() {
    rocket().launch();
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/define", routes![define])
}

#[derive(Serialize, Deserialize)]
struct Definition {
    term: String,
    definition: String,
    example_sentence: String,
    tags: Vec<String>,
}

#[get("/<term>")]
fn define(term: String) -> Option<Json<Definition>> {
    Some(Json(Definition {
        term: String::from("Drift"),
        definition: String::from("A wonderful place to work"),
        example_sentence: String::from("I sure do enjoy working at Drift"),
        tags: vec![
            String::from("company"),
            String::from("marketing"),
            String::from("sales"),
        ],
    }))
}

#[post("/", format = "json", data = "<definition>")]
fn add(definition: Json<Definition>) -> JsonValue {
    json!({ "status": "ok" }) // TODO
}
