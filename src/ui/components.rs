//! Componenti UI riutilizzabili per AirWin
//!
//! Questo modulo contiene componenti personalizzati per creare
//! un'interfaccia utente coerente e moderna.

pub mod error_dialog;
pub use error_dialog::{ErrorDialog, from_error};

use iced::{
    widget::{
        button, column, container, progress_bar, row, text, Button, Column, Container, ProgressBar,
        Row, Text, Space,
    },
    Alignment, Background, Border, Element, Length, Theme,
};

use super::{styles, Message};

/// Componente per il titolo principale
pub fn title<'a>(content: &str) -> Text<'a> {
    text(content)
        .size(styles::font_size::XLARGE)
        .style(styles::colors::TEXT_PRIMARY)
}

/// Componente per il sottotitolo
pub fn subtitle<'a>(content: &str) -> Text<'a> {
    text(content)
        .size(styles::font_size::LARGE)
        .style(styles::colors::TEXT_SECONDARY)
} 

/// Componente per il testo normale
pub fn body_text<'a>(content: &str) -> Text<'a> {
    text(content)
        .size(styles::font_size::MEDIUM)
        .style(styles::colors::TEXT_PRIMARY)
}

/// Componente per il testo secondario
pub fn secondary_text<'a>(content: &str) -> Text<'a> {
    text(content)
        .size(styles::font_size::SMALL)
        .style(styles::colors::TEXT_SECONDARY)
}

/// Componente per il testo muto
pub fn muted_text<'a>(content: &str) -> Text<'a> {
    text(content)
        .size(styles::font_size::SMALL)
        .style(styles::colors::TEXT_MUTED)
}

/// Componente per il testo di successo
pub fn success_text<'a>(content: &str) -> Text<'a> {
    text(content)
        .size(styles::font_size::MEDIUM)
        .style(styles::colors::SUCCESS)
}

/// Componente per il testo di errore
pub fn error_text<'a>(content: &str) -> Text<'a> {
    text(content)
        .size(styles::font_size::MEDIUM)
        .style(styles::colors::ERROR)
}

/// Componente per il testo di avviso
pub fn warning_text<'a>(content: &str) -> Text<'a> {
    text(content)
        .size(styles::font_size::MEDIUM)
        .style(styles::colors::WARNING)
}

/// Pulsante primario
pub fn primary_button<'a>(content: &str, message: Message) -> Button<'a, Message> {
    button(text(content).size(styles::font_size::MEDIUM))
        .style(iced::theme::Button::Primary)
        .padding([styles::spacing::SMALL.0, styles::spacing::MEDIUM.0])
        .on_press(message)
}

/// Pulsante secondario
pub fn secondary_button<'a>(content: &str, message: Message) -> Button<'a, Message> {
    button(text(content).size(styles::font_size::MEDIUM))
        .style(iced::theme::Button::Secondary)
        .padding([styles::spacing::SMALL.0, styles::spacing::MEDIUM.0])
        .on_press(message)
}

/// Pulsante card (per selezioni)
pub fn card_button<'a>(content: &str, message: Message) -> Button<'a, Message> {
    button(text(content).size(styles::font_size::MEDIUM))
        .style(iced::theme::Button::Secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .on_press(message)
}

/// Pulsante selezionato
pub fn selected_button<'a>(content: &str, message: Message) -> Button<'a, Message> {
    button(text(content).size(styles::font_size::MEDIUM))
        .style(iced::theme::Button::Secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .on_press(message)
}

/// Pulsante ghost (trasparente)
pub fn ghost_button<'a>(content: &str, message: Message) -> Button<'a, Message> {
    button(text(content).size(styles::font_size::SMALL))
        .style(iced::theme::Button::Text)
        .padding([styles::spacing::TINY.0, styles::spacing::SMALL.0])
        .on_press(message)
}

/// Container principale
pub fn main_container<'a>(content: Element<'a, Message>) -> Container<'a, Message> {
    container(content)
        .style(styles::container_primary)
        .padding(styles::spacing::LARGE.0)
        .width(Length::Fill)
        .height(Length::Fill)
}

/// Container secondario
pub fn secondary_container<'a>(content: Element<'a, Message>) -> Container<'a, Message> {
    container(content)
        .style(styles::container_secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
}

/// Container per l'header
pub fn header_container<'a>(content: Element<'a, Message>) -> Container<'a, Message> {
    container(content)
        .style(styles::container_header)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
}

/// Container per notifiche di successo
pub fn success_container<'a>(content: Element<'a, Message>) -> Container<'a, Message> {
    container(content)
        .style(styles::container_success)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
}

/// Container per notifiche di errore
pub fn error_container<'a>(content: Element<'a, Message>) -> Container<'a, Message> {
    container(content)
        .style(styles::container_error)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
}

