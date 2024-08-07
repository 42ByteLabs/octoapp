//! Configuration module for the OctoApp
//!
//! ```no_run
//! use octoapp::OctoAppConfig;
//!
//! let config = OctoAppConfig::init()
//!     .app_name("My App")
//!     .app_id(12345)
//!     .client_id("client_id")
//!     .client_secret("client_secret")
//!     .client_key("client_key")
//!     .webhook_secret("webhook_secret")
//!     .build()
//!     .expect("Failed to build config");
//!
//! ```

use std::{fmt::Display, path::PathBuf};

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// OctoApp Configuration
///
/// This struct represents the configuration for the OctoApp
#[derive(Clone)]
pub struct OctoAppConfig {
    /// The name of the app
    app_name: Option<String>,
    /// The App ID
    app_id: usize,
    /// The client id for the app
    client_id: String,
    /// The secret for the app
    client_secret: String,
    /// The private key for the app
    client_key: Option<jsonwebtoken::EncodingKey>,
    /// Optional webhook secret for verifying incoming webhooks
    webhook_secret: Option<String>,
}

impl OctoAppConfig {
    /// Initialize a new OctoAppConfigBuilder.
    ///
    /// This will load the configuration from the environment variables but
    /// can be overridden by calling the builder methods.
    ///
    pub fn init() -> OctoAppConfigBuilder {
        OctoAppConfigBuilder::default()
    }
    /// Get the app name
    pub fn app_name(&self) -> Option<&String> {
        self.app_name.as_ref()
    }
    /// Get the app id
    pub fn app_id(&self) -> usize {
        self.app_id
    }
    /// Get the client id
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    /// Get the client secret
    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }
    /// Get the client key
    pub fn client_key(&self) -> Option<&jsonwebtoken::EncodingKey> {
        self.client_key.as_ref()
    }
    /// Get the webhook secret
    pub fn webhook_secret(&self) -> Option<&String> {
        self.webhook_secret.as_ref()
    }
    /// Create an Octocrab instance using the app configuration
    #[cfg(feature = "octocrab")]
    pub fn octocrab(&self) -> Result<octocrab::Octocrab, crate::OctoAppError> {
        use crate::OctoAppError;

        if let Some(key) = &self.client_key {
            let oc = octocrab::OctocrabBuilder::new()
                .app(octocrab::models::AppId(self.app_id as u64), key.clone())
                .build()?;
            oc.installation(octocrab::models::InstallationId(self.app_id as u64));
            Ok(oc)
        } else {
            Err(OctoAppError::MissingField("Client Key".to_string()))
        }
    }

    /// Verify the signature of the incoming webhook
    ///
    /// Signature is expected to be in the format `sha256=hex(signature)`
    pub fn webhook_signature_verification(&self, data: &[u8], signature: String) -> bool {
        if let Some(secret) = &self.webhook_secret {
            if signature.starts_with("sha256=") {
                // Skip the prefix
                let hex_signature: String = signature.chars().skip(7).collect();

                let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
                mac.update(data);

                let hex_result = hex::encode(mac.finalize().into_bytes());

                tracing::debug!(
                    "WebHook({:?}) == Signature({:?})",
                    hex_signature,
                    hex_result
                );

                return hex_result == hex_signature;
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Display for OctoAppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Don't print sensitive information when displaying the config (debug only)
        write!(
            f,
            "OctoAppConfig {{ app_name: {:?}, app_id: {} }}",
            self.app_name, self.app_id
        )
    }
}

/// OctoApp Configuration Builder
///
/// This struct is used to build the OctoAppConfig struct
/// using the builder pattern.
#[derive(Debug, Clone)]
pub struct OctoAppConfigBuilder {
    app_name: Option<String>,
    app_id: Option<usize>,

    client_id: Option<String>,
    client_secret: Option<String>,
    client_key: Option<String>,
    client_key_path: Option<PathBuf>,

    webhook_secret: Option<String>,
}

impl OctoAppConfigBuilder {
    /// Set the app name
    pub fn app_name(mut self, app_name: &str) -> Self {
        self.app_name = Some(app_name.to_string());
        self
    }
    /// Set the app id
    pub fn app_id(mut self, app_id: usize) -> Self {
        self.app_id = Some(app_id);
        self
    }
    /// Set the client id
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_string());
        self
    }
    /// Set the client secret
    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = Some(client_secret.to_string());
        self
    }
    /// Set the client key
    pub fn client_key(mut self, client_key: &str) -> Self {
        self.client_key = Some(client_key.to_string());
        self
    }
    /// Set the client key path
    pub fn client_key_path(mut self, client_key_path: impl Into<PathBuf>) -> Self {
        self.client_key_path = Some(client_key_path.into());
        self
    }
    /// Set the webhook secret
    pub fn webhook_secret(mut self, webhook_secret: &str) -> Self {
        self.webhook_secret = Some(webhook_secret.to_string());
        self
    }
    /// Build the OctoAppConfig
    pub fn build(self) -> Result<OctoAppConfig, crate::OctoAppError> {
        self.try_into()
    }
}

