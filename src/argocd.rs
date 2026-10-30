use std::fs;

use reqwest::{Certificate, Client};
use serde::Deserialize;

// Kubernetes mounts these into the pod for its service account. The token is rotated
// on disk, so both are read again for every request.
const SERVICE_ACCOUNT_DIR: &str = "/var/run/secrets/kubernetes.io/serviceaccount";

// ArgoCD keeps each app's status on its Application object, so it can be read from the
// Kubernetes API with the pod's own service account instead of an ArgoCD token.
const APPLICATIONS_URL: &str =
    "https://kubernetes.default.svc/apis/argoproj.io/v1alpha1/namespaces/argocd/applications";

#[derive(Debug, Deserialize)]
struct Application {
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
    let ca = fs::read(format!("{SERVICE_ACCOUNT_DIR}/ca.crt")).expect("Failed to read cluster CA");
    let token = fs::read_to_string(format!("{SERVICE_ACCOUNT_DIR}/token"))
        .expect("Failed to read service account token");
    let client = Client::builder()
        .add_root_certificate(Certificate::from_pem(&ca).expect("Invalid cluster CA"))
        .build()
        .expect("Failed to build HTTP client");

    let url = format!("{APPLICATIONS_URL}/beatguessr-{env}");
    let app: Application = client
        .get(&url)
        .bearer_auth(token.trim())
        .send()
        .await
        .expect("Failed to send request")
        .error_for_status()
        .expect("Kubernetes API refused the request")
        .json()
        .await
        .expect("Failed to parse response");

    AppStatus {
        healthy: app.status.health.status == "Healthy",
        synced: app.status.sync.status == "Synced",
    }
}
