use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(default_source(from = "body", format = "json"))]
pub struct User {
    firstname: String,
    lastname: String,
    mail: String,
    password: String,
    role: Option<String>,
}

#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(default_source(from = "body", format = "json"))]
pub struct Data {
    title: String,
    description: String,
    path: String,
}
