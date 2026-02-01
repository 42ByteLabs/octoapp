//! Error handling for Hyper integration

use crate::OctoAppError;
use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};

/// Result type for Hyper handlers
pub type OctoAppResult<T> = Result<T, OctoAppError>;

/// Convert OctoAppError to HTTP Response
pub fn error_to_response(error: &OctoAppError) -> Response<Full<Bytes>> {
    let (status, message) = match error {
        OctoAppError::SignatureError(_) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        OctoAppError::LimitExceeded => (StatusCode::PAYLOAD_TOO_LARGE, "Payload too large"),
        OctoAppError::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error"),
        _ => (StatusCode::BAD_REQUEST, "Bad request"),
    };

    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(message)))
        .expect("Failed to build error response")
}
