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
        .mount("/", routes![index])
        .mount("/define", routes![define])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<term>")]
fn define(term: String) -> Option<Json<Definition>> {
    None // TODO
}

#[derive(Serialize, Deserialize)]
struct Definition {
    term: String,
    definition: String,
    example_sentence: String,
}

#[post("/", format = "json", data = "<definition>")]
fn add(definition: Json<Definition>) -> JsonValue {
    json!({ "status": "ok" }) // TODO
}
