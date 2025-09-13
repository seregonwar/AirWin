//! AWDL Protocol Integration
//!
//! This module integrates the OWDL (Open Wireless Direct Link) implementation
//! into the AirWin project, providing Apple Wireless Direct Link protocol support.

use owdl::{
    AwdlDaemon, DaemonBuilder, DaemonConfig,
    AwdlData,
    AwdlPeer,
};
use owdl::daemon::{IoConfig, ServiceConfig, DaemonStats};

use crate::utils::{AirWinError, AirWinResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use serde::{Deserialize, Serialize};

/// AWDL Manager for AirWin integration
#[derive(Debug)]
pub struct AwdlManager {
    /// AWDL daemon instance
    daemon: Option<AwdlDaemon>,
    /// Manager configuration
    config: AwdlManagerConfig,
    /// Current state
    state: Arc<RwLock<AwdlManagerState>>,
    /// Discovered peers
    peers: Arc<RwLock<Vec<AwdlPeer>>>,
}

/// AWDL Manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwdlManagerConfig {
    /// Enable AWDL protocol
    pub enabled: bool,
    /// Network interface to use (None for auto-detection)
    pub interface: Option<String>,
    /// Device name for AWDL
    pub device_name: String,
    /// Service name for discovery
    pub service_name: String,
    /// Auto-start daemon on initialization
    pub auto_start: bool,
    /// Peer discovery interval in seconds
    pub discovery_interval: u64,
    /// Maximum number of peers to maintain
    pub max_peers: usize,
}

/// AWDL Manager state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwdlManagerState {
    /// Manager is stopped
    Stopped,
    /// Manager is initializing
    Initializing,
    /// Manager is starting
    Starting,
    /// Manager is running
    Running,
    /// Manager is stopping
    Stopping,
    /// Manager encountered an error
    Error,
}

/// AWDL peer information for AirWin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwdlPeerInfo {
    /// Peer MAC address
    pub mac_address: [u8; 6],
    /// Peer device name
    pub device_name: String,
    /// Peer service name
    pub service_name: String,
    /// Last seen timestamp
    pub last_seen: chrono::DateTime<chrono::Utc>,
    /// Signal strength (if available)
    pub signal_strength: Option<i32>,
    /// Peer capabilities
    pub capabilities: Vec<String>,
}

impl Default for AwdlManagerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interface: None,
            device_name: "AirWin-Device".to_string(),
            service_name: "_airwin._tcp".to_string(),
            auto_start: true,
            discovery_interval: 30,
            max_peers: 50,
        }
    }
}

