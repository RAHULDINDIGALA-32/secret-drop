pub mod request;
pub mod response;

pub use request::CreateSecretRequest;

pub use response::{
    CreateSecretResponse,
    RevealResponse,
    ErrorResponse,
}