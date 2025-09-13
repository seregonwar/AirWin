//! Componente per mostrare errori in modo user-friendly

use iced::{
    widget::{button, column, container, row, text, Space},
    Alignment, Element, Length,
};

use crate::ui::{
    messages::Message,
    styles,
};

/// Dialog per mostrare errori all'utente
pub struct ErrorDialog {
    pub title: String,
    pub message: String,
    pub details: Option<String>,
    pub is_visible: bool,
}

impl ErrorDialog {
    /// Crea un nuovo dialog di errore
    pub fn new(title: String, message: String) -> Self {
        Self {
            title,
            message,
            details: None,
            is_visible: true,
        }
    }

    /// Aggiunge dettagli tecnici all'errore
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    /// Renderizza il dialog
    pub fn view<'a>(&self) -> Element<'a, Message> {
        if !self.is_visible {
            return Space::with_height(0).into();
        }

        let content = column![
            // Icona e titolo
            row![
                text("⚠️").size(24),
                Space::with_width(10),
                text(&self.title)
                    .size(18)
                    .style(styles::colors::ERROR),
            ]
            .align_items(Alignment::Center),
            
            Space::with_height(15),
            
            // Messaggio principale
            text(&self.message)
                .size(14)
                .style(styles::colors::TEXT_PRIMARY),
        ];

        let content = if let Some(ref details) = self.details {
            content.push(Space::with_height(10))
                .push(
                    container(
                        text(details)
                            .size(12)
                            .style(styles::colors::TEXT_MUTED)
                    )
                    .padding(10)
                    .style(|_theme: &iced::Theme| {
                        iced::widget::container::Appearance {
                            text_color: Some(styles::colors::TEXT_MUTED),
                            background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.1))),
                            border: iced::Border {
                                color: styles::colors::BORDER,
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            shadow: Default::default(),
                        }
                    })
                )
        } else {
            content
        };

        let content = content
            .push(Space::with_height(20))
            .push(
                row![
                    Space::with_width(Length::Fill),
                    button(text("OK").size(14))
                        .on_press(Message::HideError)
                        .style(iced::theme::Button::Primary)
                ]
            );

        container(
            container(content.spacing(5).width(400))
                .padding(20)
                .style(|_theme: &iced::Theme| {
                    iced::widget::container::Appearance {
                        text_color: Some(styles::colors::TEXT_PRIMARY),
                        background: Some(iced::Background::Color(styles::colors::BACKGROUND)),
                        border: iced::Border {
                            color: styles::colors::ERROR,
                            width: 2.0,
                            radius: 8.0.into(),
                        },
                        shadow: iced::Shadow {
                            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                            offset: iced::Vector::new(0.0, 4.0),
                            blur_radius: 10.0,
                        },
                    }
                })
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &iced::Theme| {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                ..Default::default()
            }
        })
        .into()
    }
}

/// Helper per creare un dialog di errore da un Result
pub fn from_error<E: std::fmt::Display>(error: E) -> ErrorDialog {
    let error_str = error.to_string();
    
    // Gestione speciale per errori comuni
    let (title, message, details) = if error_str.contains("10048") || error_str.contains("port") || error_str.contains("bind") {
        (
            "Porta già in uso".to_string(),
            "Un'altra applicazione sta già utilizzando la porta richiesta. AirWin continuerà con funzionalità limitate.".to_string(),
            Some(error_str),
        )
    } else if error_str.contains("network") || error_str.contains("Network") {
        (
            "Errore di rete".to_string(),
            "Si è verificato un problema di connessione. Verifica la tua connessione di rete.".to_string(),
            Some(error_str),
        )
    } else if error_str.contains("permission") || error_str.contains("Permission") {
        (
            "Permessi insufficienti".to_string(),
            "L'applicazione non ha i permessi necessari. Prova ad eseguirla come amministratore.".to_string(),
            Some(error_str),
        )
    } else {
        (
            "Errore".to_string(),
            "Si è verificato un errore imprevisto.".to_string(),
            Some(error_str),
        )
    };

    ErrorDialog::new(title, message).with_details(details.unwrap_or_default())
}
