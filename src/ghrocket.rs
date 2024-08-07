//! # Rocket Module
//!
//! This module contains the Rocket implementation for the OctoApp.
//!
//! ## Example
//!
//! ```rust
//! # #[cfg(feature = "rocket")] {
//! use octoapp::OctoAppConfig;
//!
//! #[rocket::main]
//! async fn main() {
//!     
//! }
//! # }
//! ```
//!

use crate::{events::WebHook, OctoAppError};
use rocket::{
    data::{Data, FromData, Outcome},
    http::Status,
    request::Request,
    response::content,
    State,
};

/// The application state for the OctoApp
///
/// This is used to manage the configuration and other shared state.
pub struct OctoAppState {
    /// The configuration for the OctoApp
    pub config: crate::OctoAppConfig,
}

impl OctoAppState {
    /// Create a new OctoAppState instance
    pub fn new(config: crate::OctoAppConfig) -> Self {
        Self { config }
    }
}

impl From<crate::OctoAppConfig> for OctoAppState {
    fn from(config: crate::OctoAppConfig) -> Self {
        Self::new(config)
    }
}

/// Deserialize a WebHook from a string for Rocket
impl<'r, T: serde::Deserialize<'r>> WebHook<T> {
    fn from_str(s: &'r str) -> Result<Self, crate::OctoAppError> {
        serde_json::from_str(s)
            .map(Self)
            .map_err(|e| crate::OctoAppError::from(e))
    }

    async fn from_data(
        req: &'r ::rocket::request::Request<'_>,
        data: ::rocket::data::Data<'r>,
        appstate: &State<OctoAppState>,
        signature: String,
    ) -> Result<Self, crate::OctoAppError> {
        let limit = req
            .limits()
            .get("json")
            .unwrap_or(::rocket::data::Limits::JSON);

        let string = match data.open(limit).into_string().await {
            Ok(s) if s.is_complete() => s.into_inner(),
            Ok(_) => {
                return Err(crate::OctoAppError::LimitExceeded);
            }
            Err(e) => return Err(crate::OctoAppError::from(e)),
        };

        // Validate the request signature
        if !appstate
            .config
            .webhook_signature_verification(string.as_bytes(), signature)
        {
            // Failed to validate the request signature
            return Err(OctoAppError::SignatureError(
                "Failed to validate the request signature".to_string(),
            ));
        }

        Self::from_str(::rocket::request::local_cache!(req, string))
    }
}

#[rocket::async_trait]
impl<'r, T: serde::Deserialize<'r>> FromData<'r> for WebHook<T> {
    type Error = crate::OctoAppError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        // TODO: This unwrap is not safe!
        let appstate: &State<super::OctoAppState> =
            req.guard::<&State<OctoAppState>>().await.unwrap();

        // Validate the request signature
        let signature: String = match req.headers().get_one("X-Hub-Signature-256") {
            Some(signature) => signature.to_string(),
            None => {
                return Outcome::Error((
                    rocket::http::Status::Unauthorized,
                    OctoAppError::SignatureError("Missing X-Hub-Signature-256 header".to_string()),
                ))
            }
        };

        // // TODO: Is this cloning?
        // let body: String = match data.open(u8::MAX.into()).into_string().await {
        //     Ok(data) => data.to_string(),
        //     Err(_) => {
        //         return Outcome::Error((
        //             rocket::http::Status::InternalServerError,
        //             OctoAppError::UnknownError,
        //         ))
        //     }
        // };

        // Parse the event
        // let event_name = req
        //     .headers()
        //     .get_one("X-GitHub-Event")
        //     .expect("Missing X-GitHub-Event header");

        match Self::from_data(req, data, appstate, signature).await {
            Ok(value) => Outcome::Success(value),
            Err(e) => Outcome::Error((Status::BadRequest, e)),
        }
    }
}

impl<'r, T: serde::Serialize> rocket::response::Responder<'r, 'r> for WebHook<T> {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'r> {
        content::RawJson(serde_json::to_string(&self.0).map_err(|_| Status::InternalServerError)?)
            .respond_to(req)
    }
}
