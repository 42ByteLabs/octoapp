//! Error handling for the OctoApp
use thiserror::Error;

/// OcotApp Error
///
/// This error type is used to represent all possible errors that can occure in the OctoApp
#[derive(Error, Debug)]
pub enum OctoAppError {
    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Signature Errors
    #[error("Signature Error: {0}")]
    SignatureError(String),

    /// Webhook Secret Error
    #[error("Webhook Secret Error: {0}")]
    WebhookSecretError(String),

    /// IO Error
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    /// Octocrab Error
    #[cfg(feature = "octocrab")]
    #[error("Octocrab Error: {0}")]
    OctocrabError(#[from] octocrab::Error),

    /// Serde Error
    #[error("JSON Serde Error: {0}")]
    JsonSerializationError(#[from] serde_json::Error),
    /// JWT Error
    #[error("Json Web Token Error: {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),

    /// Limit Exceeded
    #[error("Data limit exceeded")]
    LimitExceeded,

    /// Unknown Error
    #[error("Unknown Error")]
    UnknownError,
}
