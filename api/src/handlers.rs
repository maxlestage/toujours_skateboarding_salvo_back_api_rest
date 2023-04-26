use salvo::http::StatusCode;

use salvo::prelude::*;

// use scraper::youtube_thrasher_latest_videos::scraper_yt;
use sea_orm::{entity::*, DatabaseConnection};

use serde_json::json;

use database_connection::db_connection::db_connection;

use queries::data_queries::{create_data, get_data, update_data};
use queries::user_queries::create_user;
use scraper::thrasher_latest_videos::scraper;
use structs::data::Data;
use structs::user::User;

#[handler]
pub async fn hello_world() -> &'static str {
    "Hello there!"
}

#[handler]
pub async fn hello_by_id(req: &mut Request) -> String {
    req.params().get("id").cloned().unwrap_or_default()
}

#[handler]
pub async fn sign_up(user_input: User, res: &mut Response) {
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");

    let user = entities::user::ActiveModel::from_json(json!(user_input)).expect("not valid");

    if create_user(db_connect, user).await.is_some() {
        res.set_status_code(StatusCode::CREATED);
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
    }
}

#[handler]
pub async fn new_data(user_input: Data, res: &mut Response) {
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");

    let created_data =
        entities::data::ActiveModel::from_json(json!(user_input)).expect("not valid");

    if create_data(db_connect, created_data).await.is_some() {
        res.set_status_code(StatusCode::CREATED);
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
    }
}

#[handler]
pub async fn select_data(req: &mut Request, res: &mut Response) {
    let id = req.param::<i32>("id").unwrap();
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");

    let selected_data = get_data(db_connect, id).await;
    if selected_data.is_some() {
        res.render(Json(selected_data));
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}

#[handler]
pub async fn edit_data(req: &mut Request, user_input: Data, res: &mut Response) {
    let id = req.param::<i32>("id").unwrap_or_default();
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    let data = entities::data::ActiveModel::from_json(json!(user_input)).expect("not valid");
    if update_data(db_connect, id, data).await.is_some() {
        res.set_status_code(StatusCode::OK);
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}

#[handler]
pub async fn thrasher_latest_videos_crawled(res: &mut Response) {
    res.render(Text::Json(scraper().await))
}
