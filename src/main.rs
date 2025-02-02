mod discovery;
mod airplay;
mod airdrop;

use eframe::egui;
use anyhow::Result;
use discovery::{DeviceDiscovery, DiscoveredDevice, ServiceType};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use airplay::{AirPlay, AirPlayStatus};
use airdrop::{AirDrop, AirDropStatus};


struct AirWinApp {
	discovery: Arc<DeviceDiscovery>,
	discovered_devices: Arc<Mutex<Vec<DiscoveredDevice>>>,
	airdrop: Arc<Mutex<AirDrop>>,
	airplay: Arc<AirPlay>,
	is_receiving_screen: Arc<Mutex<bool>>,
	current_frame_info: Arc<Mutex<Option<(u32, u32, u64)>>>,
	cached_status: Arc<Mutex<AirPlayStatus>>,
	is_scanning: Arc<Mutex<bool>>,
}

impl Default for AirWinApp {
	fn default() -> Self {
		let discovery = Arc::new(DeviceDiscovery::new().expect("Failed to create device discovery"));
		let discovered_devices = Arc::new(Mutex::new(Vec::new()));
		
		// Initialize AirDrop and AirPlay
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

		// Start AirDrop server on all interfaces
		tokio::spawn({
			let airdrop = airdrop.clone();
			async move {
				if let Err(e) = airdrop.lock().await.start_server().await {
					tracing::error!("AirDrop server error: {}. Make sure you're running as administrator and firewall allows connections", e);
				}
			}
		});

		// Start AirPlay server on all interfaces
		tokio::spawn({
			let airplay = airplay.clone();
			async move {
				if let Err(e) = airplay.start_server().await {
					tracing::error!("AirPlay server error: {}. Make sure you're running as administrator and firewall allows connections", e);
				}
			}
		});

		// Log network interfaces for debugging
		tokio::spawn(async {
			if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
				for (name, ip) in interfaces {
					tracing::info!("Network interface: {} - IP: {}", name, ip);
				}
			}
		});

		let is_receiving_screen = Arc::new(Mutex::new(false));
		let cached_status = Arc::new(Mutex::new(AirPlayStatus::Idle));
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
			current_frame_info: Arc::new(Mutex::new(None)),
			cached_status,
			is_scanning: Arc::new(Mutex::new(false)),
		}
	}
}

fn get_status_text(status: &AirDropStatus) -> (String, bool) {
	match status {
		AirDropStatus::Idle => ("📱 Share Files".to_string(), true),
		AirDropStatus::Connecting => ("⌛ Connecting...".to_string(), false),
		AirDropStatus::Connected => ("📤 Send File".to_string(), true),
		AirDropStatus::Failed(_) => ("🔄 Retry".to_string(), true),
		AirDropStatus::Transferring(progress) => (
			format!("Sending... {:.0}%", progress),
			false
		),
	}
}

