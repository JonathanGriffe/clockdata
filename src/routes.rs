use axum::response::IntoResponse;

pub async fn route() -> impl IntoResponse {
    "Hello, world!"
}
