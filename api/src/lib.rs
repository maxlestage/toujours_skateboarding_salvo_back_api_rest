use salvo::prelude::*;

#[handler]
async fn hello_world() -> &'static str {
    "tottotootootototootoot"
}
#[tokio::main]
pub async fn main() {
    database_connection::main().await.expect("Error");
    let router = Router::new().get(hello_world);
    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}
