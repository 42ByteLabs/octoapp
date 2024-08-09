#![allow(dead_code)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "octocrab")]
extern crate octocrab;

pub mod config;
pub mod error;
#[cfg(feature = "octocrab")]
pub mod events;

#[cfg(feature = "rocket")]
pub mod ghrocket;

pub use config::OctoAppConfig;
pub use error::OctoAppError;
#[cfg(feature = "octocrab")]
pub use events::WebHook;

#[cfg(feature = "rocket")]
pub use crate::ghrocket::{OctoAppResult, OctoAppState};

#[doc(hidden)]
pub mod prelude {
    pub use crate::config::OctoAppConfig;
    pub use crate::error::OctoAppError;
    #[cfg(feature = "octocrab")]
    pub use crate::events::{Event, WebHook};

    // Re-export payloads
    #[cfg(feature = "octocrab")]
    pub use crate::events::payloads::*;

    #[cfg(feature = "rocket")]
    pub use crate::ghrocket::{OctoAppResult, OctoAppState};
}
