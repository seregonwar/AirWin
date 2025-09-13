//! Applicazione principale basata su Iced
//! 
//! Questo modulo implementa il trait Application di Iced per gestire
//! lo stato dell'applicazione, gli aggiornamenti e il rendering.

use iced::{
    widget::{column, container, row, text, Space},
    Application, Command, Element, Length, Subscription,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;

use crate::network::{DeviceDiscovery, DiscoveredDevice};
use crate::protocols::{
    airplay::{AirPlay, AirPlayStatus},
    airdrop::{AirDrop, AirDropStatus},
    awdl::AwdlManager,
};

use super::{
    Message, Theme,
    components::{DeviceList, ActionPanel, StatusBar},
    messages::{NotificationMessage, NotificationType},
    styles,
};

/// Stato dell'applicazione principale
pub struct AirWinApp {
    // Servizi di rete
    discovery: Arc<DeviceDiscovery>,
    airdrop: Arc<Mutex<AirDrop>>,
    airplay: Arc<AirPlay>,
    awdl_manager: Arc<Mutex<AwdlManager>>,
    
    // Stato UI
    theme: Theme,
    discovered_devices: Vec<DiscoveredDevice>,
    selected_device: Option<DiscoveredDevice>,
    is_scanning: bool,
    
    // Stato delle operazioni
    airplay_status: AirPlayStatus,
    airdrop_status: AirDropStatus,
    file_transfer_progress: Option<f32>,
    
    // Dialog e modali
    show_action_dialog: bool,
    show_link_dialog: bool,
    link_input: String,
    
    // Notifiche
    current_notification: Option<NotificationMessage>,
    
    // Stato della finestra
    window_size: (u32, u32),
}

impl AirWinApp {
    /// Crea una nuova istanza dell'applicazione
    pub fn new(
        discovery: Arc<DeviceDiscovery>,
        airdrop: Arc<Mutex<AirDrop>>,
        airplay: Arc<AirPlay>,
        awdl_manager: Arc<Mutex<AwdlManager>>,
    ) -> Self {
        Self {
            discovery,
            airdrop,
            airplay,
            awdl_manager,
            theme: Theme::default(),
            discovered_devices: Vec::new(),
            selected_device: None,
            is_scanning: false,
            airplay_status: AirPlayStatus::Idle,
            airdrop_status: AirDropStatus::Idle,
            file_transfer_progress: None,
            show_action_dialog: false,
            show_link_dialog: false,
            link_input: String::new(),
            current_notification: None,
            window_size: (1200, 800),
        }
    }
    
    /// Avvia la scansione dei dispositivi
    fn start_scanning(&mut self) -> Command<Message> {
        if !self.is_scanning {
            self.is_scanning = true;
            // Comando per avviare la scansione asincrona
            Command::perform(
                async { Message::StartScanning },
                |msg| msg,
            )
        } else {
            Command::none()
        }
    }
    
    /// Ferma la scansione dei dispositivi
    fn stop_scanning(&mut self) -> Command<Message> {
        if self.is_scanning {
            self.is_scanning = false;
            Command::perform(
                async { Message::StopScanning },
                |msg| msg,
            )
        } else {
            Command::none()
        }
    }
    
    /// Gestisce la selezione di un dispositivo
    fn select_device(&mut self, device: DiscoveredDevice) -> Command<Message> {
        self.selected_device = Some(device.clone());
        self.show_action_dialog = true;
        Command::none()
    }
    
    /// Gestisce l'invio di un file
    fn send_file(&mut self, device: DiscoveredDevice) -> Command<Message> {
        self.show_action_dialog = false;
        
        // Comando per aprire il dialog di selezione file
        Command::perform(
            async move {
                use rfd::AsyncFileDialog;
                
                let file = AsyncFileDialog::new()
                    .add_filter("Tutti i file", &["*"])
                    .add_filter("Immagini", &["png", "jpg", "jpeg", "gif", "bmp"])
                    .add_filter("Video", &["mp4", "avi", "mov", "mkv"])
                    .add_filter("Audio", &["mp3", "wav", "flac", "aac"])
                    .add_filter("Documenti", &["pdf", "doc", "docx", "txt"])
                    .pick_file()
                    .await;
                
                match file {
                    Some(file_handle) => {
                        let path = file_handle.path().to_path_buf();
                        Message::FileSelected(Some(path))
                    }
                    None => Message::FileSelected(None),
                }
            },
            |msg| msg,
        )
    }
    
    /// Gestisce l'invio di un link
    fn send_link(&mut self, device: DiscoveredDevice, url: String) -> Command<Message> {
        self.show_link_dialog = false;
        self.link_input.clear();
        
        if url.trim().is_empty() {
            return Command::perform(
                async {
                    Message::ShowNotification(NotificationMessage::warning(
                        "URL vuoto",
                        "Inserisci un URL valido da inviare",
                    ))
                },
                |msg| msg,
            );
        }
        
        // Comando per inviare il link al dispositivo
        Command::perform(
            async move {
                // TODO: Implementare l'invio del link tramite AirDrop
                tokio::time::sleep(Duration::from_millis(1000)).await;
                Message::ShowNotification(NotificationMessage::success(
                    "Link inviato",
                    format!("Link inviato con successo a {}", device.name),
                ))
            },
            |msg| msg,
        )
    }
    
    /// Mostra una notifica
    fn show_notification(&mut self, notification: NotificationMessage) -> Command<Message> {
        self.current_notification = Some(notification.clone());
        
        // Auto-hide della notifica dopo il timeout
        if let Some(duration) = notification.duration_ms {
            Command::perform(
                async move {
                    tokio::time::sleep(Duration::from_millis(duration)).await;
                    Message::HideNotification
                },
                |msg| msg,
            )
        } else {
            Command::none()
        }
    }
}

impl Application for AirWinApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = (
        Arc<DeviceDiscovery>,
        Arc<Mutex<AirDrop>>,
        Arc<AirPlay>,
        Arc<Mutex<AwdlManager>>,
    );

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Self::new(flags.0, flags.1, flags.2, flags.3);
        
        // Avvia automaticamente la scansione
        let command = Command::perform(
            async { Message::StartScanning },
            |msg| msg,
        );
        
        (app, command)
    }

    fn title(&self) -> String {
        "AirWin - Apple Ecosystem Integration".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick => Command::none(),
            
            Message::WindowResized(width, height) => {
                self.window_size = (width, height);
                Command::none()
            }
            
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Command::none()
            }
            
            Message::StartScanning => self.start_scanning(),
            Message::StopScanning => self.stop_scanning(),
            
            Message::DevicesUpdated(devices) => {
                self.discovered_devices = devices;
                Command::none()
            }
            
            Message::DeviceSelected(device) => self.select_device(device),
            Message::DeviceDeselected => {
                self.selected_device = None;
                self.show_action_dialog = false;
                Command::none()
            }
            
            Message::AirDropStatusChanged(status) => {
                self.airdrop_status = status;
                Command::none()
            }
            
            Message::AirPlayStatusChanged(status) => {
                self.airplay_status = status;
                Command::none()
            }
            
            Message::SendFile(device) => self.send_file(device),
            Message::SendLink(device, url) => self.send_link(device, url),
            
            Message::FileSelected(path) => {
                if let Some(_path) = path {
                    // TODO: Implementare l'invio del file
                    Command::perform(
                        async {
                            Message::ShowNotification(NotificationMessage::info(
                                "File selezionato",
                                "Invio del file in corso...",
                            ))
                        },
                        |msg| msg,
                    )
                } else {
                    Command::none()
                }
            }
            
            Message::FileSendProgress(progress) => {
                self.file_transfer_progress = Some(progress);
                Command::none()
            }
            
            Message::FileSendCompleted(result) => {
                self.file_transfer_progress = None;
                match result {
                    Ok(()) => Command::perform(
                        async {
                            Message::ShowNotification(NotificationMessage::success(
                                "File inviato",
                                "Il file è stato inviato con successo",
                            ))
                        },
                        |msg| msg,
                    ),
                    Err(error) => Command::perform(
                        async move {
                            Message::ShowNotification(NotificationMessage::error(
                                "Errore invio file",
                                format!("Errore durante l'invio: {}", error),
                            ))
                        },
                        |msg| msg,
                    ),
                }
            }
            
            Message::ShowActionDialog(device) => {
                self.selected_device = Some(device);
                self.show_action_dialog = true;
                Command::none()
            }
            
            Message::HideActionDialog => {
                self.show_action_dialog = false;
                Command::none()
            }
            
            Message::ShowLinkDialog => {
                self.show_link_dialog = true;
                Command::none()
            }
            
            Message::HideLinkDialog => {
                self.show_link_dialog = false;
                self.link_input.clear();
                Command::none()
            }
            
            Message::LinkInputChanged(input) => {
                self.link_input = input;
                Command::none()
            }
            
            Message::ShowNotification(notification) => {
                self.show_notification(notification)
            }
            
            Message::HideNotification => {
                self.current_notification = None;
                Command::none()
            }
            
            Message::Error(error) => {
                Command::perform(
                    async move {
                        Message::ShowNotification(NotificationMessage::error(
                            "Errore",
                            error,
                        ))
                    },
                    |msg| msg,
                )
            }
            
            Message::Warning(warning) => {
                Command::perform(
                    async move {
                        Message::ShowNotification(NotificationMessage::warning(
                            "Attenzione",
                            warning,
                        ))
                    },
                    |msg| msg,
                )
            }
            
            Message::Info(info) => {
                Command::perform(
                    async move {
                        Message::ShowNotification(NotificationMessage::info(
                            "Informazione",
                            info,
                        ))
                    },
                    |msg| msg,
                )
            }
            
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = column![
            // Barra di stato superiore
            StatusBar::new(
                self.is_scanning,
                self.discovered_devices.len(),
                &self.airplay_status,
                &self.airdrop_status,
            ).view(&self.theme),
            
            Space::with_height(styles::spacing::MEDIUM),
            
            // Contenuto principale
            row![
                // Lista dispositivi
                DeviceList::new(
                    &self.discovered_devices,
                    self.selected_device.as_ref(),
                    self.is_scanning,
                ).view(&self.theme),
                
                Space::with_width(styles::spacing::LARGE),
                
                // Pannello azioni
                ActionPanel::new(
                    self.selected_device.as_ref(),
                    &self.airplay_status,
                    &self.airdrop_status,
                    self.file_transfer_progress,
                ).view(&self.theme),
            ]
            .spacing(styles::spacing::LARGE)
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .spacing(styles::spacing::MEDIUM)
        .padding(styles::spacing::LARGE);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |theme: &Theme| styles::container_primary(theme))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // Subscription per aggiornamenti periodici
        iced::time::every(Duration::from_millis(100))
            .map(|_| Message::Tick)
    }

    fn theme(&self) -> Self::Theme {
        self.theme
    }
}