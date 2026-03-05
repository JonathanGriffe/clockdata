use std::{collections::HashMap, env};

use axum::{
    Extension, Json,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use elasticsearch::Elasticsearch;
use serde::Serialize;

use crate::{argocd::get_argocd_status, es::count_distinct_values};

#[derive(Debug, Serialize)]
struct AppFullStatus {
    healthy: bool,
    synced: bool,
    user_count: u64,
}

pub async fn require_token(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Get Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Expect format: "Bearer <TOKEN>"
    if token != format!("Bearer {}", env::var("TOKEN").expect("TOKEN must be set")) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

pub async fn route(Extension(client): Extension<Elasticsearch>) -> impl IntoResponse {
    let mut full_statuses: HashMap<String, AppFullStatus> = HashMap::new();
    for env in env::var("ENVS").expect("ENVS must be set").split(',') {
        let status = get_argocd_status(&env).await;
        let user_count = count_distinct_values(&env, &client).await;
        full_statuses.insert(
            env.to_string(),
            AppFullStatus {
                healthy: status.healthy,
                synced: status.synced,
                user_count: user_count,
            },
        );
    }
    Json(full_statuses)
}
