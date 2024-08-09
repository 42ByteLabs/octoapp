//! OctoApp Rocket Errors
use rocket::{
    http::Status,
    response::{self, Responder},
    serde::json::Json,
    Request,
};

use crate::OctoAppError;

/// OctoApp Result
pub type OctoAppResult<T> = Result<T, OctoAppError>;

/// API Error Response
#[derive(Responder)]
pub enum ApiResponse {
    /// Unauthorized Response
    #[response(status = 401, content_type = "json")]
    Unauthorized {
        #[allow(missing_docs)]
        inner: (Status, Json<OctoAppApiError>),
    },
    /// Internal Server Error Response
    #[response(status = 500, content_type = "json")]
    InternalServerError {
        #[allow(missing_docs)]
        inner: (Status, Json<OctoAppApiError>),
    },
}

/// OctoApp API Error
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OctoAppApiError {
    /// The status of the API request
    pub status: String,
    /// The error message
    pub message: Option<String>,
}

impl<'r> Responder<'r, 'r> for OctoAppError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
        let status = match self {
            OctoAppError::OctocrabError(_) => Status::InternalServerError,
            OctoAppError::OctocrabInstallationError(_) => Status::InternalServerError,
            _ => Status::BadRequest,
        };

        ApiResponse::InternalServerError {
            inner: (
                status,
                Json(OctoAppApiError {
                    message: Some("Internal Server Error".to_string()),
                    status: self.to_string(),
                }),
            ),
        }
        .respond_to(request)
    }
}

impl From<OctoAppError> for OctoAppApiError {
    fn from(value: OctoAppError) -> Self {
        OctoAppApiError {
            message: Some(value.to_string()),
            status: "error".to_string(),
        }
    }
}
