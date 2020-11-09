use std::collections::HashMap;

// Create form structs
#[derive(Serialize, Deserialize)]
pub struct Schema {
    pub contents: SchemaContents,
}

#[derive(Serialize, Deserialize)]
pub struct SchemaContents {
    pub schema_name: String,
    pub schema: HashMap<String, SchemaEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct SchemaEntry {
    pub label: String,
    pub input_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewFormRes {
    pub form_id: String,
}

// Show form structs and route
#[derive(Serialize, Deserialize)]
pub struct FetchFormReq {
    pub form_id: String,
}

// Form submission structs
#[derive(Serialize, Deserialize)]
pub struct CreateSubmissionReq {
    pub form_id: String,
    pub submission: HashMap<String, SubmissionEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmissionEntry {
    pub label: String,
    pub input_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSubmissionRes {
    pub submission_id: String,
}

// Show submission structs 
#[derive(Serialize, Deserialize)]
pub struct FetchSubmissionReq {
    pub form_id: String,
    pub submission_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct FetchSubmissionRes {
    pub contents: SubmissionContents,
}

#[derive(Serialize, Deserialize)]
pub struct SubmissionContents {
    pub schema_name: String,
    pub submission: HashMap<String, SubmissionEntry>,
}

// List Forms structs
#[derive(Serialize, Deserialize)]
pub struct ListFormsRes {
    pub form_list: Vec<ListFormEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct ListFormEntry {
    pub form_id: String,
    pub form_name: String,
}

// List Submissions structs
#[derive(Serialize, Deserialize)]
pub struct ListSubmisionsRes {
    pub submission_list: Vec<ListSubmissionsEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct ListSubmissionsEntry {
    pub submission_id: String,
    pub form_id: String,
    pub form_name: String,
}