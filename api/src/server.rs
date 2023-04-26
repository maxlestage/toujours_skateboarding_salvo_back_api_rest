use crate::handlers::{
    edit_data, hello_by_id, new_data, select_data, sign_up, thrasher_latest_videos_crawled,
};
use auth::jwt_auth::{sign_in, JwtClaims, SECRET_KEY};
use database_connection::db_connection::db_connection;
use migration::{Migrator, MigratorTrait};
use salvo::Router;
use salvo::__private::tracing;
use salvo::jwt_auth::HeaderFinder;
use salvo::prelude::{JwtAuth, TcpListener};
use salvo::Server;
use sea_orm::DatabaseConnection;

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
                .hoop(auth_handler)
                .path("data")
                .post(new_data)
                .push(Router::with_path("<id>").get(select_data))
                .push(Router::with_path("<id>").put(edit_data)),
        )
        .push(
            Router::new()
                .path("hello")
                // .path("upload")
                // .get(hello_world)
                // .post(new_data)
                .push(Router::with_path("<id>").get(hello_by_id)),
        );

    // Server Ready
    Server::new(TcpListener::bind("0.0.0.0:7878"))
        .serve(routing)
        .await;
}
