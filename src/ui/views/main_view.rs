//! Vista principale dell'applicazione AirWin
//!
//! Questa vista contiene il layout principale con la lista dei dispositivi,
//! il pannello delle azioni e la barra di stato.

use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input, Space,
        horizontal_rule, vertical_rule,
    },
    Alignment, Element, Length,
};

use crate::ui::{
    components,
    messages::{Message, NotificationMessage},
    styles,
    Theme,
};

/// Struttura per la vista principale
pub struct MainView<'a> {
    discovered_devices: &'a [crate::network::DiscoveredDevice],
    selected_device: Option<&'a crate::network::DiscoveredDevice>,
    is_scanning: bool,
    airplay_status: &'a crate::protocols::airplay::AirPlayStatus,
    airdrop_status: &'a crate::protocols::airdrop::AirDropStatus,
    file_transfer_progress: Option<f32>,
    notifications: &'a [NotificationMessage],
    show_link_dialog: bool,
    link_url: &'a str,
}  
/// Helper function to render the main view without constructing a temporary in the caller
pub fn render<'a>(
    discovered_devices: &'a [crate::network::DiscoveredDevice],
    selected_device: Option<&'a crate::network::DiscoveredDevice>,
    is_scanning: bool,
    airplay_status: &'a crate::protocols::airplay::AirPlayStatus,
    airdrop_status: &'a crate::protocols::airdrop::AirDropStatus,
    file_transfer_progress: Option<f32>,
    notifications: &'a [NotificationMessage],
    show_link_dialog: bool,
    link_url: &'a str,
    theme: &Theme,
) -> Element<'a, Message> {
    MainView::new(
        discovered_devices,
        selected_device,
        is_scanning,
        airplay_status,
        airdrop_status,
        file_transfer_progress,
        notifications,
        show_link_dialog,
        link_url,
    )
    .view(theme)
}

impl<'a> MainView<'a> {
    /// Crea una nuova istanza della vista principale
    pub fn new(
        discovered_devices: &'a [crate::network::DiscoveredDevice],
        selected_device: Option<&'a crate::network::DiscoveredDevice>,
        is_scanning: bool,
        airplay_status: &'a crate::protocols::airplay::AirPlayStatus,
        airdrop_status: &'a crate::protocols::airdrop::AirDropStatus,
        file_transfer_progress: Option<f32>,
        notifications: &'a [NotificationMessage],
        show_link_dialog: bool,
        link_url: &'a str,
    ) -> Self {
        Self {
            discovered_devices,
            selected_device,
            is_scanning,
            airplay_status,
            airdrop_status,
            file_transfer_progress,
            notifications,
            show_link_dialog,
            link_url,
        }
    }

    /// Renderizza la vista principale
    pub fn view(&self, theme: &Theme) -> Element<'a, Message> {
        let main_content = row![
            // Pannello sinistro - Lista dispositivi
            self.device_panel(theme),
            
            vertical_rule(1),
            
            // Pannello destro - Azioni e controlli
            self.action_panel(theme),
        ]
        .spacing(styles::spacing::MEDIUM)
        .height(Length::Fill);

        let content = column![
            // Header con titolo e controlli
            self.header(theme),
            
            horizontal_rule(1),
            
            // Contenuto principale
            main_content,
            
            horizontal_rule(1),
            
            // Barra di stato
            self.status_bar(theme),
        ]
        .spacing(styles::spacing::SMALL);

        // Overlay per notificazioni
        let with_notifications = if !self.notifications.is_empty() {
            container(
                column![
                    content,
                    Space::with_height(Length::Fill),
                    self.notifications_overlay(theme),
                ]
            )
        } else {
            container(content)
        };