impl TryFrom<OctoAppConfigBuilder> for OctoAppConfig {
    type Error = crate::OctoAppError;

    fn try_from(value: OctoAppConfigBuilder) -> Result<Self, Self::Error> {
        let client_key: Option<jsonwebtoken::EncodingKey> =
            if let Some(client_key_path) = value.client_key_path {
                let data = std::fs::read_to_string(client_key_path)?;
                Some(jsonwebtoken::EncodingKey::from_rsa_pem(data.as_bytes())?)
            } else if let Some(client_key) = &value.client_key {
                Some(jsonwebtoken::EncodingKey::from_rsa_pem(
                    client_key.clone().as_bytes(),
                )?)
            } else {
                return Err(crate::OctoAppError::MissingField("Client Key".to_string()));
            };

        Ok(OctoAppConfig {
            app_name: value.app_name,
            app_id: value
                .app_id
                .ok_or(crate::OctoAppError::MissingField("AppID".to_string()))?,
            client_id: value
                .client_id
                .ok_or(crate::OctoAppError::MissingField("Client ID".to_string()))?,
            client_secret: value
                .client_secret
                .ok_or(crate::OctoAppError::MissingField(
                    "Client Secret".to_string(),
                ))?,
            client_key,
            webhook_secret: value.webhook_secret,
        })
    }
}

impl Default for OctoAppConfigBuilder {
    fn default() -> Self {
        let app_name: Option<String> = std::env::var("APP_NAME").ok();
        let app_id: Option<usize> = std::env::var("APP_ID").ok().map(|s| s.parse().unwrap());

        let client_id: Option<String> = std::env::var("CLIENT_ID").ok();
        let client_secret: Option<String> = std::env::var("CLIENT_SECRET").ok();
        let client_key: Option<String> = std::env::var("CLIENT_KEY").ok();
        let client_key_path: Option<PathBuf> =
            std::env::var("PRIVATE_KEY_PATH").ok().map(|s| s.into());

        let webhook_secret: Option<String> = std::env::var("WEBHOOK_SECRET").ok();

        OctoAppConfigBuilder {
            app_name,
            app_id,
            client_id,
            client_secret,
            client_key,
            client_key_path,
            webhook_secret,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_verification() {
        let config = OctoAppConfig {
            app_name: None,
            app_id: 12345,
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            client_key: None,
            // This is a test secret, don't use this in production
            webhook_secret: Some("ThisIsASecret".to_string()),
        };

        let data = b"Hello, World!";

        assert!(config.webhook_signature_verification(
            data,
            "sha256=8f0f4676fdd5091bb3d5eb610a35434412970971ada809fa3fb3680d5dfff024".to_string(),
        ));
    }
}