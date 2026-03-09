use std::{env, net::SocketAddr};

use axum::{Extension, Router, middleware::from_fn, routing::get};
use clockdata::routes::{require_token, route};
use elasticsearch::{Elasticsearch, http::transport::Transport};

#[tokio::main]
async fn main() {
    let transport = Transport::single_node(
        &env::var("ELASTICSEARCH_URI").expect("ELASTICSEARCH_URI must be set"),
    )
    .unwrap();
    let client = Elasticsearch::new(transport);

    let app = Router::new()
        .route("/", get(route))
        .layer(Extension(client))
        .layer(from_fn(require_token));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
