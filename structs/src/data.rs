use salvo::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(default_source(from = "body", format = "json"))]
pub struct Data {
    user_id: i32,
    title: String,
    description: String,
    path: String,
}
