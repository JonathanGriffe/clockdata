use std::env;

use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ArgoAppResponse {
    status: Status,
}

#[derive(Debug, Deserialize)]
struct Status {
    health: Health,
    sync: Sync,
}

#[derive(Debug, Deserialize)]
struct Health {
    status: String,
}

#[derive(Debug, Deserialize)]
struct Sync {
    status: String,
}

#[derive(Debug)]
pub struct AppStatus {
    pub healthy: bool,
    pub synced: bool,
}

pub async fn get_argocd_status(env: &str) -> AppStatus {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to build HTTP client");
    let argocd_uri = env::var("ARGOCD_URI").expect("ARGOCD_URI must be set");
    let token = std::env::var("ARGOCD_TOKEN").expect("ARGOCD_TOKEN environment variable not set");
    println!("{}", argocd_uri);
    println!("{}", token);
    let url = format!("{}/api/v1/applications/beatguessr-{}", argocd_uri, env);
    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .expect("Failed to send request");
    let app_response: ArgoAppResponse = response.json().await.expect("Failed to parse response");

    AppStatus {
        healthy: app_response.status.health.status == "Healthy",
        synced: app_response.status.sync.status == "Synced",
    }
}
