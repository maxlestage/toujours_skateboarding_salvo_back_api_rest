use auth::jwt_auth::{sign_in, JwtClaims, SECRET_KEY};

use database_connection::db_connection::db_connection;
use migration::{Migrator, MigratorTrait};

use queries::data_queries::create_data;
use queries::user_queries::create_user;

use salvo::http::StatusCode;
use salvo::jwt_auth::HeaderFinder;
use salvo::prelude::*;
use salvo::{__private::tracing, handler /* , prelude::* */};

use scraper::thrasher_latest_videos::scraper;


// use scraper::youtube_thrasher_latest_videos::scraper_yt;
use sea_orm::{entity::*, DatabaseConnection};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(default_source(from = "body", format = "json"))]
pub struct User {
    firstname: String,
    lastname: String,
    mail: String,
    password: String,
}

#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(default_source(from = "body", format = "json"))]
pub struct Data {
    title: String,
    description: String,
    path: String,
}

#[handler]
async fn hello_world() -> &'static str {
    "Hello there!"
}

#[handler]
async fn hello_by_id(req: &mut Request) -> String {
    req.params().get("id").cloned().unwrap_or_default()
}

#[handler]
async fn sign_up(user_input: User, res: &mut Response) {
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");

    let user = entities::user::ActiveModel::from_json(json!(user_input)).expect("not valid");

    if create_user(db_connect, user).await.is_some() {
        res.set_status_code(StatusCode::CREATED);
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
    }
}

#[handler]
async fn new_data(user_input: Data, res: &mut Response) {
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
async fn thrasher_latest_videos_crawled(res: &mut Response) {
    res.render(Text::Json(scraper().await))
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("Listening on http://0.0.0.0:7878");
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    Migrator::up(&db_connect, None).await.expect("Error");


    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .with_finders(vec![Box::new(HeaderFinder::new())])
        .with_response_error(true);

    // Define Routing tree
    let routing = Router::new()
        .get(thrasher_latest_videos_crawled)
        .push(Router::with_path("signup").post(sign_up))
        .push(Router::with_path("signin").post(sign_in))
        .push(
            Router::new()
                .path("upload")
                .hoop(auth_handler)
                .get(hello_world)
                .post(new_data)
                .push(Router::with_path("<id>").get(hello_by_id)),
        );

    // Server Ready
    Server::new(TcpListener::bind("0.0.0.0:7878"))
        .serve(routing)
        .await;
}
