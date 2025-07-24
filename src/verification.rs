//! GPG signature verification for secure OTA updates

use crate::error::{Result, VerificationError};
use crate::manifest::{Signature, SignatureAlgorithm};
use ed25519_dalek::{Signature as Ed25519Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

/// Public key for signature verification
pub struct PublicKey {
    algorithm: SignatureAlgorithm,
    key_data: KeyData,
}

enum KeyData {
    Ed25519(VerifyingKey),
    // Add RSA support later if needed
}

/// Signature verifier for OTA updates
pub struct SignatureVerifier {
    public_key: PublicKey,
}

impl SignatureVerifier {
    /// Create a new signature verifier with the given public key
    pub fn new(public_key: PublicKey) -> Self {
        Self { public_key }
    }
    
    /// Verify a manifest signature
    pub fn verify_manifest(&self, manifest_data: &[u8], signature: &Signature) -> Result<()> {
        // Check algorithm matches
        if signature.algorithm != self.public_key.algorithm {
            return Err(VerificationError::InvalidSignature.into());
        }
        
        match &self.public_key.key_data {
            KeyData::Ed25519(key) => {
                self.verify_ed25519(manifest_data, &signature.data, key)
            }
        }
    }
    
    /// Verify firmware integrity using SHA256
    pub fn verify_firmware(&self, firmware_data: &[u8], expected_hash: &[u8; 32]) -> Result<()> {
        let mut hasher = Sha256::new();
        hasher.update(firmware_data);
        let computed_hash = hasher.finalize();
        
        if computed_hash.as_slice() != expected_hash {
            return Err(VerificationError::HashMismatch.into());
        }
        
        Ok(())
    }
    
    /// Verify Ed25519 signature
    fn verify_ed25519(
        &self,
        data: &[u8],
        signature_bytes: &[u8],
        key: &VerifyingKey,
    ) -> Result<()> {
        // Parse signature
        let signature = Ed25519Signature::from_slice(signature_bytes)
            .map_err(|_| VerificationError::InvalidSignature)?;
        
        // Verify
        key.verify(data, &signature)
            .map_err(|_| VerificationError::InvalidSignature)?;
        
        Ok(())
    }
}

impl PublicKey {
    /// Create an Ed25519 public key from bytes
    pub fn ed25519_from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(bytes)
            .map_err(|_| VerificationError::InvalidPublicKey)?;
        
        Ok(Self {
            algorithm: SignatureAlgorithm::Ed25519,
            key_data: KeyData::Ed25519(key),
        })
    }
    
    /// Get the algorithm used by this key
    pub fn algorithm(&self) -> SignatureAlgorithm {
        self.algorithm
    }
}

/// Embedded public key for firmware verification
/// This should be replaced with your actual public key
pub const EMBEDDED_PUBLIC_KEY: &[u8; 32] = &[
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];

/// Helper to get the default embedded public key
pub fn default_public_key() -> Result<PublicKey> {
    PublicKey::ed25519_from_bytes(EMBEDDED_PUBLIC_KEY)
}