impl AwdlManager {
    /// Create new AWDL manager
    pub fn new(config: AwdlManagerConfig) -> Self {
        Self {
            daemon: None,
            config,
            state: Arc::new(RwLock::new(AwdlManagerState::Stopped)),
            peers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initialize the AWDL manager
    pub async fn initialize(&mut self) -> AirWinResult<()> {
        if !self.config.enabled {
            info!("AWDL protocol is disabled in configuration");
            return Ok(());
        }

        self.set_state(AwdlManagerState::Initializing).await;
        info!("Initializing AWDL manager");

        // Create daemon configuration
        let daemon_config = DaemonConfig::default();
        let io_config = IoConfig::default();
        let service_config = ServiceConfig::default();

        // Build daemon
        let mut builder = DaemonBuilder::new()
            .with_config(daemon_config)
            .with_io_config(io_config)
            .with_service_config(service_config);

        if let Some(ref interface) = self.config.interface {
            builder = builder.with_interface(Some(interface.clone()));
        }

        match builder.build().await {
            Ok(mut daemon) => {
                // Initialize daemon
                if let Err(e) = daemon.init().await {
                    let error_msg = e.to_string();
                    // Check for port binding errors
                    if error_msg.contains("10048") || error_msg.contains("already in use") || error_msg.contains("bind") {
                        warn!("AWDL port already in use, disabling AWDL support: {}", e);
                        self.config.enabled = false;
                        self.set_state(AwdlManagerState::Stopped).await;
                        return Ok(());
                    }
                    
                    error!("Failed to initialize AWDL daemon: {}", e);
                    self.set_state(AwdlManagerState::Error).await;
                    return Err(AirWinError::NetworkError(format!("AWDL initialization failed: {}", e)));
                }

                self.daemon = Some(daemon);
                info!("AWDL manager initialized successfully");

                if self.config.auto_start {
                    self.start().await?;
                }

                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                // Check for port binding errors
                if error_msg.contains("10048") || error_msg.contains("already in use") || error_msg.contains("bind") {
                    warn!("Cannot build AWDL daemon due to port conflict, disabling AWDL: {}", e);
                    self.config.enabled = false;
                    self.set_state(AwdlManagerState::Stopped).await;
                    return Ok(());
                }
                
                error!("Failed to build AWDL daemon: {}", e);
                self.set_state(AwdlManagerState::Error).await;
                Err(AirWinError::NetworkError(format!("AWDL daemon build failed: {}", e)))
            }
        }
    }

    /// Start the AWDL manager
    pub async fn start(&mut self) -> AirWinResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        self.set_state(AwdlManagerState::Starting).await;
        info!("Starting AWDL manager");

        if let Some(ref mut daemon) = self.daemon {
            match daemon.start().await {
                Ok(()) => {
                    self.set_state(AwdlManagerState::Running).await;
                    info!("AWDL manager started successfully");

                    // Start peer discovery task
                    self.start_peer_discovery().await;

                    Ok(())
                }
                Err(e) => {
                    // Check if error is due to port already in use
                    let error_msg = e.to_string();
                    if error_msg.contains("10048") || error_msg.contains("already in use") || error_msg.contains("bind") {
                        warn!("AWDL port already in use, continuing without AWDL support");
                        self.set_state(AwdlManagerState::Stopped).await;
                        // Don't fail completely, just disable AWDL
                        self.config.enabled = false;
                        return Ok(());
                    }
                    
                    error!("Failed to start AWDL daemon: {}", e);
                    self.set_state(AwdlManagerState::Error).await;
                    Err(AirWinError::NetworkError(format!("AWDL start failed: {}", e)))
                }
            }
        } else {
            error!("AWDL daemon not initialized");
            self.set_state(AwdlManagerState::Error).await;
            Err(AirWinError::NetworkError("AWDL daemon not initialized".to_string()))
        }
    }

    /// Stop the AWDL manager
    pub async fn stop(&mut self) -> AirWinResult<()> {
        self.set_state(AwdlManagerState::Stopping).await;
        info!("Stopping AWDL manager");

        if let Some(ref mut daemon) = self.daemon {
            match daemon.stop().await {
                Ok(()) => {
                    self.set_state(AwdlManagerState::Stopped).await;
                    info!("AWDL manager stopped successfully");
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to stop AWDL daemon: {}", e);
                    self.set_state(AwdlManagerState::Error).await;
                    Err(AirWinError::NetworkError(format!("AWDL stop failed: {}", e)))
                }
            }
        } else {
            self.set_state(AwdlManagerState::Stopped).await;
            Ok(())
        }
    }

    /// Get current state
    pub async fn get_state(&self) -> AwdlManagerState {
        *self.state.read().await
    }

    /// Get discovered peers
    pub async fn get_peers(&self) -> Vec<AwdlPeerInfo> {
        let peers = self.peers.read().await;
        peers.iter().map(|peer| self.convert_peer_info(peer)).collect()
    }

    /// Send data to a specific peer
    pub async fn send_data(&self, peer_mac: [u8; 6], data: &[u8]) -> AirWinResult<()> {
        if let Some(ref _daemon) = self.daemon {
            // Create AWDL data frame
            let _src_mac = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // TODO: Get actual MAC
            let _frame = AwdlData::new(
                peer_mac,
                _src_mac,
                0x0800, // IP protocol
                bytes::Bytes::copy_from_slice(data),
            );

            // TODO: Implement actual data sending through daemon
            debug!("Sending {} bytes to peer {:02x?}", data.len(), peer_mac);
            Ok(())
        } else {
            Err(AirWinError::NetworkError("AWDL daemon not available".to_string()))
        }
    }

    /// Broadcast data to all peers
    pub async fn broadcast_data(&self, data: &[u8]) -> AirWinResult<()> {
        let peers = self.get_peers().await;
        for peer in peers {
            if let Err(e) = self.send_data(peer.mac_address, data).await {
                warn!("Failed to send data to peer {:02x?}: {}", peer.mac_address, e);
            }
        }
        Ok(())
    }

    /// Get daemon statistics
    pub async fn get_stats(&self) -> Option<DaemonStats> {
        if let Some(ref daemon) = self.daemon {
            Some(daemon.get_stats().await)
        } else {
            None
        }
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: AwdlManagerConfig) -> AirWinResult<()> {
        let was_running = self.get_state().await == AwdlManagerState::Running;

        if was_running {
            self.stop().await?;
        }

        self.config = config;

        if was_running && self.config.enabled {
            self.initialize().await?;
        }

        Ok(())
    }

    /// Set manager state
    async fn set_state(&self, state: AwdlManagerState) {
        *self.state.write().await = state;
    }

    /// Start peer discovery task
    async fn start_peer_discovery(&self) {
        let peers: Arc<RwLock<Vec<AwdlPeer>>> = Arc::clone(&self.peers);
        let interval = self.config.discovery_interval;
        let max_peers = self.config.max_peers;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(
                tokio::time::Duration::from_secs(interval)
            );

            loop {
                interval_timer.tick().await;

                // TODO: Implement actual peer discovery
                debug!("Running peer discovery...");

                // Clean up old peers and maintain max peer limit
                let mut peers_guard = peers.write().await;
                let _now = chrono::Utc::now();
                peers_guard.retain(|_peer| {
                    // TODO: Implement peer timeout logic
                    true
                });

                if peers_guard.len() > max_peers {
                    peers_guard.truncate(max_peers);
                }
            }
        });
    }

    /// Convert OWDL peer to AirWin peer info
    fn convert_peer_info(&self, peer: &AwdlPeer) -> AwdlPeerInfo {
        AwdlPeerInfo {
            mac_address: peer.address,
            device_name: peer.name.clone().unwrap_or_else(|| "Unknown".to_string()),
            service_name: self.config.service_name.clone(),
            last_seen: chrono::Utc::now(), // TODO: Use actual timestamp
            signal_strength: None, // TODO: Get from peer if available
            capabilities: vec![], // TODO: Extract from peer
        }
    }
}