/// Container per notifiche di avviso
pub fn warning_container<'a>(content: Element<'a, Message>) -> Container<'a, Message> {
    container(content)
        .style(styles::container_warning)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
}

/// Container per notifiche informative
pub fn info_container<'a>(content: Element<'a, Message>) -> Container<'a, Message> {
    container(content)
        .style(styles::container_info)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
}

/// Barra di progresso primaria
pub fn primary_progress_bar(value: f32) -> ProgressBar {
    progress_bar(0.0..=100.0, value)
        .style(styles::progress_bar_primary)
        .height(8)
}

/// Barra di progresso di successo
pub fn success_progress_bar(value: f32) -> ProgressBar {
    progress_bar(0.0..=100.0, value)
        .style(styles::progress_bar_success)
        .height(8)
}

/// Barra di progresso di avviso
pub fn warning_progress_bar(value: f32) -> ProgressBar {
    progress_bar(0.0..=100.0, value)
        .style(styles::progress_bar_warning)
        .height(8)
}

/// Barra di progresso di errore
pub fn error_progress_bar(value: f32) -> ProgressBar {
    progress_bar(0.0..=100.0, value)
        .style(styles::progress_bar_error)
        .height(8)
}

/// Layout a colonna con spaziatura standard
pub fn spaced_column<'a>(children: Vec<Element<'a, Message>>) -> Column<'a, Message> {
    column(children)
        .spacing(styles::spacing::MEDIUM)
        .width(Length::Fill)
}

/// Layout a colonna con spaziatura piccola
pub fn tight_column<'a>(children: Vec<Element<'a, Message>>) -> Column<'a, Message> {
    column(children)
        .spacing(styles::spacing::SMALL)
        .width(Length::Fill)
}

/// Layout a colonna con spaziatura grande
pub fn loose_column<'a>(children: Vec<Element<'a, Message>>) -> Column<'a, Message> {
    column(children)
        .spacing(styles::spacing::LARGE)
        .width(Length::Fill)
}

/// Layout a riga con spaziatura standard
pub fn spaced_row<'a>(children: Vec<Element<'a, Message>>) -> Row<'a, Message> {
    row(children)
        .spacing(styles::spacing::MEDIUM)
        .align_items(Alignment::Center)
}

/// Layout a riga con spaziatura piccola
pub fn tight_row<'a>(children: Vec<Element<'a, Message>>) -> Row<'a, Message> {
    row(children)
        .spacing(styles::spacing::SMALL)
        .align_items(Alignment::Center)
}

/// Layout a riga con spaziatura grande
pub fn loose_row<'a>(children: Vec<Element<'a, Message>>) -> Row<'a, Message> {
    row(children)
        .spacing(styles::spacing::LARGE)
        .align_items(Alignment::Center)
}

/// Card component per visualizzare informazioni
pub fn info_card<'a>(
    title_text: &str,
    description: &str,
    action_text: Option<&str>,
    action_message: Option<Message>,
) -> Container<'a, Message> {
    let mut content = vec![
        title(title_text).into(),
        secondary_text(description).into(),
    ];

    if let (Some(text), Some(message)) = (action_text, action_message) {
        content.push(primary_button(text, message).into());
    }

    secondary_container(spaced_column(content).into())
}

/// Card component per selezioni
pub fn selection_card<'a>(
    title: &str,
    description: &str,
    is_selected: bool,
    message: Message,
) -> Element<'a, Message> {
    let _content = spaced_column(vec![
        body_text(title).into(),
        muted_text(description).into(),
    ]);

    let button = if is_selected {
        selected_button("", message)
    } else {
        card_button("", message)
    };

    button.into()
}

/// Notification component
pub fn notification<'a>(
    message: &str,
    notification_type: NotificationType,
) -> Container<'a, Message> {
    match notification_type {
        NotificationType::Success => success_container(success_text(message).into()),
        NotificationType::Error => error_container(error_text(message).into()),
        NotificationType::Warning => warning_container(warning_text(message).into()),
        NotificationType::Info => info_container(body_text(message).into()),
    }
}

/// Tipo di notifica
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Success,
    Error,
    Warning,
    Info,
}

/// Header component con titolo e sottotitolo
pub fn page_header<'a>(title_text: &str, subtitle_text: Option<&str>) -> Container<'a, Message> {
    let mut content = vec![title(title_text).into()];

    if let Some(sub) = subtitle_text {
        content.push(subtitle(sub).into());
    }

    header_container(spaced_column(content).into())
}

