//! Widget personalizzati per l'interfaccia utente AirWin
//!
//! Questo modulo contiene widget personalizzati e riutilizzabili
//! per creare un'esperienza utente coerente e moderna.

use iced::{
    widget::{
        button, column, container, row, text, Space, progress_bar,
        horizontal_rule, vertical_rule,
    },
    Alignment, Element, Length, Background, Color, Border, Shadow, Pixels,
    Theme as IcedTheme,
};

use crate::ui::messages::Message;
use crate::ui::styles;

/// Widget per visualizzare lo stato di connessione
pub fn connection_status<'a>(
    is_connected: bool,
    device_name: Option<&str>,
    _theme: &IcedTheme,
) -> Element<'a, Message> {
    let (status_text, status_color) = if is_connected {
        ("Connesso", styles::colors::SUCCESS)
    } else {
        ("Disconnesso", styles::colors::ERROR)
    };

    let status_indicator = container(
        text("●")
            .size(12)
            .style(status_color)
    )
    .width(Length::Fixed(20.0))
    .center_x();

    let status_content = if let Some(name) = device_name {
        column![
            text(status_text)
                .size(12)
                .style(Color::BLACK),
            text(name)
                .size(10)
                .style(Color::from_rgb(0.5, 0.5, 0.5)),
        ]
        .spacing(2)
    } else {
        column![
            text(status_text)
                .size(12)
                .style(Color::BLACK),
        ]
    };

    row![
        status_indicator,
        status_content,
    ]
    .align_items(Alignment::Center)
    .spacing(styles::spacing::SMALL)
    .into()
}

/// Widget per visualizzare il progresso di trasferimento
pub fn transfer_progress<'a>(
    progress: f32,
    file_name: &str,
    transfer_speed: Option<&str>,
    _theme: &IcedTheme,
) -> Element<'a, Message> {
    let progress_bar = progress_bar(0.0..=100.0, progress)
        .style(move |theme: &IcedTheme| progress_bar::Appearance {
            background: Background::Color(if theme == &IcedTheme::Dark {
                Color::from_rgb(0.2, 0.2, 0.2)
            } else {
                Color::from_rgb(0.9, 0.9, 0.9)
            }),
            bar: Background::Color(Color::from_rgb(0.2, 0.6, 1.0)),
            border_radius: 4.0.into(),
        })
        .height(Length::Fixed(8.0));

    let progress_text = text(format!("{:.1}%", progress))
        .size(12)
        .style(Color::from_rgb(0.5, 0.5, 0.5));

    let file_info = row![
        text(file_name)
            .size(14)
            .style(Color::BLACK),
        Space::with_width(Length::Fill),
        progress_text,
    ]
    .align_items(Alignment::Center);

    let speed_info = if let Some(speed) = transfer_speed {
        Some(
            text(speed)
                .size(10)
                .style(styles::colors::TEXT_MUTED)
        )
    } else {
        None
    };

    let mut content = column![
        file_info,
        Space::with_height(styles::spacing::SMALL),
        progress_bar,
    ]
    .spacing(0);

    if let Some(speed) = speed_info {
        content = content.push(Space::with_height(Pixels(styles::spacing::SMALL.0 / 2.0)));
        content = content.push(speed);
    }

    container(content)
        .padding(styles::spacing::MEDIUM.0)
        .style(move |theme: &IcedTheme| container::Appearance {
            background: Some(Background::Color(if theme == &IcedTheme::Dark {
                Color::from_rgb(0.15, 0.15, 0.15)
            } else {
                Color::from_rgb(0.98, 0.98, 0.98)
            })),
            border: Border::with_radius(8.0),
            shadow: Shadow::default(),
            text_color: None,
        })
        .width(Length::Fill)
        .into()
}

/// Widget per visualizzare le statistiche di rete
pub fn network_stats<'a>(
    upload_speed: &str,
    download_speed: &str,
    connected_devices: usize,
    _theme: &IcedTheme,
) -> Element<'a, Message> {
    let stat_item = |label: &str, value: &str| -> Element<'a, Message> {
        column![
            text(value)
                .size(16)
                .style(Color::BLACK),
            text(label)
                .size(10)
                .style(Color::from_rgb(0.5, 0.5, 0.5)),
        ]
        .align_items(Alignment::Center)
        .spacing(2)
        .into()
    };

    let stats = row![
        stat_item("Upload", upload_speed),
        vertical_rule(1),
        stat_item("Download", download_speed),
        vertical_rule(1),
        stat_item("Dispositivi", &connected_devices.to_string()),
    ]
    .align_items(Alignment::Center)
    .spacing(styles::spacing::MEDIUM);

    container(stats)
        .padding(styles::spacing::MEDIUM.0)
        .style(move |theme: &IcedTheme| container::Appearance {
            background: Some(Background::Color(if theme == &IcedTheme::Dark {
                Color::from_rgb(0.15, 0.15, 0.15)
            } else {
                Color::from_rgb(0.98, 0.98, 0.98)
            })),
            border: Border::with_radius(8.0),
            shadow: Shadow::default(),
            text_color: None,
        })
        .width(Length::Fill)
        .into()
}

