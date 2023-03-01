use salvo::prelude::*;

#[handler]
async fn hello_world() -> &'static str {
    "Hello there!"
}

#[handler]

async fn hello_by_id(req: &mut Request) -> String {
    req.params().get("id").cloned().unwrap_or_default()
}

#[tokio::main]
pub async fn main() {
    database_connection::main().await.expect("Error");

    // Define Routing tree
    let routing = Router::with_path("")
        .get(hello_world)
        .push(Router::with_path("<id>").get(hello_by_id));

    // Server Ready
    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(routing)
        .await;
}