        // Dialog per invio link
        if self.show_link_dialog {
            let overlay: Element<Message> = with_notifications.into();
            container(
                column![
                    overlay,
                    self.link_dialog(theme),
                ]
            )
            .padding(styles::spacing::MEDIUM.0)
            .into()
        } else {
            with_notifications
                .padding(styles::spacing::MEDIUM.0)
                .into()
        }
    }
    
    /// Header dell'applicazione
    fn header(&self, theme: &Theme) -> Element<'a, Message> {
        row![
            // Titolo
            text("AirWin")
                .size(24)
                .style(styles::colors::TEXT_PRIMARY),
            
            Space::with_width(Length::Fill),
            
            // Controlli header
            row![
                // Pulsante refresh/scansione
                button(
                    text(if self.is_scanning { "⏹" } else { "🔄" })
                        .size(16)
                )
                .on_press(if self.is_scanning {
                    Message::StopScanning
                } else {
                    Message::StartScanning
                }),
                
                // Toggle tema
                button(
                    text(match theme {
                        Theme::Light => "🌙",
                        Theme::Dark => "☀",
                    })
                    .size(16)
                )
                .on_press(Message::ThemeChanged(match theme { Theme::Light => Theme::Dark, Theme::Dark => Theme::Light })),
            ]
            .spacing(styles::spacing::SMALL)
        ]
        .align_items(Alignment::Center)
        .padding(styles::spacing::MEDIUM.0)
        .into()
    }

    /// Pannello dei dispositivi
    fn device_panel(&self, _theme: &Theme) -> Element<'a, Message> {
        let header = row![
            text("Dispositivi Scoperti")
                .size(18)
                .style(styles::colors::TEXT_SECONDARY),
            
            Space::with_width(Length::Fill),
            
            text(format!("({})", self.discovered_devices.len()))
                .size(14)
                .style(styles::colors::TEXT_MUTED),
        ]
        .align_items(Alignment::Center);

        let device_list: Element<'a, Message> = if self.discovered_devices.is_empty() {
            if self.is_scanning {
                container(
                    column![
                        text("🔍")
                            .size(48)
                            .style(styles::colors::TEXT_MUTED),
                        text("Scansione in corso...")
                            .size(16)
                            .style(styles::colors::TEXT_MUTED),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(styles::spacing::MEDIUM)
                )
                .center_x()
                .center_y()
                .height(Length::Fill)
                .into()
            } else {
                container(
                    column![
                        text("📱")
                            .size(48)
                            .style(styles::colors::TEXT_MUTED),
                        text("Nessun dispositivo trovato")
                            .size(16)
                            .style(styles::colors::TEXT_MUTED),
                        text("Premi il pulsante refresh per cercare")
                            .size(14)
                            .style(styles::colors::TEXT_MUTED),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(styles::spacing::SMALL)
                )
                .center_x()
                .center_y()
                .height(Length::Fill)
                .into()
            }
        } else {
            let devices: Element<'a, Message> = self.discovered_devices
                .iter()
                .cloned()
                .fold(
                    column![].spacing(styles::spacing::SMALL),
                    |col, device| {
                        let is_selected = self.selected_device
                            .as_ref()
                            .map(|selected| selected.name == device.name)
                            .unwrap_or(false);
                        
                        let desc = format!("{} • {}:{}", 
                            match device.service_type { 
                                crate::network::ServiceType::AirDrop => "AirDrop",
                                crate::network::ServiceType::AirPlay => "AirPlay",
                                _ => "Altro",
                            },
                            device.address,
                            device.port
                        );
                        col.push(
                            components::selection_card(
                                &device.name,
                                &desc,
                                is_selected,
                                Message::DeviceSelected(device.clone()),
                            )
                        )
                    }
                )
                .into();

            scrollable(devices)
                .height(Length::Fill)
                .into()
        };

        container(
            column![
                header,
                Space::with_height(styles::spacing::MEDIUM),
                device_list,
            ]
        )
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .into()
    }

    /// Pannello delle azioni
    fn action_panel(&self, theme: &Theme) -> Element<'a, Message> {
        let header = text("Azioni")
            .size(18)
            .style(styles::colors::TEXT_SECONDARY);

        let content = if let Some(device) = self.selected_device {
            column![
                // Informazioni dispositivo selezionato
                self.selected_device_info(device, theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Azioni AirDrop
                self.airdrop_actions(theme),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                // Azioni AirPlay (se il servizio selezionato è AirPlay)
                if matches!(device.service_type, crate::network::ServiceType::AirPlay) {
                    self.airplay_actions(theme)
                } else {
                    Space::with_height(0).into()
                },
                
                Space::with_height(Length::Fill),
                
                // Progresso trasferimento
                if let Some(progress) = self.file_transfer_progress {
                    self.transfer_progress(progress, theme)
                } else {
                    Space::with_height(styles::spacing::SMALL).into()
                },
            ]
        } else {
            column![
                container(
                    column![
                        text("👆")
                            .size(48)
                            .style(styles::colors::TEXT_MUTED),
                        text("Seleziona un dispositivo")
                            .size(16)
                            .style(styles::colors::TEXT_MUTED),
                        text("per iniziare")
                            .size(14)
                            .style(styles::colors::TEXT_MUTED),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(styles::spacing::SMALL)
                )
                .center_x()
                .center_y()
                .height(Length::Fill)
            ]
        };

        container(
            column![
                header,
                Space::with_height(styles::spacing::MEDIUM),
                content,
            ]
        )
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .into()
    }

    /// Informazioni del dispositivo selezionato
    fn selected_device_info(
        &self,
        device: &crate::network::DiscoveredDevice,
        _theme: &Theme,
    ) -> Element<'a, Message> {
        column![
            text(&device.name)
                .size(16)
                .style(styles::colors::TEXT_PRIMARY),
            
            text(format!("{} • {}:{}", 
                match device.service_type { 
                    crate::network::ServiceType::AirDrop => "AirDrop",
                    crate::network::ServiceType::AirPlay => "AirPlay",
                    _ => "Altro",
                },
                device.address,
                device.port
            ))
                .size(12)
                .style(styles::colors::TEXT_MUTED),
        ]
        .spacing(styles::spacing::SMALL)
        .into()
    }

    /// Azioni AirDrop
    fn airdrop_actions(&self, _theme: &Theme) -> Element<'a, Message> {
        let status_text = match self.airdrop_status {
            crate::protocols::airdrop::AirDropStatus::Idle => "Pronto",
            crate::protocols::airdrop::AirDropStatus::Connecting => "Connessione...",
            crate::protocols::airdrop::AirDropStatus::Connected => "Connesso",
            crate::protocols::airdrop::AirDropStatus::Transferring(_) => "Trasferimento...",
            crate::protocols::airdrop::AirDropStatus::Failed(_) => "Errore",
        };

        column![
            text("AirDrop")
                .size(14)
                .style(styles::colors::TEXT_SECONDARY),
            
            text(status_text)
                .size(12)
                .style(styles::colors::TEXT_MUTED),
            
            Space::with_height(styles::spacing::SMALL),
            
            button(
                text("📁 Invia File")
                    .size(14)
            )
            .on_press_maybe(
                if matches!(self.airdrop_status, crate::protocols::airdrop::AirDropStatus::Idle | crate::protocols::airdrop::AirDropStatus::Connected) {
                    self.selected_device.map(|d| Message::SendFile(d.clone()))
                } else {
                    None
                }
            )
            .width(Length::Fill),
            
            button(
                text("🔗 Invia Link")
                    .size(14)
            )
            .on_press_maybe(
                if matches!(self.airdrop_status, crate::protocols::airdrop::AirDropStatus::Idle | crate::protocols::airdrop::AirDropStatus::Connected) {
                    Some(Message::ShowLinkDialog)
                } else {
                    None
                }
            )
            .width(Length::Fill),
        ]
        .spacing(styles::spacing::SMALL)
        .into()
    }

    /// Azioni AirPlay
    fn airplay_actions(&self, _theme: &Theme) -> Element<'a, Message> {
        let (status_text, button_text, button_action) = match self.airplay_status {
            crate::protocols::airplay::AirPlayStatus::Idle => {
                ("Disconnesso", "📺 Connetti", self.selected_device.map(|d| Message::StartScreenMirroring(d.clone())))
            },
            crate::protocols::airplay::AirPlayStatus::Connecting => {
                ("Connessione...", "⏳ Connessione...", None)
            },
            crate::protocols::airplay::AirPlayStatus::Connected => {
                ("Connesso", "⏹ Disconnetti", Some(Message::StopScreenMirroring))
            },
            crate::protocols::airplay::AirPlayStatus::Failed(_) => {
                ("Errore", "🔄 Riprova", self.selected_device.map(|d| Message::StartScreenMirroring(d.clone())))
            },
        };

        column![
            text("AirPlay")
                .size(14)
                .style(styles::colors::TEXT_SECONDARY),
            
            text(status_text)
                .size(12)
                .style(styles::colors::TEXT_MUTED),
            
            Space::with_height(styles::spacing::SMALL),
            
            button(
                text(button_text)
                    .size(14)
            )
            .on_press_maybe(button_action)
            .width(Length::Fill),
        ]
        .spacing(styles::spacing::SMALL)
        .into()
    }

    /// Progresso del trasferimento
    fn transfer_progress(&self, progress: f32, _theme: &Theme) -> Element<'a, Message> {
        column![
            text("Trasferimento in corso")
                .size(14)
                .style(styles::colors::TEXT_SECONDARY),
            
            iced::Element::<Message>::from(components::primary_progress_bar(progress)),
            
            text(format!("{:.1}%", progress))
                .size(12)
                .style(styles::colors::TEXT_MUTED),
        ]
        .spacing(styles::spacing::SMALL)
        .into()
    }

    /// Barra di stato
    fn status_bar(&self, _theme: &Theme) -> Element<'a, Message> {
        let left = if self.is_scanning { "Scansione in corso...".to_string() } else { format!("Dispositivi: {}", self.discovered_devices.len()) };
        let right = self.selected_device.map(|d| d.name.clone()).unwrap_or_else(|| "Nessun dispositivo".to_string());
        container(
            row![
                text(left).style(styles::colors::TEXT_SECONDARY),
                Space::with_width(Length::Fill),
                text(right).style(styles::colors::TEXT_MUTED),
            ]
            .align_items(Alignment::Center)
        )
        .padding(styles::spacing::MEDIUM.0)
        .into()
    }

    /// Overlay delle notifiche
    fn notifications_overlay(&self, _theme: &Theme) -> Element<'a, Message> {
        let notifications: Element<Message> = self.notifications
            .iter()
            .fold(
                column![].spacing(styles::spacing::SMALL),
                |col, notification| {
                    col.push(
                        container(
                            column![
                                text(&notification.title).style(styles::colors::TEXT_PRIMARY),
                                text(&notification.content).style(styles::colors::TEXT_SECONDARY),
                            ]
                        )
                        .padding(styles::spacing::SMALL.0)
                    )
                }
            )
            .into();

        container(notifications)
            .padding(styles::spacing::MEDIUM.0)
            .into()
    }

    /// Dialog per l'invio di link
    fn link_dialog(&self, _theme: &Theme) -> Element<'a, Message> {
        let dialog_content = column![
            text("Invia Link")
                .size(18)
                .style(styles::colors::TEXT_SECONDARY),
            
            Space::with_height(styles::spacing::MEDIUM),
            
            text_input("Inserisci URL...", self.link_url)
                .on_input(Message::LinkInputChanged)
                .width(Length::Fill),
            
            Space::with_height(styles::spacing::MEDIUM),
            
            row![
                button(
                    text("Annulla")
                        .size(14)
                )
                .on_press(Message::HideLinkDialog),
                
                Space::with_width(styles::spacing::MEDIUM),
                
                button(
                    text("Invia")
                        .size(14)
                )
                .on_press_maybe(
                    if !self.link_url.trim().is_empty() {
                        self.selected_device.map(|d| Message::SendLink(d.clone(), self.link_url.to_string()))
                    } else {
                        None
                    }
                ),
            ]
            .align_items(Alignment::Center),
        ]
        .spacing(styles::spacing::MEDIUM)
        .max_width(400);

        container(
            container(dialog_content)
                .padding(styles::spacing::LARGE.0)
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        
        .into()
    }
}