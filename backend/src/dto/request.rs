use serde::Deserialize;
use validator::Validator;

#[derive(Debug, Deserialize, Validator)]
pub struct CreateSecretRequest {
    #[validator(length(min = 1, max = 10000, message = "Secret must be between 1 and 10000 characters long"))]
    pub secret: String,

    #[validator(range(min = 60, max = 86400, message = "Expiration time must be between 60 and 86400 seconds"))]
    pub ttl_seconds: Option<u64>,
}