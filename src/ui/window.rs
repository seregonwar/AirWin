use eframe::egui::{self, RichText, Color32};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::network::{DeviceDiscovery, DiscoveredDevice, ServiceType};
use crate::protocols::airplay::{AirPlay, AirPlayStatus};
use crate::protocols::airdrop::{AirDrop, AirDropStatus};
use crate::protocols::awdl::AwdlManager;
use super::components::{self, DeviceCard, DeviceStatus, StyleConfig};
use tokio::time::Duration;
use std::net::SocketAddr;
use rfd::FileDialog;
use std::path::PathBuf;
use tracing::{error, warn};

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
enum SendOption {
    None,
    File,
    Link,
}

pub struct MainWindow {
    discovery: Arc<DeviceDiscovery>,
    discovered_devices: Arc<Mutex<Vec<DiscoveredDevice>>>,
    airdrop: Arc<Mutex<AirDrop>>,
    airplay: Arc<AirPlay>,
    awdl_manager: Arc<Mutex<AwdlManager>>,
    is_receiving_screen: Arc<Mutex<bool>>,
    current_frame_info: Arc<Mutex<Option<(u32, u32, u64)>>>,
    cached_status: Arc<Mutex<AirPlayStatus>>,
    is_scanning: Arc<Mutex<bool>>,
    style: StyleConfig,
    url_to_send: String,
    show_link_dialog: bool,
    selected_device: Option<DiscoveredDevice>,
}

