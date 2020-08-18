use actix_web::client::Client;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
struct Status {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct GitHubStatus {
    status: Status,
}

pub async fn github_status() -> String {
    let client = Client::default();

    let response = client
        .get("https://kctbh9vrtdwd.statuspage.io/api/v2/status.json")
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await;
    let body = response.unwrap().body().await.unwrap();
    let json_string = str::from_utf8(&body).unwrap();
    let gh_status: GitHubStatus = serde_json::from_str(json_string).unwrap();
    gh_status.status.description
}
