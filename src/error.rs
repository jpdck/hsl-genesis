//! Error types for Genesis OTA library

use core::fmt;

/// Result type alias for Genesis operations
pub type Result<T> = core::result::Result<T, Error>;

/// Main error type for Genesis operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Network-related errors
    Network(NetworkError),
    /// Storage operation errors
    Storage(StorageError),
    /// Signature verification errors
    Verification(VerificationError),
    /// Configuration errors
    Config(ConfigError),
    /// Update manifest errors
    Manifest(ManifestError),
    /// General OTA process errors
    Ota(OtaError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    ConnectionFailed,
    Timeout,
    InvalidResponse,
    HttpError(u16), // HTTP status code
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    ReadFailed,
    WriteFailed,
    EraseFailed,
    InsufficientSpace,
    PartitionNotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationError {
    InvalidSignature,
    InvalidPublicKey,
    HashMismatch,
    MissingSignature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    InvalidUrl,
    InvalidVersion,
    MissingField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestError {
    InvalidFormat,
    VersionMismatch,
    UnsupportedVersion,
    InvalidChecksum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtaError {
    UpdateInProgress,
    NoUpdateAvailable,
    RollbackFailed,
    InvalidState,
}

// Implement fmt::Display for better error messages
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(e) => write!(f, "Network error: {:?}", e),
            Error::Storage(e) => write!(f, "Storage error: {:?}", e),
            Error::Verification(e) => write!(f, "Verification error: {:?}", e),
            Error::Config(e) => write!(f, "Configuration error: {:?}", e),
            Error::Manifest(e) => write!(f, "Manifest error: {:?}", e),
            Error::Ota(e) => write!(f, "OTA error: {:?}", e),
        }
    }
}

// Conversion implementations
impl From<NetworkError> for Error {
    fn from(err: NetworkError) -> Self {
        Error::Network(err)
    }
}

impl From<StorageError> for Error {
    fn from(err: StorageError) -> Self {
        Error::Storage(err)
    }
}

impl From<VerificationError> for Error {
    fn from(err: VerificationError) -> Self {
        Error::Verification(err)
    }
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        Error::Config(err)
    }
}

impl From<ManifestError> for Error {
    fn from(err: ManifestError) -> Self {
        Error::Manifest(err)
    }
}

impl From<OtaError> for Error {
    fn from(err: OtaError) -> Self {
        Error::Ota(err)
    }
}