impl MainWindow {
    pub fn new(
        discovery: Arc<DeviceDiscovery>,
        airdrop: Arc<Mutex<AirDrop>>,
        airplay: Arc<AirPlay>,
        awdl_manager: Arc<Mutex<AwdlManager>>,
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
            awdl_manager,
            is_receiving_screen,
            current_frame_info,
            cached_status,
            is_scanning,
            style: StyleConfig::default(),
            url_to_send: String::new(),
            show_link_dialog: false,
            selected_device: None,
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

    fn draw_main_panel(&mut self, ctx: &egui::Context) {
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
                            .color(Color32::WHITE)
                    )
                    .min_size(egui::vec2(160.0, 36.0))
                    .fill(self.style.primary_color)
                    .rounding(6.0);

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

            // Clone devices in a separate scope to avoid holding an immutable borrow of `self`
            let devices_clone_opt = {
                let discovered = self.discovered_devices.clone();
                // Ensure the temporary from try_lock() is dropped before the block ends
                let tmp = if let Ok(devices) = discovered.try_lock() {
                    Some(devices.clone())
                } else {
                    None
                };
                tmp
            };

            if let Some(devices_clone) = devices_clone_opt {
                if devices_clone.is_empty() {
                    self.draw_empty_state(ui);
                } else {
                    self.draw_device_list(ui, &devices_clone);
                }
            }

            // Persist the action dialog when a device is selected
            if self.selected_device.is_some() {
                self.show_action_dialog(ctx);
            }
        });
    }

    fn draw_empty_state(&mut self, ui: &mut egui::Ui) {
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

    fn draw_device_list(&mut self, ui: &mut egui::Ui, devices: &[DiscoveredDevice]) {
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
                    
                    let response = components::device_card_ui(ui, &card, &self.style);
                    
                    // Handle click on device card
                    if response.clicked() {
                        match device.service_type {
                            ServiceType::AirDrop | ServiceType::Companion => {
                                // Treat Companion as an AirDrop-capable peer for send actions
                                // The action dialog is shown persistently from draw_main_panel
                                self.selected_device = Some(device.clone());
                            },
                            ServiceType::AirPlay => {
                                // Toggle AirPlay streaming only for AirPlay devices
                                let airplay = self.airplay.clone();
                                tokio::spawn(async move {
                                    match airplay.get_status().await {
                                        AirPlayStatus::Idle => {
                                            let _ = airplay.start_receiving().await;
                                        },
                                        AirPlayStatus::Connected => {
                                            let _ = airplay.stop_receiving().await;
                                        },
                                        _ => {}
                                    }
                                });
                            },
                            _ => {}
                        }
                    }
                    
                    ui.add_space(5.0);
                }
            });
        
        // Show link dialog if needed
        if self.show_link_dialog {
            self.show_link_input_dialog(ui.ctx());
        }
    }

    fn get_device_status(&self, device: &DiscoveredDevice) -> DeviceStatus {
        match device.service_type {
            crate::network::discovery::ServiceType::AirDrop => {
                // Avoid blocking within a runtime; read cached status via try_lock
                if let Ok(airdrop) = self.airdrop.try_lock() {
                    if let Ok(status) = airdrop.status.try_lock() {
                        DeviceStatus::AirDrop(status.clone())
                    } else {
                        DeviceStatus::AirDrop(AirDropStatus::Idle)
                    }
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
    
    fn show_action_dialog(&mut self, ctx: &egui::Context) {
        // Clone selected device to avoid holding an immutable borrow of `self` across the UI closure
        let selected = self.selected_device.clone();
        if let Some(device) = selected {
            let frame = egui::Frame::none()
                .fill(self.style.card_bg_color)
                .stroke(egui::Stroke { width: 1.0, color: self.style.primary_color })
                .rounding(10.0)
                .shadow(egui::epaint::Shadow { extrusion: 10.0, color: Color32::from_black_alpha(120) });

            egui::Window::new("Choose Action")
                .collapsible(false)
                .resizable(false)
                .frame(frame)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(RichText::new(format!("Send to {}", device.name))
                            .color(self.style.text_color));
                        ui.add_space(20.0);
                        
                        ui.horizontal(|ui| {
                            let button = egui::Button::new(RichText::new("📁 Send File")
                                .size(18.0)
                                .color(Color32::WHITE))
                                .min_size(egui::vec2(130.0, 42.0))
                                .fill(self.style.accent_color)
                                .rounding(6.0);
                            if ui.add(button).clicked() {
                                self.send_file_to_device(device.clone());
                                self.selected_device = None;
                            }
                            
                            ui.add_space(10.0);
                            
                            let button = egui::Button::new(RichText::new("🔗 Send Link")
                                .size(18.0)
                                .color(Color32::WHITE))
                                .min_size(egui::vec2(130.0, 42.0))
                                .fill(self.style.primary_color)
                                .rounding(6.0);
                            if ui.add(button).clicked() {
                                self.show_link_dialog = true;
                                // Keep the selected device for the link dialog
                            }
                        });
                        
                        ui.add_space(10.0);
                        
                        let cancel_btn = egui::Button::new(RichText::new("Cancel").color(Color32::WHITE))
                            .min_size(egui::vec2(100.0, 36.0))
                            .fill(self.style.error_color)
                            .rounding(6.0);
                        if ui.add(cancel_btn).clicked() {
                            self.selected_device = None;
                        }
                    });
                });
        }
    }
    
    fn show_link_input_dialog(&mut self, ctx: &egui::Context) {
        let frame = egui::Frame::none()
            .fill(self.style.card_bg_color)
            .stroke(egui::Stroke { width: 1.0, color: self.style.primary_color })
            .rounding(10.0)
            .shadow(egui::epaint::Shadow { extrusion: 10.0, color: Color32::from_black_alpha(120) });

        egui::Window::new("Send Link")
            .collapsible(false)
            .resizable(false)
            .frame(frame)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Enter URL to send")
                        .color(self.style.text_color));
                    ui.add_space(10.0);
                    
                    let text_edit = egui::TextEdit::singleline(&mut self.url_to_send)
                        .hint_text("https://example.com/...")
                        .desired_width(300.0);
                    ui.add(text_edit);
                    
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        let send_btn = egui::Button::new(RichText::new("Send").color(Color32::WHITE))
                            .min_size(egui::vec2(100.0, 36.0))
                            .fill(self.style.accent_color)
                            .rounding(6.0);
                        if ui.add(send_btn).clicked() && !self.url_to_send.trim().is_empty() {
                            if let Some(device) = self.selected_device.clone() {
                                self.send_link_to_device(device, self.url_to_send.clone());
                            }
                            self.show_link_dialog = false;
                            self.selected_device = None;
                            self.url_to_send.clear();
                        }
                        
                        ui.add_space(10.0);
                        
                        let cancel_btn = egui::Button::new(RichText::new("Cancel").color(Color32::WHITE))
                            .min_size(egui::vec2(100.0, 36.0))
                            .fill(self.style.error_color)
                            .rounding(6.0);
                        if ui.add(cancel_btn).clicked() {
                            self.show_link_dialog = false;
                            self.selected_device = None;
                            self.url_to_send.clear();
                        }
                    });
                });
            });
    }
    
    fn send_file_to_device(&self, device: DiscoveredDevice) {
        if let Some(path) = FileDialog::new()
            .set_title(&format!("Select file to send to {}", device.name))
            .pick_file() {
            let airdrop = self.airdrop.clone();
            // Use AirDrop standard port for AirDrop/Companion services
            let port = match device.service_type {
                ServiceType::AirDrop | ServiceType::Companion => 8771,
                _ => device.port,
            };
            let addr = SocketAddr::new(device.address, port);

            // Clone AirDrop instance without holding the lock across .await
            let ad_opt = match airdrop.try_lock() {
                Ok(guard) => Some(guard.clone()),
                Err(_) => None,
            };

            tokio::spawn(async move {
                if let Some(ad) = ad_opt {
                    if let Err(e) = ad.send_file_to(addr, PathBuf::from(path)).await {
                        error!("Failed to send file to {}: {}", addr, e);
                    }
                } else {
                    warn!("AirDrop busy; could not acquire lock to send file");
                }
            });
        }
    }
    
    fn send_link_to_device(&self, device: DiscoveredDevice, url: String) {
        // Create a temporary text file with the URL
        let temp_path = std::env::temp_dir().join("airwin_link.url");
        let url_content = format!("[InternetShortcut]\nURL={}", url);
        
        if std::fs::write(&temp_path, url_content.as_bytes()).is_ok() {
            let airdrop = self.airdrop.clone();
            // Use AirDrop standard port for AirDrop/Companion services
            let port = match device.service_type {
                ServiceType::AirDrop | ServiceType::Companion => 8771,
                _ => device.port,
            };
            let addr = SocketAddr::new(device.address, port);

            // Clone AirDrop instance without holding the lock across .await
            let ad_opt = match airdrop.try_lock() {
                Ok(guard) => Some(guard.clone()),
                Err(_) => None,
            };

            tokio::spawn(async move {
                if let Some(ad) = ad_opt {
                    if let Err(e) = ad.send_file_to(addr, temp_path).await {
                        error!("Failed to send link file to {}: {}", addr, e);
                    }
                } else {
                    warn!("AirDrop busy; could not acquire lock to send link");
                }
            });
        }
    }
}
