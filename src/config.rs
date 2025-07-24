//! Configuration management for OTA updates

use crate::error::{ConfigError, Result};
use heapless::String;
use serde::{Deserialize, Serialize};

/// Maximum URL length for OTA endpoints
pub const MAX_URL_LENGTH: usize = 256;

/// OTA client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaConfig {
    /// Base URL for OTA server (e.g., "https://solari.local/ota")
    pub server_url: String<MAX_URL_LENGTH>,
    
    /// Current firmware version
    pub current_version: Version,
    
    /// Device identifier
    pub device_id: String<32>,
    
    /// Update check interval in seconds
    pub check_interval: u32,
    
    /// Retry configuration
    pub retry_config: RetryConfig,
    
    /// Enable automatic updates
    pub auto_update: bool,
}

/// Firmware version representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
    pub build: u32,
}

/// Retry configuration for network operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u8,
    
    /// Initial backoff delay in milliseconds
    pub initial_delay_ms: u32,
    
    /// Maximum backoff delay in milliseconds
    pub max_delay_ms: u32,
    
    /// Backoff multiplier
    pub backoff_multiplier: f32,
}

/// Configuration manager for persistent storage
pub struct ConfigManager<S> {
    storage: S,
    config: OtaConfig,
}

impl OtaConfig {
    /// Create a new OTA configuration with defaults
    pub fn new(server_url: &str) -> Result<Self> {
        let url = String::try_from(server_url)
            .map_err(|_| ConfigError::InvalidUrl)?;
        
        Ok(Self {
            server_url: url,
            current_version: Version::new(0, 1, 0, 0),
            device_id: String::try_from("unknown").unwrap(),
            check_interval: 3600, // 1 hour
            retry_config: RetryConfig::default(),
            auto_update: false,
        })
    }
    
    /// Set the device ID
    pub fn with_device_id(mut self, device_id: &str) -> Result<Self> {
        self.device_id = String::try_from(device_id)
            .map_err(|_| ConfigError::InvalidUrl)?;
        Ok(self)
    }
    
    /// Set the current version
    pub fn with_version(mut self, version: Version) -> Self {
        self.current_version = version;
        self
    }
    
    /// Enable or disable automatic updates
    pub fn with_auto_update(mut self, enabled: bool) -> Self {
        self.auto_update = enabled;
        self
    }
}

impl Version {
    /// Create a new version
    pub const fn new(major: u8, minor: u8, patch: u16, build: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            build,
        }
    }
    
    /// Parse version from string (e.g., "1.2.3-456")
    pub fn parse(version_str: &str) -> Result<Self> {
        // Simplified parsing - in production, use a proper parser
        let parts: heapless::Vec<&str, 4> = version_str
            .split(|c| c == '.' || c == '-')
            .collect();
        
        if parts.len() < 3 {
            return Err(ConfigError::InvalidVersion.into());
        }
        
        Ok(Self {
            major: parts[0].parse().map_err(|_| ConfigError::InvalidVersion)?,
            minor: parts[1].parse().map_err(|_| ConfigError::InvalidVersion)?,
            patch: parts[2].parse().map_err(|_| ConfigError::InvalidVersion)?,
            build: parts.get(3).and_then(|b| b.parse().ok()).unwrap_or(0),
        })
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

impl<S> ConfigManager<S> 
where
    S: embedded_storage_async::nor_flash::NorFlash,
{
    /// Create a new configuration manager
    pub fn new(storage: S, config: OtaConfig) -> Self {
        Self { storage, config }
    }
    
    /// Load configuration from storage
    pub async fn load(&mut self) -> Result<()> {
        // TODO: Implement storage read and deserialization
        Ok(())
    }
    
    /// Save configuration to storage
    pub async fn save(&mut self) -> Result<()> {
        // TODO: Implement serialization and storage write
        Ok(())
    }
    
    /// Get current configuration
    pub fn config(&self) -> &OtaConfig {
        &self.config
    }
    
    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut OtaConfig {
        &mut self.config
    }
}