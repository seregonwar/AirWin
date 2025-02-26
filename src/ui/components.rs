use eframe::egui::{self, RichText, Color32};
use crate::network::{DiscoveredDevice, ServiceType};
use crate::protocols::airplay::AirPlayStatus;
use crate::protocols::airdrop::AirDropStatus;

pub struct DeviceCard {
    pub device: DiscoveredDevice,
    pub status: DeviceStatus,
}

pub enum DeviceStatus {
    AirDrop(AirDropStatus),
    AirPlay(AirPlayStatus),
    None,
}

pub struct StyleConfig {
    pub primary_color: Color32,
    pub secondary_color: Color32,
    pub accent_color: Color32,
    pub error_color: Color32,
    pub success_color: Color32,
    pub text_color: Color32,
    pub bg_color: Color32,
    pub card_bg_color: Color32,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            primary_color: Color32::from_rgb(52, 152, 219),    // Blue
            secondary_color: Color32::from_rgb(41, 128, 185),  // Darker blue
            accent_color: Color32::from_rgb(46, 204, 113),     // Green
            error_color: Color32::from_rgb(231, 76, 60),       // Red
            success_color: Color32::from_rgb(39, 174, 96),     // Dark green
            text_color: Color32::from_rgb(52, 73, 94),         // Dark gray
            bg_color: Color32::from_rgb(236, 240, 241),        // Light gray
            card_bg_color: Color32::WHITE,
        }
    }
}

pub fn setup_custom_style(ctx: &egui::Context) {
    let style = StyleConfig::default();
    let mut visuals = ctx.style().visuals.clone();
    
    // Set modern rounded corners
    visuals.window_rounding = 8.0.into();
    visuals.widgets.noninteractive.rounding = 6.0.into();
    visuals.widgets.inactive.rounding = 6.0.into();
    visuals.widgets.active.rounding = 6.0.into();
    visuals.widgets.hovered.rounding = 6.0.into();

    // Set colors
    visuals.override_text_color = Some(style.text_color);
    visuals.widgets.noninteractive.bg_fill = style.bg_color;
    visuals.widgets.inactive.bg_fill = style.card_bg_color;
    visuals.widgets.active.bg_fill = style.primary_color;
    visuals.widgets.hovered.bg_fill = style.secondary_color;
    visuals.selection.bg_fill = style.accent_color;
    
    // Customize window appearance
    visuals.window_shadow.extrusion = 8.0;
    visuals.popup_shadow.extrusion = 4.0;
    
    let mut style = (*ctx.style()).clone();
    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(15.0);
    style.spacing.button_padding = egui::vec2(10.0, 6.0);
    
    ctx.set_style(style);
}

pub fn device_card_ui(ui: &mut egui::Ui, card: &DeviceCard, style: &StyleConfig) {
    egui::Frame::none()
        .fill(style.card_bg_color)
        .rounding(10.0)
        .shadow(egui::epaint::Shadow {
            extrusion: 4.0,
            color: Color32::from_black_alpha(20),
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Device icon and basic info
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true), |ui| {
                    let icon = match card.device.service_type {
                        ServiceType::AirDrop => "📱",
                        ServiceType::AirPlay => "🎥",
                        ServiceType::Companion => "🔄",
                        _ => "📍",
                    };
                    
                    ui.add_space(10.0);
                    ui.label(RichText::new(icon).size(24.0));
                    ui.add_space(10.0);

                    ui.vertical(|ui| {
                        ui.label(RichText::new(&card.device.name)
                            .size(16.0)
                            .color(style.text_color)
                            .strong());
                        ui.label(RichText::new(format!("Type: {:?}", card.device.service_type))
                            .size(14.0)
                            .color(style.text_color));
                        ui.small(RichText::new(format!("IP: {}", card.device.address))
                            .color(style.text_color.linear_multiply(0.7)));
                    });
                });

                // Status and controls
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    match &card.status {
                        DeviceStatus::AirDrop(status) => airdrop_controls(ui, status, style),
                        DeviceStatus::AirPlay(status) => airplay_controls(ui, status, style),
                        DeviceStatus::None => {},
                    }
                });
            });
        });
}

fn airdrop_controls(ui: &mut egui::Ui, status: &AirDropStatus, style: &StyleConfig) {
    ui.vertical(|ui| {
        match status {
            AirDropStatus::Idle => {
                ui.label(RichText::new("Ready to share files")
                    .color(style.text_color));
                if ui.button(RichText::new("📤 Send File")
                    .color(style.card_bg_color))
                    .clicked() {
                    // Handle in parent
                }
            },
            AirDropStatus::Connecting => {
                ui.label(RichText::new("Connecting...")
                    .color(style.text_color));
                ui.spinner();
            },
            AirDropStatus::Connected => {
                ui.label(RichText::new("Connected")
                    .color(style.success_color));
            },
            AirDropStatus::Transferring(progress) => {
                ui.label(RichText::new(format!("Transferring: {:.0}%", progress))
                    .color(style.accent_color));
                ui.add(egui::ProgressBar::new(*progress / 100.0)
                    .fill(style.accent_color));
            },
            AirDropStatus::Failed(err) => {
                ui.colored_label(style.error_color, err);
                if ui.button("Retry").clicked() {
                    // Handle in parent
                }
            },
        }
    });
}

fn airplay_controls(ui: &mut egui::Ui, status: &AirPlayStatus, style: &StyleConfig) {
    ui.vertical(|ui| {
        match status {
            AirPlayStatus::Idle => {
                ui.label(RichText::new("Ready to receive")
                    .color(style.text_color));
                if ui.button(RichText::new("▶ Start")
                    .color(style.card_bg_color))
                    .clicked() {
                    // Handle in parent
                }
            },
            AirPlayStatus::Connecting => {
                ui.label(RichText::new("Connecting...")
                    .color(style.text_color));
                ui.spinner();
            },
            AirPlayStatus::Connected => {
                ui.label(RichText::new("Streaming")
                    .color(style.success_color));
                if ui.button(RichText::new("⏹ Stop")
                    .color(style.card_bg_color))
                    .clicked() {
                    // Handle in parent
                }
            },
            AirPlayStatus::Failed(err) => {
                ui.colored_label(style.error_color, err);
                if ui.button("Retry").clicked() {
                    // Handle in parent
                }
            },
        }
    });
}
