use chrono::{DateTime,Utc};

#[derive(Debug,Clone)]
pub struct Secret {
    pub slug: String,
    pub ciphertext: String,
    pub none: Vec<u8>,
    pub key: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}