/// AWDL protocol utilities
pub struct AwdlUtils;

impl AwdlUtils {
    /// Check if AWDL is supported on this system
    pub fn is_supported() -> bool {
        // TODO: Implement platform-specific checks
        true
    }

    /// Get available network interfaces for AWDL
    pub fn get_available_interfaces() -> Vec<String> {
        // TODO: Implement interface enumeration
        vec![]
    }

    /// Validate MAC address format
    pub fn validate_mac_address(mac: &[u8; 6]) -> bool {
        // Check for valid MAC address (not all zeros, not broadcast)
        *mac != [0; 6] && *mac != [0xff; 6]
    }

    /// Format MAC address for display
    pub fn format_mac_address(mac: &[u8; 6]) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_awdl_manager_config_default() {
        let config = AwdlManagerConfig::default();
        assert!(config.enabled);
        assert!(config.auto_start);
        assert_eq!(config.device_name, "AirWin-Device");
        assert_eq!(config.service_name, "_airwin._tcp");
    }

    #[test]
    fn test_awdl_utils_mac_validation() {
        assert!(!AwdlUtils::validate_mac_address(&[0; 6]));
        assert!(!AwdlUtils::validate_mac_address(&[0xff; 6]));
        assert!(AwdlUtils::validate_mac_address(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]));
    }

    #[test]
    fn test_awdl_utils_mac_formatting() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let formatted = AwdlUtils::format_mac_address(&mac);
        assert_eq!(formatted, "00:11:22:33:44:55");
    }

    #[tokio::test]
    async fn test_awdl_manager_creation() {
        let config = AwdlManagerConfig::default();
        let manager = AwdlManager::new(config);
        assert_eq!(manager.get_state().await, AwdlManagerState::Stopped);
    }
}