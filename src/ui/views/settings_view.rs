//! Vista delle impostazioni dell'applicazione AirWin
//!
//! Questa vista permette di configurare le preferenze dell'applicazione,
//! le impostazioni di rete e i protocolli di comunicazione.

use iced::{
    widget::{
        button, checkbox, column, container, pick_list, row, scrollable, text,
        text_input, Space, horizontal_rule, slider,
    },
    Alignment, Element, Length,
};

use crate::ui::{
    messages::Message,
    styles,
    Theme,
};

// Scelte statiche per i controlli `pick_list` per evitare riferimenti a temporanei
const AIRDROP_VISIBILITIES: [AirDropVisibility; 3] = [
    AirDropVisibility::Everyone,
    AirDropVisibility::ContactsOnly,
    AirDropVisibility::ReceivingOff,
];
 
const AIRPLAY_QUALITIES: [AirPlayQuality; 4] = [
    AirPlayQuality::Auto,
    AirPlayQuality::Low,
    AirPlayQuality::Medium,
    AirPlayQuality::High,
]; 
 
const LOG_LEVELS: [LogLevel; 5] = [
    LogLevel::Error,
    LogLevel::Warn,
    LogLevel::Info,
    LogLevel::Debug,
    LogLevel::Trace,
];

// Scelte vuote statiche per l'elenco interfacce di rete (placeholder)
const EMPTY_INTERFACES: [&str; 0] = [];

/// Struttura per la vista delle impostazioni
#[derive(Debug, Clone)]
pub struct SettingsView {
    // Impostazioni generali
    auto_discovery: bool,
    discovery_interval: u32,
    show_notifications: bool,
    minimize_to_tray: bool,
    
    // Impostazioni AirDrop
    airdrop_enabled: bool,
    airdrop_visibility: AirDropVisibility,
    auto_accept_from_contacts: bool,
    
    // Impostazioni AirPlay
    airplay_enabled: bool,
    airplay_quality: AirPlayQuality,
    airplay_audio_only: bool,
    
    // Impostazioni di rete
    network_interface: Option<String>,
    available_interfaces: Vec<String>,
    custom_port: Option<u16>,
    // Versione testuale persistente della porta personalizzata per `text_input`
    custom_port_text: String,
    
    // Impostazioni avanzate
    debug_mode: bool,
    log_level: LogLevel,
    max_concurrent_transfers: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AirDropVisibility {
    Everyone,
    ContactsOnly,
    ReceivingOff,
}

impl std::fmt::Display for AirDropVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AirDropVisibility::Everyone => write!(f, "Tutti"),
            AirDropVisibility::ContactsOnly => write!(f, "Solo Contatti"),
            AirDropVisibility::ReceivingOff => write!(f, "Disattivato"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AirPlayQuality {
    Low,
    Medium,
    High,
    Auto,
}

impl std::fmt::Display for AirPlayQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AirPlayQuality::Low => write!(f, "Bassa"),
            AirPlayQuality::Medium => write!(f, "Media"),
            AirPlayQuality::High => write!(f, "Alta"),
            AirPlayQuality::Auto => write!(f, "Automatica"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "Errore"),
            LogLevel::Warn => write!(f, "Avviso"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Trace => write!(f, "Trace"),
        }
    }
}

impl SettingsView {
    /// Crea una nuova istanza della vista impostazioni
    pub fn new(
        auto_discovery: bool,
        discovery_interval: u32,
        show_notifications: bool,
        minimize_to_tray: bool,
        airdrop_enabled: bool,
        airdrop_visibility: AirDropVisibility,
        auto_accept_from_contacts: bool,
        airplay_enabled: bool,
        airplay_quality: AirPlayQuality,
        airplay_audio_only: bool,
        network_interface: Option<String>,
        available_interfaces: Vec<String>,
        custom_port: Option<u16>,
        debug_mode: bool,
        log_level: LogLevel,
        max_concurrent_transfers: u32,
    ) -> Self {
        Self {
            auto_discovery,
            discovery_interval,
            show_notifications,
            minimize_to_tray,
            airdrop_enabled,
            airdrop_visibility,
            auto_accept_from_contacts,
            airplay_enabled,
            airplay_quality,
            airplay_audio_only,
            network_interface,
            available_interfaces,
            custom_port,
            custom_port_text: custom_port.map(|p| p.to_string()).unwrap_or_default(),
            debug_mode,
            log_level,
            max_concurrent_transfers,
        }
    }

