#![no_std]

//! # Genesis OTA Library
//! 
//! Embedded OTA and configuration management library for ESP32-C3 devices.
//! 
//! ## Features
//! - Secure OTA updates with GPG signature verification
//! - Configuration management with atomic updates
//! - Rollback capability for failed updates
//! - Async/await support via Embassy
//! 
//! ## Example
//! ```no_run
//! use genesis::{OtaClient, OtaConfig};
//! 
//! let config = OtaConfig::new("https://solari.local/ota");
//! let mut client = OtaClient::new(config, public_key);
//! 
//! // Check for updates
//! if let Some(manifest) = client.check_update().await? {
//!     client.download_and_apply(manifest).await?;
//! }
//! ```

// Re-export commonly used types
pub use crate::client::OtaClient;
pub use crate::config::{ConfigManager, OtaConfig};
pub use crate::error::{Error, Result};
pub use crate::manifest::{Manifest, UpdateManifest};
pub use crate::storage::UpdateStorage;
pub use crate::verification::SignatureVerifier;

// Module declarations
pub mod client;
pub mod config;
pub mod error;
pub mod manifest;
pub mod storage;
pub mod verification;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Common imports for no_std
extern crate alloc;

// Embassy async runtime type aliases
pub type Instant = embassy_time::Instant;
pub type Duration = embassy_time::Duration;