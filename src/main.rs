use salvo::prelude::*;

#[handler]
async fn hello_world() -> &'static str {
    "Hello world"
}
#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);
    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}
