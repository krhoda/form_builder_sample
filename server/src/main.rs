#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::http::Method;
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::path::{Path, PathBuf};
use uuid::Uuid;

// UI / Form locations:
const APP_DIR: &str = "public";
const SCHEMA_DIR: &str = "schema";
const SUBMISSION_DIR: &str = "submission";

// CORS helper for easier dev.
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

// Create form structs and route.
#[derive(Serialize, Deserialize)]
struct Schema {
    contents: SchemaContents,
}

#[derive(Serialize, Deserialize)]
struct SchemaContents {
    schema_name: String,
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

#[post("/create-form", format = "application/json", data = "<new_schema>")]
fn create_form(new_schema: Json<Schema>) -> Result<JsonValue, std::io::Error> {
    std::fs::create_dir_all(&SCHEMA_DIR)?;

    let form_id = Uuid::new_v4();
    let file_name = format!("{}.json", form_id);

    let full_path = Path::new(SCHEMA_DIR).join(file_name);

    serde_json::to_writer(&File::create(full_path)?, &new_schema.0.contents)?;

    let res = NewFormRes {
        form_id: format!("{}", form_id),
    };

    Ok(json!(res))
}

// Show form structs and route
#[derive(Serialize, Deserialize)]
struct FetchFormReq {
    form_id: String,
}

#[post("/fetch-form", format = "application/json", data = "<fetch_form_req>")]
fn fetch_form(fetch_form_req: Json<FetchFormReq>) -> Result<JsonValue, std::io::Error> {
    let file_name = format!("{}.json", fetch_form_req.0.form_id);

    let s = read_to_string(Path::new(&SCHEMA_DIR).join(file_name))?;

    let contents = serde_json::from_str(&s)?;
    let res = Schema { contents: contents };

    Ok(json!(res))
}

// Form submission structs and route
#[derive(Serialize, Deserialize)]
struct CreateSubmissionReq {
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
struct CreateSubmissionRes {
    submission_id: String,
}

#[post("/submit", format = "application/json", data = "<sub_req>")]
fn submit(sub_req: Json<CreateSubmissionReq>) -> Result<JsonValue, std::io::Error> {
    std::fs::create_dir_all(&SUBMISSION_DIR)?;

    let form_id = sub_req.0.form_id;

    let dir_name = Path::new(SUBMISSION_DIR).join(form_id);
    std::fs::create_dir_all(&dir_name)?;

    let next_id = Uuid::new_v4();
    let filename = format!("{}.json", next_id);

    serde_json::to_writer(
        &File::create(Path::new(&dir_name).join(filename))?,
        &sub_req.0.submission,
    )?;

    let res = CreateSubmissionRes {
        submission_id: format!("{}", next_id),
    };

    Ok(json!(res))
}

// Show submission structs and route
#[derive(Serialize, Deserialize)]
struct FetchSubmissionReq {
    form_id: String,
    submission_id: String,
}

#[derive(Serialize, Deserialize)]
struct FetchSubmissionRes {
    contents: SubmissionContents,
}

#[derive(Serialize, Deserialize)]
struct SubmissionContents {
    schema_name: String,
    submission: HashMap<String, SubmissionEntry>,
}

#[post(
    "/fetch-submission",
    format = "application/json",
    data = "<fetch_submission_req>"
)]
fn fetch_submission(
    fetch_submission_req: Json<FetchSubmissionReq>,
) -> Result<JsonValue, std::io::Error> {
    let schema_file_name = format!("{}.json", fetch_submission_req.0.form_id);

    let schema_contents = read_to_string(Path::new(&SCHEMA_DIR).join(schema_file_name))?;

    let target_schema: SchemaContents = serde_json::from_str(&schema_contents)?;

    let file_name = format!("{}.json", fetch_submission_req.0.submission_id);
    let submission_contents = read_to_string(
        Path::new(&SUBMISSION_DIR)
            .join(fetch_submission_req.0.form_id)
            .join(file_name),
    )?;

    let inner_submission = serde_json::from_str(&submission_contents)?;
    let res = FetchSubmissionRes {
        contents: SubmissionContents {
            schema_name: target_schema.schema_name,
            submission: inner_submission,
        },
    };

    Ok(json!(res))
}

