//! Main OTA client implementation

use crate::config::{OtaConfig};
use crate::error::{Error, NetworkError, OtaError, Result};
use crate::manifest::{UpdateManifest, UpdateFile};
use crate::storage::{UpdateProgress, UpdateOperation, UpdateStorage};
use crate::verification::{PublicKey, SignatureVerifier};

use embassy_net::tcp::TcpSocket;
use embedded_tls::{Aes128GcmSha256, TlsConfig};
use heapless::{String, Vec};

/// Maximum response buffer size
const MAX_RESPONSE_SIZE: usize = 4096;

/// OTA client for managing updates
pub struct OtaClient<S> {
    config: OtaConfig,
    storage: S,
    verifier: SignatureVerifier,
    progress: Option<UpdateProgress>,
}

/// Update check result
#[derive(Debug)]
pub enum UpdateStatus {
    /// No update available
    UpToDate,
    /// Update available with manifest
    Available(UpdateManifest),
    /// Update check failed
    CheckFailed(Error),
}

impl<S> OtaClient<S>
where
    S: UpdateStorage,
{
    /// Create a new OTA client
    pub fn new(config: OtaConfig, storage: S, public_key: PublicKey) -> Self {
        Self {
            config,
            storage,
            verifier: SignatureVerifier::new(public_key),
            progress: None,
        }
    }
    
    /// Check for available updates
    pub async fn check_update<'a>(
        &mut self,
        socket: &'a mut TcpSocket<'a>,
        tls_rx_buffer: &'a mut [u8],
        tls_tx_buffer: &'a mut [u8],
    ) -> UpdateStatus {
        match self.fetch_manifest(socket, tls_rx_buffer, tls_tx_buffer).await {
            Ok(manifest) => {
                if manifest.is_applicable(&self.config.current_version) {
                    UpdateStatus::Available(manifest)
                } else {
                    UpdateStatus::UpToDate
                }
            }
            Err(e) => UpdateStatus::CheckFailed(e),
        }
    }
    
    /// Download and apply an update
    pub async fn download_and_apply<'a>(
        &mut self,
        manifest: UpdateManifest,
        socket: &'a mut TcpSocket<'a>,
        tls_rx_buffer: &'a mut [u8],
        tls_tx_buffer: &'a mut [u8],
    ) -> Result<()> {
        // Initialize progress tracking
        self.progress = Some(UpdateProgress::new(manifest.total_size()));
        
        // Find firmware file
        let firmware_file = manifest
            .firmware_file()
            .ok_or(Error::Ota(OtaError::InvalidState))?;
        
        // Download firmware
        self.update_progress(0, UpdateOperation::Downloading);
        let firmware_data = self
            .download_file(firmware_file, socket, tls_rx_buffer, tls_tx_buffer)
            .await?;
        
        // Verify integrity
        self.update_progress(firmware_file.size, UpdateOperation::Verifying);
        self.verifier
            .verify_firmware(&firmware_data, &firmware_file.sha256)?;
        
        // Write to storage
        self.update_progress(firmware_file.size, UpdateOperation::Writing);
        self.write_firmware(&firmware_data).await?;
        
        // Finalize update
        self.update_progress(firmware_file.size, UpdateOperation::Finalizing);
        self.finalize_update(&manifest).await?;
        
        self.update_progress(firmware_file.size, UpdateOperation::Complete);
        Ok(())
    }
    
    /// Get current update progress
    pub fn progress(&self) -> Option<&UpdateProgress> {
        self.progress.as_ref()
    }
    
    /// Fetch update manifest from server
    async fn fetch_manifest<'a>(
        &self,
        socket: &'a mut TcpSocket<'a>,
        tls_rx_buffer: &'a mut [u8],
        tls_tx_buffer: &'a mut [u8],
    ) -> Result<UpdateManifest> {
        let manifest_url = self.build_manifest_url()?;
        let response = self
            .http_get(&manifest_url, socket, tls_rx_buffer, tls_tx_buffer)
            .await?;
        
        // Parse and verify manifest
        let manifest = crate::manifest::Manifest::parse(&response)?;
        
        // Verify signature
        let manifest_bytes = postcard::to_vec::<_, 512>(&manifest)
            .map_err(|_| Error::Manifest(crate::error::ManifestError::InvalidFormat))?;
        self.verifier
            .verify_manifest(&manifest_bytes, &manifest.signature)?;
        
        Ok(manifest)
    }
    
    /// Download a file from the update
    async fn download_file<'a>(
        &self,
        file: &UpdateFile,
        socket: &'a mut TcpSocket<'a>,
        tls_rx_buffer: &'a mut [u8],
        tls_tx_buffer: &'a mut [u8],
    ) -> Result<Vec<u8, 65536>> {
        let file_url = self.build_file_url(&file.url)?;
        
        // For larger files, we'd want to stream directly to storage
        // For now, we'll buffer in RAM (limited to 64KB)
        let response = self
            .http_get(&file_url, socket, tls_rx_buffer, tls_tx_buffer)
            .await?;
        
        Vec::from_slice(&response)
            .map_err(|_| Error::Storage(crate::error::StorageError::InsufficientSpace))
    }
    
    /// Perform HTTP GET request
    async fn http_get<'a>(
        &self,
        url: &str,
        socket: &'a mut TcpSocket<'a>,
        tls_rx_buffer: &'a mut [u8],
        tls_tx_buffer: &'a mut [u8],
    ) -> Result<Vec<u8, MAX_RESPONSE_SIZE>> {
        // This is a simplified HTTP client implementation
        // In production, you'd want proper error handling and retries
        
        let mut rx_buffer = [0; MAX_RESPONSE_SIZE];
        let mut response: Vec<u8, MAX_RESPONSE_SIZE> = Vec::new();
        
        // Parse URL components (simplified)
        let (host, path) = self.parse_url(url)?;
        
        // Create TLS client
        let tls_config = TlsConfig::<Aes128GcmSha256>::new()
            .with_server_name(&host);
        
        // In a real implementation, you'd:
        // 1. Resolve DNS for the host
        // 2. Connect TCP socket
        // 3. Perform TLS handshake
        // 4. Send HTTP request
        // 5. Parse response
        
        // Placeholder for actual implementation
        Err(NetworkError::ConnectionFailed.into())
    }
    
    /// Write firmware to storage
    async fn write_firmware(&mut self, data: &[u8]) -> Result<()> {
        // Erase target partition
        let erase_size = self.storage.erase_size();
        let blocks_needed = (data.len() as u32 + erase_size - 1) / erase_size;
        self.storage.erase(0, blocks_needed * erase_size).await?;
        
        // Write firmware in chunks
        const CHUNK_SIZE: usize = 4096;
        for (offset, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
            self.storage
                .write((offset * CHUNK_SIZE) as u32, chunk)
                .await?;
        }
        
        Ok(())
    }
    
    /// Finalize the update process
    async fn finalize_update(&mut self, manifest: &UpdateManifest) -> Result<()> {
        // Update configuration with new version
        self.config.current_version = manifest.version;
        
        // In a real implementation, you would:
        // 1. Update boot flags to switch partitions
        // 2. Save rollback information
        // 3. Schedule a reboot
        
        Ok(())
    }
    
    /// Build manifest URL
    fn build_manifest_url(&self) -> Result<String<256>> {
        let mut url = String::new();
        url.push_str(&self.config.server_url)
            .map_err(|_| Error::Config(crate::error::ConfigError::InvalidUrl))?;
        url.push_str("/manifest.json")
            .map_err(|_| Error::Config(crate::error::ConfigError::InvalidUrl))?;
        url.push_str("?device_id=")
            .map_err(|_| Error::Config(crate::error::ConfigError::InvalidUrl))?;
        url.push_str(&self.config.device_id)
            .map_err(|_| Error::Config(crate::error::ConfigError::InvalidUrl))?;
        Ok(url)
    }
    
    /// Build file download URL
    fn build_file_url(&self, file_path: &str) -> Result<String<256>> {
        let mut url = String::new();
        url.push_str(&self.config.server_url)
            .map_err(|_| Error::Config(crate::error::ConfigError::InvalidUrl))?;
        url.push_str("/")
            .map_err(|_| Error::Config(crate::error::ConfigError::InvalidUrl))?;
        url.push_str(file_path)
            .map_err(|_| Error::Config(crate::error::ConfigError::InvalidUrl))?;
        Ok(url)
    }
    
    /// Parse URL into host and path
    fn parse_url(&self, url: &str) -> Result<(String<64>, String<128>)> {
        // Simplified URL parsing
        // In production, use a proper URL parser
        let url = url.trim_start_matches("https://");
        let parts: Vec<&str, 2> = url.splitn(2, '/').collect();
        
        if parts.is_empty() {
            return Err(Error::Config(crate::error::ConfigError::InvalidUrl));
        }
        
        let host = String::try_from(parts[0])
            .map_err(|_| Error::Config(crate::error::ConfigError::InvalidUrl))?;
        let path = if parts.len() > 1 {
            {
                let mut path = String::try_from("/").unwrap();
                path.push_str(parts[1]).unwrap();
                path
            }
        } else {
            String::try_from("/").unwrap()
        };
        
        Ok((host, path))
    }
    
    /// Update progress tracking
    fn update_progress(&mut self, bytes: u32, operation: UpdateOperation) {
        if let Some(progress) = &mut self.progress {
            progress.update(bytes, operation);
        }
    }
}