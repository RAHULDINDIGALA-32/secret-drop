use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CreateSecretResponse {
    pub slug: String,
    pub url: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RevealResponse {
    pub secret: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}