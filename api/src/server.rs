use crate::handlers::{
    deleteted_data, edit_data, getall_data, hello_by_id, new_data, select_data, sign_up,
    thrasher_latest_videos_crawled,
};
use auth::jwt_auth::{sign_in, JwtClaims, SECRET_KEY};
use database_connection::db_connection::db_connection;
use migration::{Migrator, MigratorTrait};
use salvo::Router;
use salvo::__private::tracing;
use salvo::handler;
use salvo::jwt_auth::HeaderFinder;
// use salvo::prelude::{JwtAuth, TcpListener};
use salvo::cors::Cors;
use salvo::prelude::*;
use salvo::Server;
use sea_orm::DatabaseConnection;

#[handler]
pub async fn empty(_req: &mut Request, res: &mut Response) {
    res.set_status_code(StatusCode::OK)
}

pub async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("Listening on http://0.0.0.0:7878");

    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    Migrator::up(&db_connect, None).await.expect("Error");

    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .with_finders(vec![Box::new(HeaderFinder::new())])
        .with_response_error(true);

    let cors_handler: Cors = Cors::builder()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec![
            "CONTENT-TYPE",
            "Access-Control-Request-Method",
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers",
            "Access-Control-Max-Age",
        ])
        .build();

    // Define Routing tree
    let routing = Router::new()
        .push(Router::with_path("<**rest>").options(empty))
        .hoop(cors_handler)
        .get(thrasher_latest_videos_crawled)
        .push(Router::with_path("signup").post(sign_up))
        .push(Router::with_path("signin").post(sign_in))
        .push(
            Router::new()
                .hoop(auth_handler)
                .path("data")
                .get(getall_data)
                .post(new_data)
                .push(
                    Router::with_path("<id>")
                        .get(select_data)
                        .put(edit_data)
                        .delete(deleteted_data),
                ), // .push(Router::with_path("<id>").put(edit_data)),
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