/// Widget per visualizzare un badge di stato
pub fn status_badge<'a>(
    text_content: &str,
    badge_type: BadgeType,
    _theme: &IcedTheme,
) -> Element<'a, Message> {
    let (bg_color, text_color) = match badge_type {
        BadgeType::Success => (styles::colors::SUCCESS, Color::WHITE),
        BadgeType::Warning => (styles::colors::WARNING, Color::BLACK),
        BadgeType::Error => (styles::colors::ERROR, Color::WHITE),
        BadgeType::Info => (styles::colors::INFO, Color::WHITE),
        BadgeType::Neutral => (styles::colors::SURFACE, styles::colors::TEXT_PRIMARY),
    };

    container(
        text(text_content)
            .size(10)
            .style(text_color)
    )
    .padding([2, 6])
    .style(move |_: &IcedTheme| container::Appearance {
        background: Some(Background::Color(bg_color)),
        border: Border::with_radius(12.0),
        shadow: Shadow::default(),
        text_color: None,
    })
    .into()
}

/// Tipi di badge disponibili
#[derive(Debug, Clone, Copy)]
pub enum BadgeType {
    Success,
    Warning,
    Error,
    Info,
    Neutral,
}

/// Widget per visualizzare un separatore con testo
pub fn text_separator<'a>(
    text_content: &str,
    _theme: &IcedTheme,
) -> Element<'a, Message> {
    row![
        horizontal_rule(1),
        
        container(
            text(text_content)
                .size(12)
                .style(styles::colors::TEXT_MUTED)
        )
        .padding([0, styles::spacing::MEDIUM.0 as u16]),
        
        horizontal_rule(1),
    ]
    .align_items(Alignment::Center)
    .into()
}

/// Widget per visualizzare un tooltip informativo
pub fn info_tooltip<'a>(
    content: Element<'a, Message>,
    _tooltip_text: &str,
    _theme: &IcedTheme,
) -> Element<'a, Message> {
    // Per ora restituiamo solo il contenuto, in futuro si può implementare un vero tooltip
    content
}

/// Widget per creare un layout a griglia responsive
pub fn responsive_grid<'a>(
    items: Vec<Element<'a, Message>>,
    columns: usize,
) -> Element<'a, Message> {
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let len = items.len();
    
    for (index, item) in items.into_iter().enumerate() {
        current_row.push(item);
        
        if current_row.len() == columns || index == len - 1 {
            let row_element = row(current_row)
                .spacing(styles::spacing::MEDIUM)
                .align_items(Alignment::Start);
            rows.push(row_element.into());
            current_row = Vec::new();
        }
    }
    
    column(rows)
        .spacing(styles::spacing::MEDIUM)
        .into()
}

/// Widget per creare un header di sezione
pub fn section_header<'a>(
    title: &str,
    subtitle: Option<&str>,
    action_button: Option<Element<'a, Message>>,
    _theme: &IcedTheme,
) -> Element<'a, Message> {
    let title_text = text(title)
        .size(18)
        .style(styles::colors::TEXT_PRIMARY);

    let mut header_content = column![title_text];

    if let Some(sub) = subtitle {
        let subtitle_text = text(sub)
            .size(12)
            .style(styles::colors::TEXT_MUTED);
        header_content = header_content.push(subtitle_text);
    }

    let mut header_row = row![header_content].align_items(Alignment::Center);

    if let Some(button) = action_button {
        header_row = header_row.push(Space::with_width(Length::Fill));
        header_row = header_row.push(button);
    }

    container(header_row)
        .padding([0, 0, styles::spacing::MEDIUM.0 as u16, 0])
        .width(Length::Fill)
        .into()
}

/// Widget per creare un pannello collassabile
pub fn collapsible_panel<'a>(
    title: &str,
    is_expanded: bool,
    content: Element<'a, Message>,
    on_toggle: Message,
    _theme: &IcedTheme,
) -> Element<'a, Message> {
    let toggle_icon = if is_expanded { "▼" } else { "▶" };
    
    let header = button(
        row![
            text(toggle_icon)
                .size(12)
                .style(styles::colors::TEXT_MUTED),
            
            Space::with_width(styles::spacing::SMALL),
            
            text(title)
                .size(14)
                .style(styles::colors::TEXT_PRIMARY),
        ]
        .align_items(Alignment::Center)
    )
    .on_press(on_toggle)
    .width(Length::Fill);

    let mut panel = column![header];

    if is_expanded {
        panel = panel.push(Space::with_height(styles::spacing::SMALL));
        panel = panel.push(content);
    }

    container(panel)
        .style(move |theme: &IcedTheme| container::Appearance {
            background: Some(Background::Color(if theme == &IcedTheme::Dark {
                Color::from_rgb(0.15, 0.15, 0.15)
            } else {
                Color::from_rgb(0.98, 0.98, 0.98)
            })),
            border: Border::with_radius(8.0),
            shadow: Shadow::default(),
            text_color: None,
        })
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
}