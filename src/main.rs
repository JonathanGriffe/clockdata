use std::net::SocketAddr;

use axum::{Router, routing::get};
use clockdata::routes::route;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(route));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
