//! # Hyper Module
//!
//! This module provides hyper integration for OctoApp webhook handling.
//!
//! ## Example
//!
//! ```rust,no_run
//! # #[cfg(feature = "hyper")] {
//! use octoapp::{OctoAppConfig, HyperWebhookHandler};
//! use octoapp::events::Event;
//! use hyper::server::conn::http1;
//! use hyper_util::rt::TokioIo;
//! use tokio::net::TcpListener;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = OctoAppConfig::init()
//!         .app_id(12345)
//!         .webhook_secret("my-secret")
//!         .build()?;
//!     
//!     let handler = HyperWebhookHandler::new(config)
//!         .path("/github")
//!         .on_event(|webhook: octoapp::WebHook<Event>| async move {
//!             println!("Received event: {:?}", webhook.into_inner());
//!             Ok(())
//!         });
//!     
//!     handler.serve("127.0.0.1:8000").await?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{events::WebHook, OctoAppConfig, OctoAppError};
use http_body_util::{BodyExt, Full};
use hyper::{body::Bytes, body::Incoming, Method, Request, Response, StatusCode};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

pub mod errors;

pub use errors::OctoAppResult;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
type HandlerFn<T> = Arc<dyn Fn(WebHook<T>) -> BoxFuture<Result<(), OctoAppError>> + Send + Sync>;

/// The hyper webhook handler for OctoApp
///
/// This provides a user-friendly API for handling GitHub webhooks with hyper.
pub struct HyperWebhookHandler<T> {
    config: Arc<OctoAppConfig>,
    path: String,
    handler: Option<HandlerFn<T>>,
}

impl<T: serde::de::DeserializeOwned + Send + 'static> HyperWebhookHandler<T> {
    /// Create a new HyperWebhookHandler instance
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[cfg(feature = "hyper")] {
    /// use octoapp::{OctoAppConfig, HyperWebhookHandler};
    /// # async fn example() {
    /// let config = OctoAppConfig::init()
    ///     .app_id(12345)
    ///     .webhook_secret("test-secret")
    ///     .build()
    ///     .unwrap();
    /// let handler = HyperWebhookHandler::<octoapp::events::Event>::new(config);
    /// # }
    /// # }
    /// ```
    pub fn new(config: OctoAppConfig) -> Self {
        Self {
            config: Arc::new(config),
            path: "/".to_string(),
            handler: None,
        }
    }

    /// Set the webhook endpoint path
    ///
    /// Default is `/`. Set to `/github` or any other path as needed.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Register an event handler function
    ///
    /// The handler receives a `WebHook<T>` and should return a `Result<(), OctoAppError>`.
    pub fn on_event<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(WebHook<T>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), OctoAppError>> + Send + 'static,
    {
        self.handler = Some(Arc::new(move |webhook| Box::pin(handler(webhook))));
        self
    }

    /// Start the hyper server on the specified address
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[cfg(feature = "hyper")] {
    /// # use octoapp::{OctoAppConfig, HyperWebhookHandler};
    /// # async fn example() {
    /// # let config = OctoAppConfig::init().app_id(12345).webhook_secret("test").build().unwrap();
    /// let handler = HyperWebhookHandler::<octoapp::events::Event>::new(config)
    ///     .path("/github")
    ///     .on_event(|webhook| async move { Ok(()) });
    /// // handler.serve("127.0.0.1:8000").await.unwrap();
    /// # }
    /// # }
    /// ```
    pub async fn serve(self, addr: impl Into<String>) -> Result<(), OctoAppError> {
        let addr: SocketAddr = addr.into().parse().map_err(|e: std::net::AddrParseError| {
            OctoAppError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            ))
        })?;

        let listener = tokio::net::TcpListener::bind(addr).await?;

        tracing::info!("Hyper server listening on http://{}{}", addr, self.path);

        let handler = Arc::new(self);

        loop {
            let (stream, _) = listener.accept().await?;

            let io = hyper_util::rt::TokioIo::new(stream);
            let handler = handler.clone();

            tokio::spawn(async move {
                let service = hyper::service::service_fn(move |req| {
                    let handler = handler.clone();
                    async move { handler.handle_request(req).await }
                });

                if let Err(err) = hyper::server::conn::http1::Builder::new()
                    .serve_connection(io, service)
                    .await
                {
                    tracing::error!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    async fn handle_request(
        &self,
        req: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>, hyper::Error> {
        // Only accept POST requests to the webhook path
        if req.method() != Method::POST || req.uri().path() != self.path {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not Found")))
                .expect("Failed to build NOT_FOUND response"));
        }

        // Extract signature header
        let signature = match req.headers().get("X-Hub-Signature-256") {
            Some(sig) => match sig.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Full::new(Bytes::from("Invalid signature header")))
                        .expect("Failed to build BAD_REQUEST response"));
                }
            },
            None => {
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Full::new(Bytes::from("Missing X-Hub-Signature-256 header")))
                    .expect("Failed to build UNAUTHORIZED response"));
            }
        };

        // Read body
        let body_bytes = match req.collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(_) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::from("Failed to read body")))
                    .expect("Failed to build BAD_REQUEST response"));
            }
        };

        // Verify signature
        if !self
            .config
            .webhook_signature_verification(&body_bytes, signature)
        {
            tracing::warn!("Signature verification failed");
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Full::new(Bytes::from("Signature verification failed")))
                .expect("Failed to build UNAUTHORIZED response"));
        }

        // Parse webhook
        let body_str = match std::str::from_utf8(&body_bytes) {
            Ok(s) => s,
            Err(_) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::from("Invalid UTF-8")))
                    .expect("Failed to build BAD_REQUEST response"));
            }
        };

        let webhook = match parse_webhook(body_str) {
            Ok(wh) => wh,
            Err(e) => {
                tracing::error!("Failed to parse webhook: {:?}", e);
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::from("Invalid webhook payload")))
                    .expect("Failed to build BAD_REQUEST response"));
            }
        };

        // Call handler if registered
        if let Some(ref handler) = self.handler {
            match handler(webhook).await {
                Ok(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Full::new(Bytes::from("OK")))
                        .expect("Failed to build OK response"));
                }
                Err(e) => {
                    tracing::error!("Handler error: {:?}", e);
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Full::new(Bytes::from("Internal server error")))
                        .expect("Failed to build INTERNAL_SERVER_ERROR response"));
                }
            }
        }

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Full::new(Bytes::from("OK")))
            .expect("Failed to build OK response"))
    }
}

