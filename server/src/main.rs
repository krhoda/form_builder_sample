#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::http::{Method};
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::path::{Path, PathBuf};
use uuid::Uuid;

const APP_DIR: &str = "./public";
const SCHEMA_DIR: &str = "./schema";
const SUBMISSION_DIR: &str = "./submission";

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
struct Schema {
    schema: HashMap<String, SchemaEntry>,
}

#[derive(Serialize, Deserialize)]
struct SchemaEntry {
    label: String,
    input_type: String,
}

#[derive(Serialize, Deserialize)]
struct NewFormRes {
    form_id: String,
}

#[derive(Serialize, Deserialize)]
struct FetchFormReq {
    form_id: String,
}

#[derive(Serialize, Deserialize)]
struct SubmissionReq {
    form_id: String,
    submission: HashMap<String, SubmissionEntry>,
}

#[derive(Serialize, Deserialize)]
struct SubmissionEntry {
    label: String,
    input_type: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct SubmissionRes {
    submission_id: String,
}

#[post("/create-form", format = "application/json", data = "<new_schema>")]
fn create_form(new_schema: Json<Schema>) -> Result<JsonValue, std::io::Error> {
    std::fs::create_dir_all(&SCHEMA_DIR)?;

    let form_id = Uuid::new_v4();
    let file_name = format!("{}.json", form_id);

    let full_path = Path::new(SCHEMA_DIR).join(file_name);

    serde_json::to_writer(&File::create(full_path)?, &new_schema.0.schema)?;

    let res = NewFormRes {
        form_id: format!("{}", form_id),
    };

    Ok(json!(res))
}

#[post("/fetch-form", format = "application/json", data = "<fetch_req>")]
fn fetch_form(fetch_req: Json<FetchFormReq>) -> Result<JsonValue, std::io::Error> {
    std::fs::create_dir_all(&SCHEMA_DIR)?;

    let file_name = format!("{}.json", fetch_req.0.form_id);

    let s = read_to_string(Path::new(&SCHEMA_DIR).join(file_name))?;

    let inner_schema = serde_json::from_str(&s)?;
    let res = Schema {
        schema: inner_schema,
    };

    Ok(json!(res))
}

#[post("/submit", format = "application/json", data = "<sub_req>")]
fn submit(sub_req: Json<SubmissionReq>) -> Result<JsonValue, std::io::Error> {
    std::fs::create_dir_all(&SUBMISSION_DIR)?;

    let form_id = sub_req.0.form_id;

    let dir_name = Path::new(SUBMISSION_DIR).join(form_id);
    std::fs::create_dir_all(&dir_name)?;

    let next_id = Uuid::new_v4();
    let filename = format!("{}.json", next_id);

    serde_json::to_writer(&File::create(Path::new(&dir_name).join(filename))?, &sub_req.0.submission)?;

    let res = SubmissionRes {
        submission_id: format!("{}", next_id),
    };

    Ok(json!(res))
}

#[get("/")]
fn show_form() -> Option<NamedFile> {
    NamedFile::open(Path::new(APP_DIR).join("index.html")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(APP_DIR).join(file)).ok()
}

fn main() {
    match make_cors() {
        Ok(cors) => {
            rocket::ignite()
                .mount("/", routes![create_form, fetch_form, submit, show_form, files])
                .attach(cors)
                .launch();
        }
        Err(err) => {
            panic!("Failed in CORS creation, err: {}", err);
        }
    }
}