impl eframe::App for AirWinApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// Set custom style once
		static STYLE_SET: std::sync::Once = std::sync::Once::new();
		STYLE_SET.call_once(|| {
			let mut style = (*ctx.style()).clone();
			style.spacing.item_spacing = egui::vec2(10.0, 10.0);
			style.visuals.window_rounding = 5.0.into();
			style.visuals.widgets.noninteractive.rounding = 5.0.into();
			style.visuals.widgets.inactive.rounding = 5.0.into();
			style.visuals.widgets.active.rounding = 5.0.into();
			style.visuals.widgets.hovered.rounding = 5.0.into();
			ctx.set_style(style);
		});

		// Update device list and frame info less frequently
		if ctx.input(|i| i.time % 1.0 < 0.1) {
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

		// Request repaints at a lower rate when not scanning
		if let Ok(scanning) = self.is_scanning.try_lock() {
			let repaint_after = if *scanning {
				std::time::Duration::from_millis(100)
			} else {
				std::time::Duration::from_millis(500)
			};
			ctx.request_repaint_after(repaint_after);
		}



		egui::CentralPanel::default().show(ctx, |ui| {
			ui.vertical_centered(|ui| {
				ui.add_space(10.0);
				ui.heading(egui::RichText::new("AirWin").size(24.0));
				ui.add_space(5.0);
			});
			
			ui.horizontal(|ui| {
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					let scan_button = egui::Button::new(egui::RichText::new("🔄 Scan for Devices").size(16.0))
						.min_size(egui::vec2(150.0, 30.0));
					if ui.add(scan_button).clicked() {
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
				});
			});

			ui.add_space(10.0);
			ui.separator();
			ui.add_space(10.0);

			ui.heading(egui::RichText::new("Nearby Devices").size(20.0));
			if let Ok(devices) = self.discovered_devices.try_lock() {
				if devices.is_empty() {
					ui.vertical_centered(|ui| {
						ui.add_space(20.0);
						if let Ok(scanning) = self.is_scanning.try_lock() {
							if *scanning {
								ui.label("Scanning for devices...");
							} else {
								ui.label("No devices found. Click 'Scan for Devices' to search.");
							}
						}
					});
				} else {
					egui::ScrollArea::vertical()
						.max_height(ui.available_height() - 100.0) // Dynamic height based on available space
						.show(ui, |ui| {
							for (idx, device) in devices.iter().enumerate() {
								ui.add_space(5.0);
								egui::Frame::none()
									.fill(ui.style().visuals.extreme_bg_color)
									.rounding(10.0)
									.shadow(egui::epaint::Shadow::small_light())
									.show(ui, |ui| {
										ui.horizontal(|ui| {
											ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true), |ui| {
												ui.add_space(10.0);
												let icon = match device.service_type {
													ServiceType::AirDrop => "📱",
													ServiceType::AirPlay => "🎥",
													ServiceType::Companion => "🔄",
													_ => "📍",
												};
												ui.label(icon);
												ui.add_space(10.0);

												ui.vertical(|ui| {
													ui.label(egui::RichText::new(&device.name).strong());
													ui.label(format!("Type: {:?}", device.service_type));
													ui.small(format!("IP: {}", device.address));
												});
											});

											ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
												match device.service_type {
													ServiceType::AirDrop => {
                                                        let current_status = {
                                                            let airdrop = self.airdrop.clone();
                                                            tokio::runtime::Handle::current().block_on(async {
                                                                if let Ok(airdrop) = airdrop.try_lock() {
                                                                    airdrop.get_status().await
                                                                } else {
                                                                    AirDropStatus::Idle
                                                                }
                                                            })
                                                        };

                                                        ui.vertical(|ui| {
                                                            // Status indicator at the top
                                                            match &current_status {
                                                                AirDropStatus::Idle => {
                                                                    ui.label("Ready to share files");
                                                                },
                                                                AirDropStatus::Connecting => {
                                                                    ui.label("Connecting to device...");
                                                                },
                                                                AirDropStatus::Connected => {
                                                                    ui.label("Connected - Ready to send");
                                                                },
                                                                AirDropStatus::Transferring(progress) => {
                                                                    ui.label(format!("Transfer progress: {:.0}%", progress));
                                                                    ui.add(egui::ProgressBar::new(*progress / 100.0));
                                                                },
                                                                AirDropStatus::Failed(err_msg) => {
                                                                    ui.colored_label(egui::Color32::RED, err_msg);
                                                                    if ui.button("Clear Error").clicked() {
                                                                        let airdrop = self.airdrop.clone();
                                                                        tokio::spawn(async move {
                                                                            if let Ok(airdrop) = airdrop.try_lock() {
                                                                                *airdrop.status.lock().await = AirDropStatus::Idle;
                                                                            }
                                                                        });
                                                                    }
                                                                }
                                                            }

                                                            ui.add_space(5.0);

                                                            // Button below status
                                                            let (button_text, button_enabled) = get_status_text(&current_status);
                                                            let send_button = egui::Button::new(button_text)
                                                                .min_size(egui::vec2(120.0, 30.0));

                                                            if ui.add_enabled(button_enabled, send_button).clicked() {
                                                                let airdrop = self.airdrop.clone();
                                                                let ctx = ctx.clone();
                                                                
                                                                tokio::spawn(async move {
                                                                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                                                                        if let Ok(airdrop) = airdrop.try_lock() {
                                                                            if let Err(e) = airdrop.send_file(path).await {
                                                                                tracing::error!("Failed to send file: {}", e);
                                                                            }
                                                                        }
                                                                    }
                                                                    ctx.request_repaint();
                                                                });
                                                            }
                                                        });


													},
													ServiceType::AirPlay | ServiceType::Companion => {
														let current_status = self.cached_status.try_lock()
															.map(|guard| guard.clone())
															.unwrap_or(AirPlayStatus::Idle);
														
														ui.vertical(|ui| {
															// Status indicator
															match &current_status {
																AirPlayStatus::Idle => {
																	ui.label("Ready to receive AirPlay stream");
																},
																AirPlayStatus::Connecting => {
																	ui.label("Waiting for connection...");
																	ui.spinner();
																},
																AirPlayStatus::Connected => {
																	ui.label("Receiving stream");
																	if let Ok(Some(info)) = self.current_frame_info.try_lock().map(|g| *g) {
																		let (width, height, _) = info;
																		ui.label(format!("Resolution: {}x{}", width, height));
																	}
																},
																AirPlayStatus::Failed(err_msg) => {
																	ui.colored_label(egui::Color32::RED, err_msg);
																	if ui.button("Clear Error").clicked() {
																		if let Ok(mut status) = self.cached_status.try_lock() {
																			*status = AirPlayStatus::Idle;
																		}
																	}
																}
															}

															ui.add_space(5.0);

															// Control buttons
															let (button_text, button_enabled) = match &current_status {
																AirPlayStatus::Idle => ("▶ Start Receiving", true),
																AirPlayStatus::Connecting => ("⌛ Connecting...", false),
																AirPlayStatus::Connected => ("⏹ Stop", true),
																AirPlayStatus::Failed(_) => ("▶ Retry", true),
															};

															let stream_button = egui::Button::new(button_text)
																.min_size(egui::vec2(120.0, 30.0));

															if ui.add_enabled(button_enabled, stream_button).clicked() {
																let airplay = self.airplay.clone();
																let is_receiving_screen = self.is_receiving_screen.clone();
																let cached_status = self.cached_status.clone();
																let ctx = ctx.clone();
																
																tokio::spawn(async move {
																	if let Ok(mut receiving) = is_receiving_screen.try_lock() {
																		if *receiving {
																			if let Err(e) = airplay.stop_receiving().await {
																				tracing::error!("Failed to stop receiving: {}", e);
																				if let Ok(mut status) = cached_status.try_lock() {
																					*status = AirPlayStatus::Failed(format!("Failed to stop: {}", e));
																				}
																			}
																			*receiving = false;
																		} else {
																			match airplay.start_receiving().await {
																				Ok(_) => *receiving = true,
																				Err(e) => {
																					tracing::error!("Failed to start receiving: {}", e);
																					if let Ok(mut status) = cached_status.try_lock() {
																						*status = AirPlayStatus::Failed(format!("Failed to start: {}", e));
																					}
																				}
																			}
																		}
																		ctx.request_repaint();
																	}
																});
															}
														});
													},

													_ => {}
												}
											});
										});

										if !device.txt_records.is_empty() {
											ui.add_space(5.0);
											ui.collapsing(format!("i Device Info ##{}", idx), |ui| {
												egui::Grid::new(format!("device_info_grid_{}", idx))
													.striped(true)
													.spacing([10.0, 5.0])
													.show(ui, |ui| {
														for (key, value) in &device.txt_records {
															ui.label(egui::RichText::new(key).strong());
															ui.label(value);
															ui.end_row();
														}
													});
											});
										}
										ui.add_space(5.0);
									});
								ui.add_space(4.0);
							}
						});
				}
			}
		});
	}
}


#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing with minimal output
	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::INFO)
		.with_target(false)
		.with_thread_ids(false)
		.with_file(false)
		.with_line_number(false)
		.init();

	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(800.0, 600.0)),
		min_window_size: Some(egui::vec2(600.0, 400.0)),
		default_theme: eframe::Theme::Dark,
		..Default::default()
	};

	eframe::run_native(
		"AirWin",
		options,
		Box::new(|_cc| Box::new(AirWinApp::default())),
	).map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))?;

	Ok(())
}