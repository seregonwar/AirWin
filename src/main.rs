// Module declarations
mod network;
mod protocols;
mod ui;
mod utils;

use eframe::egui;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use local_ip_address;
use tracing;
use std::time::Duration;

// Import network types
use network::DeviceDiscovery;
use network::DiscoveredDevice;
use network::ServiceType;

// Import protocol types
use protocols::airplay::AirPlay;
use protocols::airplay::AirPlayStatus;
use protocols::airdrop::AirDrop;
use protocols::airdrop::AirDropStatus;

// Import UI
use ui::MainWindow;

struct AirWinApp {
    window: MainWindow,
}

impl Default for AirWinApp {
    fn default() -> Self {
        // Initialize services
        let discovery = Arc::new(DeviceDiscovery::new().expect("Failed to create device discovery"));
        let airdrop = Arc::new(Mutex::new(AirDrop::new()));
        let airplay = Arc::new(AirPlay::new());
        
        // Start discovery service
        tokio::spawn({
            let discovery = discovery.clone();
            async move {
                if let Err(e) = discovery.start_discovery().await {
                    tracing::error!("Discovery error: {}", e);
                }
            }
        });

        // Start AirDrop server
        tokio::spawn({
            let airdrop = airdrop.clone();
            async move {
                if let Err(e) = airdrop.lock().await.start_server().await {
                    tracing::error!("AirDrop server error: {}. Make sure you're running as administrator and firewall allows connections", e);
                }
            }
        });

        // Start AirPlay server
        tokio::spawn({
            let airplay = airplay.clone();
            async move {
                if let Err(e) = airplay.start_server().await {
                    tracing::error!("AirPlay server error: {}. Make sure you're running as administrator and firewall allows connections", e);
                }
            }
        });

        // Log network interfaces
        tokio::spawn(async {
            if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
                for (name, ip) in interfaces {
                    tracing::info!("Network interface: {} - IP: {}", name, ip);
                }
            }
        });

        Self {
            window: MainWindow::new(discovery, airdrop, airplay),
        }
    }
}

impl eframe::App for AirWinApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.window.update(ctx);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    utils::setup_logging();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        min_window_size: Some(egui::vec2(600.0, 400.0)),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "AirWin",
        options,
        Box::new(|_cc| Box::new(AirWinApp::default())),
    ).map_err(|e| anyhow::anyhow!("Failed to start application: {}", e))?;

    Ok(())
}