// List Forms structs and route
#[derive(Serialize, Deserialize)]
struct ListFormsRes {
    form_list: Vec<ListFormEntry>,
}

#[derive(Serialize, Deserialize)]
struct ListFormEntry {
    form_id: String,
    form_name: String,
}

#[get("/list-forms")]
fn list_forms() -> Result<JsonValue, std::io::Error> {
    let mut res = ListFormsRes {
        form_list: Vec::new(),
    };

    for entry in std::fs::read_dir(Path::new(&SCHEMA_DIR))? {
        let entry = entry?;
        let entry_path = entry.path();
        let schema_string = read_to_string(&entry_path)?;
        let target_schema: SchemaContents = serde_json::from_str(&schema_string)?;

        let mut entry_str = entry_path.into_os_string().into_string().unwrap();
        let entry_len = entry_str.len();
        entry_str.truncate(entry_len - 5);
        let form_id = entry_str.trim_start_matches("schema/");

        let list_entry = ListFormEntry {
            form_id: form_id.to_string(),
            form_name: target_schema.schema_name,
        };

        res.form_list.push(list_entry);
    }

    Ok(json!(res))
}

// List Submissions structs and route
#[derive(Serialize, Deserialize)]
struct ListSubmisionsRes {
    submission_list: Vec<ListSubmissionsEntry>,
}

#[derive(Serialize, Deserialize)]
struct ListSubmissionsEntry {
    submission_id: String,
    form_id: String,
    form_name: String,
}

#[get("/list-submissions")]
fn list_submissions() -> Result<JsonValue, std::io::Error> {
    let mut res = ListSubmisionsRes {
        submission_list: Vec::new(),
    };

    let mut name_map: HashMap<String, String> = HashMap::new();

    for entry in std::fs::read_dir(Path::new(&SCHEMA_DIR))? {
        let entry = entry?;
        let entry_path = entry.path();
        let schema_string = read_to_string(&entry_path)?;
        let target_schema: SchemaContents = serde_json::from_str(&schema_string)?;

        let mut entry_str = entry_path.into_os_string().into_string().unwrap();
        let entry_len = entry_str.len();
        entry_str.truncate(entry_len - 5);
        let form_id = entry_str.trim_start_matches("schema/");

        name_map.insert(form_id.to_string(), target_schema.schema_name);
    }

    for entry in std::fs::read_dir(Path::new(&SUBMISSION_DIR))? {
        let entry = entry?;
        let entry_path = entry.path();

        let entry_str = &entry_path.into_os_string().into_string().unwrap();
        let form_id = entry_str.trim_start_matches("submission/");
        let form_name = match name_map.get(&form_id.to_string()) {
            Some(x) => x,
            None => "Missing Form"
        };

        for f in std::fs::read_dir(entry.path())? {
            let f = f?;
            let mut f_str = f.path().into_os_string().into_string().unwrap();
            let f_len = f_str.len();
            f_str.truncate(f_len - 5);

            let prefix = entry.path().into_os_string().into_string().unwrap();
            let submission_id = f_str.trim_start_matches(&prefix);
            let sub_entry = ListSubmissionsEntry {
                submission_id: submission_id.to_string(),
                form_id: form_id.to_string(),
                form_name: form_name.to_string(),
            };

            res.submission_list.push(sub_entry);
        }
    }

    Ok(json!(res))
}

// UI App serving routes
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
                .mount(
                    "/",
                    routes![
                        list_forms,
                        list_submissions,
                        create_form,
                        fetch_form,
                        fetch_submission,
                        submit,
                        show_form,
                        files
                    ],
                )
                .attach(cors)
                .launch();
        }
        Err(err) => {
            panic!("Failed in CORS creation, err: {}", err);
        }
    }
}
