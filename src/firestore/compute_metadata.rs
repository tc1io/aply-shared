use super::FsClientError;
use hyper::{body, Body, Client, Method, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: u32,
    token_type: String,
}

/// TODO doc
pub async fn get_token() -> std::result::Result<String, FsClientError> {
    let bytes = get_metadata(
        "instance/service-accounts/default/token?scopes=https://www.googleapis.com/auth/datastore",
    )
        .await?;
    let body: TokenResponse = serde_json::from_slice(&bytes.to_vec())?;
    let token = body.access_token;
    Ok(token)
}

/// TODO doc
pub async fn get_project_id() -> std::result::Result<String, FsClientError> {
    let bytes = get_metadata("project/project-id").await?;
    let project_id = String::from_utf8(bytes.to_vec())?;
    Ok(project_id)
}

/// TODO doc
pub async fn get_metadata(entry: &'static str) -> std::result::Result<body::Bytes, FsClientError> {
    let request = Request::builder()
        .method(Method::GET)
        .uri(format!(
            "http://metadata.google.internal/computeMetadata/v1/{}",
            entry
        ))
        .header("Metadata-Flavor", "Google")
        .body(Body::empty())?;

    let client = Client::new();
    let response = client.request(request).await?;
    let bytes = body::to_bytes(response.into_body()).await?;
    Ok(bytes)
}