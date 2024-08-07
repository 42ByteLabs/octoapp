#![allow(dead_code)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
extern crate octocrab;

pub mod config;
pub mod error;
pub mod events;

#[cfg(feature = "rocket")]
pub mod ghrocket;

pub use config::OctoAppConfig;
pub use error::OctoAppError;
pub use events::WebHook;

#[cfg(feature = "rocket")]
pub use ghrocket::OctoAppState;

#[doc(hidden)]
pub mod prelude {
    pub use crate::config::OctoAppConfig;
    pub use crate::error::OctoAppError;
    pub use crate::events::{Event, WebHook};

    // Re-export payloads
    pub use crate::events::payloads::*;

    #[cfg(feature = "rocket")]
    pub use crate::ghrocket::OctoAppState;
}
