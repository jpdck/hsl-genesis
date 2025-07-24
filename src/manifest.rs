//! Update manifest structures and parsing

use crate::config::Version;
use crate::error::{ManifestError, Result};
use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

/// Maximum number of files in a single update
pub const MAX_UPDATE_FILES: usize = 8;

/// Update manifest describing available firmware
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifest {
    /// Manifest format version
    pub manifest_version: u8,
    
    /// Firmware version
    pub version: Version,
    
    /// Release timestamp (Unix epoch)
    pub timestamp: u64,
    
    /// Release notes or description
    pub description: String<256>,
    
    /// Minimum required version for this update
    pub min_version: Option<Version>,
    
    /// Files included in this update
    pub files: Vec<UpdateFile, MAX_UPDATE_FILES>,
    
    /// GPG signature of the manifest
    pub signature: Signature,
    
    /// Update urgency level
    pub urgency: UpdateUrgency,
    
    /// Rollback information
    pub rollback: RollbackInfo,
}

/// Individual file in an update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFile {
    /// File type (firmware, config, etc.)
    pub file_type: FileType,
    
    /// Target partition or location
    pub target: String<32>,
    
    /// Download URL (relative to base URL)
    pub url: String<128>,
    
    /// File size in bytes
    pub size: u32,
    
    /// SHA256 hash of the file
    pub sha256: [u8; 32],
    
    /// Compression type
    pub compression: CompressionType,
}

/// Update urgency levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateUrgency {
    /// Optional update
    Low,
    /// Recommended update
    Normal,
    /// Important update (security or critical bug fix)
    High,
    /// Critical update (must install ASAP)
    Critical,
}

/// File types in updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    /// Main firmware binary
    Firmware,
    /// Configuration data
    Config,
    /// Bootloader update
    Bootloader,
    /// File system image
    FileSystem,
}

/// Compression types supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionType {
    /// No compression
    None,
    /// Gzip compression
    Gzip,
    /// Zstd compression
    Zstd,
}

/// GPG signature information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// Signature algorithm
    pub algorithm: SignatureAlgorithm,
    
    /// Key ID that created the signature
    pub key_id: [u8; 8],
    
    /// The actual signature bytes
    pub data: Vec<u8, 256>,
}

/// Supported signature algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    /// Ed25519 signature
    Ed25519,
    /// RSA-2048 signature
    Rsa2048,
}

/// Rollback configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RollbackInfo {
    /// Enable automatic rollback on failure
    pub enabled: bool,
    
    /// Number of boot attempts before rollback
    pub max_attempts: u8,
    
    /// Watchdog timeout in seconds
    pub watchdog_timeout: u32,
}

/// Manifest validation and parsing
pub struct Manifest;

impl Manifest {
    /// Parse and validate a manifest from raw bytes
    pub fn parse(data: &[u8]) -> Result<UpdateManifest> {
        // Deserialize using postcard
        let manifest: UpdateManifest = postcard::from_bytes(data)
            .map_err(|_| ManifestError::InvalidFormat)?;
        
        // Validate manifest version
        if manifest.manifest_version != 1 {
            return Err(ManifestError::UnsupportedVersion.into());
        }
        
        // Validate required fields
        if manifest.files.is_empty() {
            return Err(ManifestError::InvalidFormat.into());
        }
        
        Ok(manifest)
    }
    
    /// Create a manifest for local testing
    #[cfg(feature = "test")]
    pub fn create_test_manifest() -> UpdateManifest {
        UpdateManifest {
            manifest_version: 1,
            version: Version::new(1, 0, 0, 1),
            timestamp: 1234567890,
            description: String::try_from("Test update").unwrap(),
            min_version: None,
            files: Vec::new(),
            signature: Signature {
                algorithm: SignatureAlgorithm::Ed25519,
                key_id: [0; 8],
                data: Vec::new(),
            },
            urgency: UpdateUrgency::Normal,
            rollback: RollbackInfo::default(),
        }
    }
}

impl UpdateManifest {
    /// Check if this update can be applied to the current version
    pub fn is_applicable(&self, current_version: &Version) -> bool {
        // Check minimum version requirement
        if let Some(min_ver) = &self.min_version {
            if current_version < min_ver {
                return false;
            }
        }
        
        // Don't downgrade
        &self.version > current_version
    }
    
    /// Get the primary firmware file from the manifest
    pub fn firmware_file(&self) -> Option<&UpdateFile> {
        self.files
            .iter()
            .find(|f| f.file_type == FileType::Firmware)
    }
    
    /// Calculate total download size
    pub fn total_size(&self) -> u32 {
        self.files.iter().map(|f| f.size).sum()
    }
}

impl Default for RollbackInfo {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 3,
            watchdog_timeout: 300, // 5 minutes
        }
    }
}