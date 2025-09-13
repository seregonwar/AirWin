//! Definizione dei messaggi per l'architettura Iced
//! 
//! Questo modulo contiene tutti i messaggi che possono essere inviati
//! nell'applicazione per gestire gli eventi e le azioni dell'utente.

use crate::network::DiscoveredDevice;
use crate::protocols::airplay::AirPlayStatus;
use crate::protocols::airdrop::AirDropStatus;
use std::path::PathBuf;

/// Messaggi principali dell'applicazione
#[derive(Debug, Clone)]
pub enum Message {
    // Messaggi di sistema
    Tick,
    WindowResized(u32, u32),
    ThemeChanged(crate::ui::Theme),
    InitializationComplete,
    
    // Messaggi di discovery
    StartScanning,
    StopScanning,
    DevicesUpdated(Vec<DiscoveredDevice>),
    DeviceSelected(DiscoveredDevice),
    DeviceDeselected,
    
    // Messaggi di AirDrop
    AirDropStatusChanged(AirDropStatus),
    SendFile(DiscoveredDevice),
    SendLink(DiscoveredDevice, String),
    FileSelected(Option<PathBuf>),
    FileSendProgress(f32),
    FileSendCompleted(Result<(), String>),
    
    // Messaggi di AirPlay
    AirPlayStatusChanged(AirPlayStatus),
    StartScreenMirroring(DiscoveredDevice),
    StopScreenMirroring,
    ScreenMirroringFrame(Vec<u8>),
    
    // Messaggi di interfaccia
    ShowActionDialog(DiscoveredDevice),
    HideActionDialog,
    ShowLinkDialog,
    HideLinkDialog,
    LinkInputChanged(String),
    
    // Messaggi di notifica
    ShowNotification(NotificationMessage),
    HideNotification,
    
    // Messaggi di errore
    Error(String),
    Warning(String),
    Info(String),
    ClearError,
    HideError,
    
    // Messaggi per le impostazioni
    CustomPortChanged,
    ToggleDebugMode,
    LogLevelChanged,
    MaxConcurrentTransfersChanged,
    OpenLogFolder,
    ClearCache,
    RunDiagnostics,
    
    // Messaggi per la navigazione
    ShowMainView,
    
    // Messaggi per i link esterni
    OpenLicenses,
    OpenWebsite,
    OpenDocumentation,
    OpenIssues,
    OpenFeatureRequest,
}

/// Tipi di notifiche
#[derive(Debug, Clone)]
pub struct NotificationMessage {
    pub title: String,
    pub content: String,
    pub notification_type: NotificationType,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Warning,
    Error,
    Info,
}

impl NotificationMessage {
    pub fn success(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            notification_type: NotificationType::Success,
            duration_ms: Some(3000),
        }
    }
    
    pub fn error(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            notification_type: NotificationType::Error,
            duration_ms: Some(5000),
        }
    }
    
    pub fn warning(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            notification_type: NotificationType::Warning,
            duration_ms: Some(4000),
        }
    }
    
    pub fn info(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            notification_type: NotificationType::Info,
            duration_ms: Some(3000),
        }
    }
}

/// Subscription messages per eventi asincroni
#[derive(Debug, Clone)]
pub enum SubscriptionMessage {
    DeviceDiscoveryUpdate(Vec<DiscoveredDevice>),
    AirPlayStatusUpdate(AirPlayStatus),
    AirDropStatusUpdate(AirDropStatus),
    ScreenFrame(Vec<u8>),
    FileTransferProgress(f32),
}

impl From<SubscriptionMessage> for Message {
    fn from(sub_msg: SubscriptionMessage) -> Self {
        match sub_msg {
            SubscriptionMessage::DeviceDiscoveryUpdate(devices) => Message::DevicesUpdated(devices),
            SubscriptionMessage::AirPlayStatusUpdate(status) => Message::AirPlayStatusChanged(status),
            SubscriptionMessage::AirDropStatusUpdate(status) => Message::AirDropStatusChanged(status),
            SubscriptionMessage::ScreenFrame(frame) => Message::ScreenMirroringFrame(frame),
            SubscriptionMessage::FileTransferProgress(progress) => Message::FileSendProgress(progress),
        }
    }
}