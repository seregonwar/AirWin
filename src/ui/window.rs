use eframe::egui::{self, RichText};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::network::{DeviceDiscovery, DiscoveredDevice, ServiceType};
use crate::protocols::airplay::{AirPlay, AirPlayStatus};
use crate::protocols::airdrop::{AirDrop, AirDropStatus};
use super::components::{self, DeviceCard, DeviceStatus, StyleConfig};
use tokio::time::Duration;

pub struct MainWindow {
    discovery: Arc<DeviceDiscovery>,
    discovered_devices: Arc<Mutex<Vec<DiscoveredDevice>>>,
    airdrop: Arc<Mutex<AirDrop>>,
    airplay: Arc<AirPlay>,
    is_receiving_screen: Arc<Mutex<bool>>,
    current_frame_info: Arc<Mutex<Option<(u32, u32, u64)>>>,
    cached_status: Arc<Mutex<AirPlayStatus>>,
    is_scanning: Arc<Mutex<bool>>,
    style: StyleConfig,
}

impl MainWindow {
    pub fn new(
        discovery: Arc<DeviceDiscovery>,
        airdrop: Arc<Mutex<AirDrop>>,
        airplay: Arc<AirPlay>,
    ) -> Self {
        let discovered_devices = Arc::new(Mutex::new(Vec::new()));
        let is_receiving_screen = Arc::new(Mutex::new(false));
        let current_frame_info = Arc::new(Mutex::new(None));
        let cached_status = Arc::new(Mutex::new(AirPlayStatus::Idle));
        let is_scanning = Arc::new(Mutex::new(false));
        
        // Start status update task
        let status_update = cached_status.clone();
        let airplay_clone = airplay.clone();
        tokio::spawn(async move {
            loop {
                let status = airplay_clone.get_status().await;
                if let Ok(mut current) = status_update.try_lock() {
                    if *current != status {
                        *current = status;
                    }
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        Self {
            discovery,
            discovered_devices,
            airdrop,
            airplay,
            is_receiving_screen,
            current_frame_info,
            cached_status,
            is_scanning,
            style: StyleConfig::default(),
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        // Set custom style
        components::setup_custom_style(ctx);

        // Update device list and frame info
        if ctx.input(|i| i.time % 1.0 < 0.1) {
            self.update_devices(ctx);
            self.update_frame_info(ctx);
        }

        // Adjust repaint rate based on scanning status
        if let Ok(scanning) = self.is_scanning.try_lock() {
            let repaint_after = if *scanning {
                Duration::from_millis(100)
            } else {
                Duration::from_millis(500)
            };
            ctx.request_repaint_after(repaint_after);
        }

        // Draw main UI
        self.draw_main_panel(ctx);
    }

    fn update_devices(&self, ctx: &egui::Context) {
        let discovery = self.discovery.clone();
        let devices = self.discovered_devices.clone();
        let ctx_clone = ctx.clone();
        
        tokio::spawn(async move {
            if let Ok(new_devices) = discovery.get_devices().await {
                let mut devices = devices.lock().await;
                *devices = new_devices;
                ctx_clone.request_repaint();
            }
        });
    }

    fn update_frame_info(&self, ctx: &egui::Context) {
        let airplay = self.airplay.clone();
        let frame_info = self.current_frame_info.clone();
        let is_receiving = self.is_receiving_screen.clone();
        let ctx_clone = ctx.clone();
        
        tokio::spawn(async move {
            if let Ok(receiving) = is_receiving.try_lock() {
                if *receiving {
                    if let Some(info) = airplay.get_frame_info().await {
                        *frame_info.lock().await = Some(info);
                        ctx_clone.request_repaint();
                    }
                }
            }
        });
    }

    fn draw_main_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(RichText::new("AirWin")
                    .size(32.0)
                    .color(self.style.primary_color));
                ui.label(RichText::new("AirDrop and AirPlay for Windows")
                    .size(16.0)
                    .color(self.style.text_color));
                ui.add_space(10.0);
            });

            // Scan button
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let scan_button = egui::Button::new(
                        RichText::new("🔄 Scan for Devices")
                            .size(16.0)
                            .color(self.style.card_bg_color)
                    ).min_size(egui::vec2(150.0, 35.0));

                    if ui.add(scan_button).clicked() {
                        self.handle_scan_click(ctx);
                    }
                });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Devices section
            ui.heading(RichText::new("Nearby Devices")
                .size(24.0)
                .color(self.style.text_color));
            ui.add_space(10.0);

            if let Ok(devices) = self.discovered_devices.try_lock() {
                if devices.is_empty() {
                    self.draw_empty_state(ui);
                } else {
                    self.draw_device_list(ui, &devices);
                }
            }
        });
    }

    fn draw_empty_state(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            if let Ok(scanning) = self.is_scanning.try_lock() {
                if *scanning {
                    ui.label(RichText::new("Scanning for devices...")
                        .size(16.0)
                        .color(self.style.text_color));
                    ui.spinner();
                } else {
                    ui.label(RichText::new("No devices found")
                        .size(16.0)
                        .color(self.style.text_color));
                    ui.label(RichText::new("Click 'Scan for Devices' to search")
                        .size(14.0)
                        .color(self.style.text_color.linear_multiply(0.7)));
                }
            }
            ui.add_space(40.0);
        });
    }

    fn draw_device_list(&self, ui: &mut egui::Ui, devices: &[DiscoveredDevice]) {
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 100.0)
            .show(ui, |ui| {
                for device in devices {
                    ui.add_space(5.0);
                    let status = self.get_device_status(device);
                    let card = DeviceCard {
                        device: device.clone(),
                        status,
                    };
                    components::device_card_ui(ui, &card, &self.style);
                    ui.add_space(5.0);
                }
            });
    }

    fn get_device_status(&self, device: &DiscoveredDevice) -> DeviceStatus {
        match device.service_type {
            crate::network::discovery::ServiceType::AirDrop => {
                if let Ok(airdrop) = self.airdrop.try_lock() {
                    DeviceStatus::AirDrop(
                        tokio::runtime::Handle::current()
                            .block_on(async { airdrop.get_status().await })
                    )
                } else {
                    DeviceStatus::AirDrop(AirDropStatus::Idle)
                }
            },
            crate::network::discovery::ServiceType::AirPlay |
            crate::network::discovery::ServiceType::Companion => {
                if let Ok(status) = self.cached_status.try_lock() {
                    DeviceStatus::AirPlay(status.clone())
                } else {
                    DeviceStatus::AirPlay(AirPlayStatus::Idle)
                }
            },
            _ => DeviceStatus::None,
        }
    }

    fn handle_scan_click(&self, ctx: &egui::Context) {
        let discovery = self.discovery.clone();
        let devices = self.discovered_devices.clone();
        let is_scanning = self.is_scanning.clone();
        let ctx_clone = ctx.clone();
        
        if let Ok(mut scanning) = is_scanning.try_lock() {
            *scanning = true;
        }
        ctx_clone.request_repaint();
        
        tokio::spawn(async move {
            if let Ok(new_devices) = discovery.get_devices().await {
                if let Ok(mut devices) = devices.try_lock() {
                    *devices = new_devices;
                }
            }
            if let Ok(mut scanning) = is_scanning.try_lock() {
                *scanning = false;
            }
            ctx_clone.request_repaint();
        });
    }
}