    /// Sezione impostazioni AirPlay
    fn airplay_settings(&self, _theme: &Theme) -> Element<Message> {
        let section_header = text("AirPlay")
            .size(18);

        let settings = column![
            // AirPlay abilitato
            checkbox(
                "Abilita AirPlay",
                self.airplay_enabled
            )
            .on_toggle(|_| Message::Tick),
            
            if self.airplay_enabled {
                column![
                    // Qualità
                    row![
                        text("Qualità video:")
                            .size(14)
                            .width(Length::FillPortion(1)),
                        
                        pick_list(
                            &AIRPLAY_QUALITIES[..],
                            Some(self.airplay_quality.clone()),
                            |_| Message::Tick
                        )
                        .width(Length::FillPortion(2)),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(styles::spacing::MEDIUM),
                    
                    // Solo audio
                    checkbox(
                        "Solo audio (migliori prestazioni)",
                        self.airplay_audio_only
                    )
                    .on_toggle(|_| Message::Tick),
                ]
                .spacing(styles::spacing::MEDIUM)
            } else {
                column![]
            },
        ]
        .spacing(styles::spacing::MEDIUM);

        container(
            column![
                section_header,
                Space::with_height(styles::spacing::MEDIUM),
                settings,
            ]
        )
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Renderizza la vista delle impostazioni
    pub fn view(&self, theme: &Theme) -> Element<Message> {
        let header = row![
            button(
                text("← Indietro")
                    .size(14)
            )
            .on_press(Message::ShowMainView)
            .style(iced::theme::Button::Secondary),
            
            Space::with_width(styles::spacing::MEDIUM),
            
            text("Impostazioni")
                .size(24)
                ,
            
            Space::with_width(Length::Fill),
            
            button(
                text("💾 Salva")
                    .size(14)
            )
            // Placeholder azione salvataggio
            .on_press(Message::Tick)
            .style(iced::theme::Button::Primary),
            
            button(
                text("🔄 Reset")
                    .size(14)
            )
            // Placeholder azione reset
            .on_press(Message::Tick)
            .style(iced::theme::Button::Secondary),
        ]
        .align_items(Alignment::Center)
        .padding(styles::spacing::MEDIUM.0);

        let content = scrollable(
            column![
                // Impostazioni generali
                self.general_settings(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Impostazioni AirDrop
                self.airdrop_settings(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Impostazioni AirPlay
                self.airplay_settings(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Impostazioni di rete
                self.network_settings(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Impostazioni avanzate
                self.advanced_settings(theme),
                
                Space::with_height(styles::spacing::LARGE),
            ]
            .spacing(0)
        )
        .height(Length::Fill);

        container(
            column![
                header,
                horizontal_rule(1),
                content,
            ]
        )
        .padding(styles::spacing::MEDIUM.0)
        .into()
    }

    /// Sezione impostazioni generali
    fn general_settings(&self, _theme: &Theme) -> Element<Message> {
        let section_header = text("Generale")
            .size(18);

        let settings = column![
            // Auto discovery
            row![
                checkbox(
                    "Scoperta automatica dispositivi",
                    self.auto_discovery
                )
                .on_toggle(|_| Message::Tick),
            ],
            
            // Discovery interval
            if self.auto_discovery {
                column![
                    text(format!("Intervallo scansione: {} secondi", self.discovery_interval))
                        .size(14)
                        ,
                    
                    slider(
                        5..=60,
                        self.discovery_interval,
                        |_| Message::Tick
                    )
                    ,
                ]
                .spacing(styles::spacing::SMALL)
            } else {
                column![]
            },
            
            // Notifiche
            checkbox(
                "Mostra notifiche",
                self.show_notifications
            )
            .on_toggle(|_| Message::Tick),
            
            // Minimize to tray
            checkbox(
                "Minimizza nella system tray",
                self.minimize_to_tray
            )
            .on_toggle(|_| Message::Tick),
        ]
        .spacing(styles::spacing::MEDIUM);

        container(
            column![
                section_header,
                Space::with_height(styles::spacing::MEDIUM),
                settings,
            ]
        )
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Sezione impostazioni AirDrop
    fn airdrop_settings(&self, _theme: &Theme) -> Element<Message> {
        let section_header = text("AirDrop")
            .size(18);

        let settings = column![
            // AirDrop abilitato
            checkbox(
                "Abilita AirDrop",
                self.airdrop_enabled
            )
            .on_toggle(|_| Message::Tick),
            
            if self.airdrop_enabled {
                column![
                    // Visibilità
                    row![
                        text("Visibilità:")
                            .size(14)
                            
                            .width(Length::FillPortion(1)),
                        
                        pick_list(
                            &AIRDROP_VISIBILITIES[..],
                            Some(self.airdrop_visibility.clone()),
                            |_| Message::Tick
                        )
                        
                        .width(Length::FillPortion(2)),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(styles::spacing::MEDIUM),
                    
                    // Auto accept da contatti
                    checkbox(
                        "Accetta automaticamente da contatti",
                        self.auto_accept_from_contacts
                    )
                    .on_toggle(|_| Message::Tick),
                ]
                .spacing(styles::spacing::MEDIUM)
            } else {
                column![]
            },
        ]
        .spacing(styles::spacing::MEDIUM);

        container(
            column![
                section_header,
                Space::with_height(styles::spacing::MEDIUM),
                settings,
            ]
        )
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Sezione impostazioni di rete
    fn network_settings(&self, _theme: &Theme) -> Element<Message> {
        let section_header = text("Rete")
            .size(18);

        let settings = column![
            // Interfaccia di rete
            row![
                text("Interfaccia di rete:")
                    .size(14)
                    
                    .width(Length::FillPortion(1)),
                
                pick_list(
                    &EMPTY_INTERFACES[..],
                    None::<&str>,
                    |_| Message::Tick
                )
                .placeholder("Automatica")
                
                .width(Length::FillPortion(2)),
            ]
            .align_items(Alignment::Center)
            .spacing(styles::spacing::MEDIUM),
            
            // Porta personalizzata
            row![
                text("Porta personalizzata:")
                    .size(14)
                    
                    .width(Length::FillPortion(1)),
                
                text_input(
                    "Automatica",
                    ""
                )
                .on_input(|_| Message::Tick)
                .width(Length::FillPortion(2)),
            ]
            .align_items(Alignment::Center)
            .spacing(styles::spacing::MEDIUM),
        ]
        .spacing(styles::spacing::MEDIUM);

        container(
            column![
                section_header,
                Space::with_height(styles::spacing::MEDIUM),
                settings,
            ]
        )
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Sezione impostazioni avanzate
    fn advanced_settings(&self, _theme: &Theme) -> Element<Message> {
        let section_header = text("Avanzate")
            .size(18);

        let settings = column![
            // Debug mode
            checkbox(
                "Modalità debug",
                self.debug_mode
            )
            .on_toggle(|_| Message::ToggleDebugMode)
            ,
            
            // Log level
            row![
                text("Livello di log:")
                    .size(14)
                    
                    .width(Length::FillPortion(1)),
                
                pick_list(
                    &LOG_LEVELS[..],
                    Some(self.log_level.clone()),
                    |_| Message::Tick
                )
                
                .width(Length::FillPortion(2)),
            ]
            .align_items(Alignment::Center)
            .spacing(styles::spacing::MEDIUM),
            
            // Max concurrent transfers
            column![
                text(format!("Trasferimenti simultanei: {}", self.max_concurrent_transfers))
                    .size(14)
                    ,
                
                slider(
                    1..=10,
                    self.max_concurrent_transfers,
                    |_| Message::Tick
                )
                ,
            ]
            .spacing(styles::spacing::SMALL),
            
            // Azioni avanzate
            row![
                button(
                    text("🗂 Apri Log")
                        .size(14)
                )
                .on_press(Message::OpenLogFolder)
                .style(iced::theme::Button::Secondary),
                
                button(
                    text("🧹 Pulisci Cache")
                        .size(14)
                )
                .on_press(Message::ClearCache)
                .style(iced::theme::Button::Secondary),
                
                button(
                    text("📊 Diagnostica")
                        .size(14)
                )
                .on_press(Message::RunDiagnostics)
                .style(iced::theme::Button::Secondary),
            ]
            .spacing(styles::spacing::MEDIUM),
        ]
        .spacing(styles::spacing::MEDIUM);

        container(
            column![
                section_header,
                Space::with_height(styles::spacing::MEDIUM),
                settings,
            ]
        )
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }
}