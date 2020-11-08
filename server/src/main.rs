#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::http::{Method, Status};
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

fn make_cors() -> Result<rocket_cors::Cors, rocket_cors::Error> {
    let allowed_origins = AllowedOrigins::all();

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Delete, Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
}

#[derive(Serialize, Deserialize)]
struct NewForm {
    schema: HashMap<String, SchemaEntry>,
}

#[derive(Serialize, Deserialize)]
struct SchemaEntry {
    label: String,
    input_type: String,
}

#[derive(Serialize, Deserialize)]
struct NewFormRes {
    form_id: String
}


#[post("/form", format = "application/json", data = "<next>")]
fn create_form(next: Json<NewForm>) -> Result<JsonValue, std::io::Error> {
    let next_id = Uuid::new_v4();
    let filename = format!("./schema/{}.json", next_id);
    let n = NewFormRes {
        form_id: format!("{}", next_id)
    };
    serde_json::to_writer(&File::create(filename)?, &next.0.schema)?;
    Ok(json!(n))
}

fn main() {
    match make_cors() {
        Ok(cors) => {
            rocket::ignite()
                .mount("/", routes![create_form])
                .attach(cors)
                .launch();
        }
        Err(err) => {
            panic!("Failed in CORS creation, err: {}", err);
        }
    }
}
