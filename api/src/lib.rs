use auth::jwt_auth::{sign_in, JwtClaims, SECRET_KEY};
use database_connection::db_connection::db_connection;
use queries::*;
use salvo::http::{response, StatusCode};
use salvo::jwt_auth::HeaderFinder;
use salvo::prelude::*;
use salvo::{__private::tracing, handler /* , prelude::* */};
use scraper::thrasher_latest_videos::scraper;
use sea_orm::{entity::*, DatabaseConnection};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(default_source(from = "body", format = "json"))]
struct User {
    firstname: String,
    lastname: String,
    mail: String,
    password: String,
    role: Option<String>,
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
async fn scraptest(/* res: &mut Response */) -> anyhow::Result<()> {
    println!("{:#?}", scraper().await);

    // res.render(Text::Json(scraper().await));
    Ok(())
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("Listening on http://0.0.0.0:7878");

    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .with_finders(vec![Box::new(HeaderFinder::new())])
        .with_response_error(true);

    // Define Routing tree
    let routing = Router::new()
        .get(scraptest)
        .push(Router::with_path("signup").post(sign_up))
        .push(Router::with_path("signin").post(sign_in))
        .push(
            Router::new()
                .path("hello")
                .hoop(auth_handler)
                .get(hello_world)
                .push(Router::with_path("<id>").get(hello_by_id)),
        );

    // Server Ready
    Server::new(TcpListener::bind("0.0.0.0:7878"))
        .serve(routing)
        .await;
}