/// Deserialize a WebHook from a string
fn parse_webhook<T: serde::de::DeserializeOwned>(s: &str) -> Result<WebHook<T>, OctoAppError> {
    // Extract installation ID
    #[derive(serde::Deserialize)]
    #[non_exhaustive]
    struct ReqBlob {
        installation: InsBlob,
    }

    #[derive(serde::Deserialize)]
    #[non_exhaustive]
    struct InsBlob {
        id: u64,
    }

    let id: u64 = match serde_json::from_str::<ReqBlob>(s) {
        Ok(installation) => installation.installation.id,
        Err(_) => 0,
    };

    serde_json::from_str(s)
        .map(|value| WebHook(value, id))
        .map_err(OctoAppError::from)
}

#[cfg(feature = "octocrab")]
impl<T> WebHook<T> {
    /// Get an octocrab client scoped to the webhook's installation
    ///
    /// # Example
    /// ```rust,ignore
    /// # use octoapp::{OctoAppConfig, WebHook, events::Event};
    /// # async fn example(webhook: WebHook<Event>) {
    /// # let config = OctoAppConfig::init().app_id(12345).webhook_secret("test").build().unwrap();
    /// let octo = webhook.octocrab_from_config(&config).await.unwrap();
    /// # }
    /// ```
    pub async fn octocrab_from_config(
        &self,
        config: &OctoAppConfig,
    ) -> Result<octocrab::Octocrab, OctoAppError> {
        let id = self.installation();
        if id == 0 {
            return Err(OctoAppError::OctocrabInstallationError(id));
        }
        config.octocrab_by_installation(id).await
    }
}