/// Status indicator component
pub fn status_indicator<'a>(
    label: &str,
    status: StatusType,
    value: Option<&str>,
) -> Element<'a, Message> {
    let status_text = match status {
        StatusType::Active => success_text("Attivo"),
        StatusType::Inactive => muted_text("Inattivo"),
        StatusType::Error => error_text("Errore"),
        StatusType::Warning => warning_text("Attenzione"),
        StatusType::Processing => body_text("In elaborazione"),
    };

    let mut row_content = vec![
        secondary_text(label).into(),
        status_text.into(),
    ];

    if let Some(val) = value {
        row_content.push(body_text(val).into());
    }

    spaced_row(row_content).into()
}

/// Tipo di stato
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusType {
    Active,
    Inactive,
    Error,
    Warning,
    Processing,
}

/// Progress indicator con etichetta
pub fn labeled_progress<'a>(
    label: &str,
    value: f32,
    progress_type: ProgressType,
) -> Element<'a, Message> {
    let progress_bar = match progress_type {
        ProgressType::Primary => primary_progress_bar(value),
        ProgressType::Success => success_progress_bar(value),
        ProgressType::Warning => warning_progress_bar(value),
        ProgressType::Error => error_progress_bar(value),
    };

    spaced_column(vec![
        spaced_row(vec![
            secondary_text(label).into(),
            body_text(&format!("{:.0}%", value)).into(),
        ])
        .into(),
        progress_bar.into(),
    ])
    .into()
}

/// Tipo di progress bar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressType {
    Primary,
    Success,
    Warning,
    Error,
}

/// Spacer component per aggiungere spazio
pub fn spacer<'a>(size: SpacerSize) -> Element<'a, Message> {
    let height = match size {
        SpacerSize::Small => styles::spacing::SMALL,
        SpacerSize::Medium => styles::spacing::MEDIUM,
        SpacerSize::Large => styles::spacing::LARGE,
        SpacerSize::XLarge => styles::spacing::XLARGE,
    };

    container(text(""))
        .height(height)
        .into()
}

/// Dimensioni del spacer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacerSize {
    Small,
    Medium,
    Large,
    XLarge,
}

/// Action bar con pulsanti
pub fn action_bar<'a>(actions: Vec<(String, Message)>) -> Element<'a, Message> {
    let buttons: Vec<Element<'a, Message>> = actions
        .into_iter()
        .enumerate()
        .map(|(i, (text, message))| {
            if i == 0 {
                primary_button(&text, message).into()
            } else {
                secondary_button(&text, message).into()
            }
        })
        .collect();

    spaced_row(buttons).into()
}

/// Divider component
pub fn divider<'a>() -> Element<'a, Message> {
    container(text(""))
        .style(styles::container_secondary)
        .height(1)
        .width(Length::Fill)
        .into()
}

/// Empty state component
pub fn empty_state<'a>(
    title_text: &str,
    description: &str,
    action_text: Option<&str>,
    action_message: Option<Message>,
) -> Element<'a, Message> {
    let mut content = vec![
        spacer(SpacerSize::Large),
        title(title_text).into(),
        secondary_text(description).into(),
    ];

    if let (Some(text), Some(message)) = (action_text, action_message) {
        content.push(spacer(SpacerSize::Medium));
        content.push(primary_button(text, message).into());
    }

    content.push(spacer(SpacerSize::Large));

    main_container(
        spaced_column(content)
            .align_items(Alignment::Center)
            .into(),
    )
    .into()
}

/// Stato di caricamento generico con animazione migliorata
pub fn loading_state(message: &str) -> Element<Message> {
    main_container(
        spaced_column(vec![
            spacer(SpacerSize::Large),
            container(
                column![
                    container(
                        text("⚡")
                            .size(64)
                            .style(styles::colors::PRIMARY)
                    )
                    .padding(20),
                    text("AirWin")
                        .size(32)
                        .style(styles::colors::TEXT_PRIMARY),
                    Space::with_height(20),
                    text(message)
                        .size(16)
                        .style(styles::colors::TEXT_SECONDARY),
                    Space::with_height(30),
                    // Progress indicator
                    container(
                        Space::with_width(200)
                    )
                    .height(4)
                    .style(|_theme: &Theme| {
                        container::Appearance {
                            background: Some(Background::Color(iced::Color::from_rgba(
                                styles::colors::PRIMARY.r,
                                styles::colors::PRIMARY.g,
                                styles::colors::PRIMARY.b,
                                0.3
                            ))),
                            border: Border {
                                radius: 2.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    }),
                ]
                .align_items(Alignment::Center)
                .spacing(styles::spacing::SMALL)
            )
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| {
                container::Appearance {
                    background: Some(Background::Color(styles::colors::BACKGROUND)),
                    ..Default::default()
                }
            })
            .into()
        ])
        .into()
    )
    .into()
}
