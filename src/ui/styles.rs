//! Stili e temi per l'interfaccia Iced
//!
//! Questo modulo definisce tutti gli stili, colori e dimensioni utilizzati
//! nell'interfaccia utente per garantire coerenza e un design moderno.

use iced::{
    widget::{
        button::{Appearance as ButtonAppearance},
        container::{Appearance as ContainerAppearance},
        progress_bar::{Appearance as ProgressBarAppearance},
        text::{Appearance as TextAppearance},
    },
    Background, Border, Color, Shadow, Vector, border::Radius,
};

use iced::Theme;

/// Stato visuale per i pulsanti (locale a questo modulo)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStatus {
    Active,
    Hovered,
    Pressed,
    Disabled,
}

/// Dimensioni dei font
pub mod font_size {
    pub const TINY: u16 = 10;
    pub const SMALL: u16 = 12;
    pub const MEDIUM: u16 = 14;
    pub const LARGE: u16 = 18;
    pub const XLARGE: u16 = 24;
}

/// Spaziature standard
pub mod spacing {
    use iced::Pixels;
    
    pub const TINY: Pixels = Pixels(4.0);
    pub const SMALL: Pixels = Pixels(8.0);
    pub const MEDIUM: Pixels = Pixels(16.0);
    pub const LARGE: Pixels = Pixels(24.0);
    pub const XLARGE: Pixels = Pixels(32.0);
}

/// Raggi di curvatura
pub mod radius {
    // Costanti numeriche: converti con `Radius::from(...)` nei punti d'uso
    pub const SMALL: f32 = 4.0;
    pub const MEDIUM: f32 = 8.0;
    pub const LARGE: f32 = 12.0;
    pub const XLARGE: f32 = 16.0;
}

/// Palette colori per il tema scuro
pub mod colors {
    use iced::Color;
    
    // Colori di base
    pub const BACKGROUND: Color = Color::from_rgb(0.07, 0.08, 0.09); // #121314
    pub const SURFACE: Color = Color::from_rgb(0.12, 0.14, 0.16); // #1F242A
    pub const SURFACE_VARIANT: Color = Color::from_rgb(0.16, 0.18, 0.20); // #292E34
    
    // Colori del testo
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.95); // #F2F2F2
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.75, 0.75, 0.75); // #BFBFBF
    pub const TEXT_MUTED: Color = Color::from_rgb(0.55, 0.55, 0.55); // #8C8C8C
    pub const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0); // #FFFFFF
    
    // Colori di accento
    pub const PRIMARY: Color = Color::from_rgb(0.31, 0.53, 0.91); // #4F85EB
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.23, 0.42, 0.76); // #3B6BC1
    pub const PRIMARY_ACTIVE: Color = Color::from_rgb(0.18, 0.33, 0.61); // #2E549C
    
    // Colori di stato
    pub const SUCCESS: Color = Color::from_rgb(0.18, 0.80, 0.44); // #2ECC71
    pub const WARNING: Color = Color::from_rgb(0.95, 0.61, 0.07); // #F39C12
    pub const ERROR: Color = Color::from_rgb(0.91, 0.30, 0.24); // #E74C3C
    pub const INFO: Color = Color::from_rgb(0.20, 0.67, 0.86); // #3498DB
    
    // Colori per i bordi
    pub const BORDER: Color = Color::from_rgb(0.25, 0.27, 0.30); // #404548
    pub const BORDER_FOCUS: Color = PRIMARY;
    pub const BORDER_ERROR: Color = ERROR;
    
    // Colori per le ombre
    pub const SHADOW: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.2);
    pub const SHADOW_STRONG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.4);
}

/// Stili per il testo
pub fn text_primary(_theme: &Theme) -> TextAppearance {
    TextAppearance {
        color: Some(colors::TEXT_PRIMARY),
    }
}

pub fn text_secondary(_theme: &Theme) -> TextAppearance {
    TextAppearance {
        color: Some(colors::TEXT_SECONDARY),
    }
}

pub fn text_muted(_theme: &Theme) -> TextAppearance {
    TextAppearance {
        color: Some(colors::TEXT_MUTED),
    }
}

pub fn text_accent(_theme: &Theme) -> TextAppearance {
    TextAppearance {
        color: Some(colors::PRIMARY),
    }
}

pub fn text_success(_theme: &Theme) -> TextAppearance {
    TextAppearance {
        color: Some(colors::SUCCESS),
    }
}

pub fn text_warning(_theme: &Theme) -> TextAppearance {
    TextAppearance {
        color: Some(colors::WARNING),
    }
}

pub fn text_error(_theme: &Theme) -> TextAppearance {
    TextAppearance {
        color: Some(colors::ERROR),
    }
}

/// Stili per i container
pub fn container_primary(_theme: &Theme) -> ContainerAppearance {
    ContainerAppearance {
        text_color: Some(colors::TEXT_PRIMARY),
        background: Some(Background::Color(colors::BACKGROUND)),
        border: Border {
            color: colors::BORDER,
            width: 1.0,
            radius: Radius::from(radius::MEDIUM),
        },
        shadow: Shadow {
            color: colors::SHADOW,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
    }
}

pub fn container_secondary(_theme: &Theme) -> ContainerAppearance {
    ContainerAppearance {
        text_color: Some(colors::TEXT_SECONDARY),
        background: Some(Background::Color(colors::SURFACE)),
        border: Border {
            color: colors::BORDER,
            width: 1.0,
            radius: Radius::from(radius::MEDIUM),
        },
        shadow: Shadow {
            color: colors::SHADOW,
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
    }
}

pub fn container_header(_theme: &Theme) -> ContainerAppearance {
    ContainerAppearance {
        text_color: Some(colors::TEXT_PRIMARY),
        background: Some(Background::Color(colors::SURFACE_VARIANT)),
        border: Border {
            color: colors::BORDER,
            width: 0.0,
            radius: Radius::from(radius::SMALL),
        },
        shadow: Shadow {
            color: colors::SHADOW,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
    }
}

pub fn container_disabled(_theme: &Theme) -> ContainerAppearance {
    ContainerAppearance {
        text_color: Some(colors::TEXT_MUTED),
        background: Some(Background::Color(Color::from_rgba(
            colors::SURFACE.r,
            colors::SURFACE.g,
            colors::SURFACE.b,
            0.5,
        ))),
        border: Border {
            color: Color::from_rgba(
                colors::BORDER.r,
                colors::BORDER.g,
                colors::BORDER.b,
                0.5,
            ),
            width: 1.0,
            radius: Radius::from(radius::MEDIUM),
        },
        shadow: Shadow::default(),
    }
}

/// Container per notifiche
pub fn container_success(_theme: &Theme) -> ContainerAppearance {
    ContainerAppearance {
        text_color: Some(colors::TEXT_PRIMARY),
        background: Some(Background::Color(Color::from_rgba(
            colors::SUCCESS.r,
            colors::SUCCESS.g,
            colors::SUCCESS.b,
            0.1,
        ))),
        border: Border {
            color: colors::SUCCESS,
            width: 1.0,
            radius: Radius::from(radius::MEDIUM),
        },
        shadow: Shadow {
            color: colors::SHADOW,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
    }
}

pub fn container_error(_theme: &Theme) -> ContainerAppearance {
    ContainerAppearance {
        text_color: Some(colors::TEXT_PRIMARY),
        background: Some(Background::Color(Color::from_rgba(
            colors::ERROR.r,
            colors::ERROR.g,
            colors::ERROR.b,
            0.1,
        ))),
        border: Border {
            color: colors::ERROR,
            width: 1.0,
            radius: Radius::from(radius::MEDIUM),
        },
        shadow: Shadow {
            color: colors::SHADOW,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
    }
}

pub fn container_warning(_theme: &Theme) -> ContainerAppearance {
    ContainerAppearance {
        text_color: Some(colors::TEXT_PRIMARY),
        background: Some(Background::Color(Color::from_rgba(
            colors::WARNING.r,
            colors::WARNING.g,
            colors::WARNING.b,
            0.1,
        ))),
        border: Border {
            color: colors::WARNING,
            width: 1.0,
            radius: Radius::from(radius::MEDIUM),
        },
        shadow: Shadow {
            color: colors::SHADOW,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
    }
}

pub fn container_info(_theme: &Theme) -> ContainerAppearance {
    ContainerAppearance {
        text_color: Some(colors::TEXT_PRIMARY),
        background: Some(Background::Color(Color::from_rgba(
            colors::INFO.r,
            colors::INFO.g,
            colors::INFO.b,
            0.1,
        ))),
        border: Border {
            color: colors::INFO,
            width: 1.0,
            radius: Radius::from(radius::MEDIUM),
        },
        shadow: Shadow {
            color: colors::SHADOW,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
    }
}

/// Stili per i pulsanti
pub fn button_primary(_theme: &Theme, status: ButtonStatus) -> ButtonAppearance {
    match status {
        ButtonStatus::Active => ButtonAppearance {
            background: Some(Background::Color(colors::PRIMARY)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY,
                width: 1.0,
                radius: Radius::from(radius::MEDIUM),
            },
            shadow_offset: Vector::new(0.0, 2.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
        },
        ButtonStatus::Hovered => ButtonAppearance {
            background: Some(Background::Color(colors::PRIMARY_HOVER)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY_HOVER,
                width: 1.0,
                radius: Radius::from(radius::MEDIUM),
            },
            shadow_offset: Vector::new(0.0, 4.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
        },
        ButtonStatus::Pressed => ButtonAppearance {
            background: Some(Background::Color(colors::PRIMARY_ACTIVE)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY_ACTIVE,
                width: 1.0,
                radius: Radius::from(radius::MEDIUM),
            },
            shadow_offset: Vector::new(0.0, 1.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
        },
        ButtonStatus::Disabled => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::SURFACE.r,
                colors::SURFACE.g,
                colors::SURFACE.b,
                0.5,
            ))),
            text_color: colors::TEXT_MUTED,
            border: Border {
                color: Color::from_rgba(
                    colors::BORDER.r,
                    colors::BORDER.g,
                    colors::BORDER.b,
                    0.5,
                ),
                width: 1.0,
                radius: Radius::from(radius::MEDIUM),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow::default(),
        },
    }
}

pub fn button_secondary(_theme: &Theme, status: ButtonStatus) -> ButtonAppearance {
    match status {
        ButtonStatus::Active => ButtonAppearance {
            background: Some(Background::Color(colors::SURFACE)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: Radius::from(radius::MEDIUM),
            },
            shadow_offset: Vector::new(0.0, 2.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
        },
        ButtonStatus::Hovered => ButtonAppearance {
            background: Some(Background::Color(colors::SURFACE_VARIANT)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY,
                width: 1.0,
                radius: Radius::from(radius::MEDIUM),
            },
            shadow_offset: Vector::new(0.0, 4.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
        },
        ButtonStatus::Pressed => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::SURFACE_VARIANT.r,
                colors::SURFACE_VARIANT.g,
                colors::SURFACE_VARIANT.b,
                0.8,
            ))),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY_ACTIVE,
                width: 1.0,
                radius: Radius::from(radius::MEDIUM),
            },
            shadow_offset: Vector::new(0.0, 1.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
        },
        ButtonStatus::Disabled => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::SURFACE.r,
                colors::SURFACE.g,
                colors::SURFACE.b,
                0.5,
            ))),
            text_color: colors::TEXT_MUTED,
            border: Border {
                color: Color::from_rgba(
                    colors::BORDER.r,
                    colors::BORDER.g,
                    colors::BORDER.b,
                    0.5,
                ),
                width: 1.0,
                radius: Radius::from(radius::MEDIUM),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow::default(),
        },
    }
}

pub fn button_card(_theme: &Theme, status: ButtonStatus) -> ButtonAppearance {
    match status {
        ButtonStatus::Active => ButtonAppearance {
            background: Some(Background::Color(colors::SURFACE)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: Radius::from(radius::LARGE),
            },
            shadow_offset: Vector::new(0.0, 2.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
        },
        ButtonStatus::Hovered => ButtonAppearance {
            background: Some(Background::Color(colors::SURFACE_VARIANT)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY,
                width: 2.0,
                radius: Radius::from(radius::LARGE),
            },
            shadow_offset: Vector::new(0.0, 4.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
        },
        ButtonStatus::Pressed => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::SURFACE_VARIANT.r,
                colors::SURFACE_VARIANT.g,
                colors::SURFACE_VARIANT.b,
                0.8,
            ))),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY_ACTIVE,
                width: 2.0,
                radius: Radius::from(radius::LARGE),
            },
            shadow_offset: Vector::new(0.0, 1.0),
            shadow: Shadow {
                color: colors::SHADOW,
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
        },
        ButtonStatus::Disabled => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::SURFACE.r,
                colors::SURFACE.g,
                colors::SURFACE.b,
                0.3,
            ))),
            text_color: colors::TEXT_MUTED,
            border: Border {
                color: Color::from_rgba(
                    colors::BORDER.r,
                    colors::BORDER.g,
                    colors::BORDER.b,
                    0.3,
                ),
                width: 1.0,
                radius: Radius::from(radius::LARGE),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow::default(),
        },
    }
}

pub fn button_selected(_theme: &Theme, status: ButtonStatus) -> ButtonAppearance {
    match status {
        ButtonStatus::Active => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::PRIMARY.r,
                colors::PRIMARY.g,
                colors::PRIMARY.b,
                0.2,
            ))),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY,
                width: 2.0,
                radius: Radius::from(radius::LARGE),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow {
                color: Color::from_rgba(
                    colors::PRIMARY.r,
                    colors::PRIMARY.g,
                    colors::PRIMARY.b,
                    0.3,
                ),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 8.0,
            },
        },
        ButtonStatus::Hovered => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::PRIMARY.r,
                colors::PRIMARY.g,
                colors::PRIMARY.b,
                0.3,
            ))),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY_HOVER,
                width: 2.0,
                radius: Radius::from(radius::LARGE),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow {
                color: Color::from_rgba(
                    colors::PRIMARY.r,
                    colors::PRIMARY.g,
                    colors::PRIMARY.b,
                    0.4,
                ),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 12.0,
            },
        },
        ButtonStatus::Pressed => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::PRIMARY.r,
                colors::PRIMARY.g,
                colors::PRIMARY.b,
                0.4,
            ))),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::PRIMARY_ACTIVE,
                width: 2.0,
                radius: Radius::from(radius::LARGE),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow {
                color: Color::from_rgba(
                    colors::PRIMARY.r,
                    colors::PRIMARY.g,
                    colors::PRIMARY.b,
                    0.2,
                ),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 4.0,
            },
        },
        ButtonStatus::Disabled => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::SURFACE.r,
                colors::SURFACE.g,
                colors::SURFACE.b,
                0.3,
            ))),
            text_color: colors::TEXT_MUTED,
            border: Border {
                color: Color::from_rgba(
                    colors::BORDER.r,
                    colors::BORDER.g,
                    colors::BORDER.b,
                    0.3,
                ),
                width: 1.0,
                radius: Radius::from(radius::LARGE),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow::default(),
        },
    }
}

pub fn button_ghost(_theme: &Theme, status: ButtonStatus) -> ButtonAppearance {
    match status {
        ButtonStatus::Active => ButtonAppearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: colors::TEXT_SECONDARY,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(radius::SMALL),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow::default(),
        },
        ButtonStatus::Hovered => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::SURFACE.r,
                colors::SURFACE.g,
                colors::SURFACE.b,
                0.5,
            ))),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(radius::SMALL),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow::default(),
        },
        ButtonStatus::Pressed => ButtonAppearance {
            background: Some(Background::Color(Color::from_rgba(
                colors::SURFACE.r,
                colors::SURFACE.g,
                colors::SURFACE.b,
                0.7,
            ))),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(radius::SMALL),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow::default(),
        },
        ButtonStatus::Disabled => ButtonAppearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: colors::TEXT_MUTED,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(radius::SMALL),
            },
            shadow_offset: Vector::new(0.0, 0.0),
            shadow: Shadow::default(),
        },
    }
}

/// Stili per le progress bar
pub fn progress_bar_primary(_theme: &Theme) -> ProgressBarAppearance {
    ProgressBarAppearance {
        background: Background::Color(colors::SURFACE),
        bar: Background::Color(colors::PRIMARY),
        border_radius: Radius::from(radius::SMALL),
    }
}

pub fn progress_bar_success(_theme: &Theme) -> ProgressBarAppearance {
    ProgressBarAppearance {
        background: Background::Color(colors::SURFACE),
        bar: Background::Color(colors::SUCCESS),
        border_radius: Radius::from(radius::SMALL),
    }
}

pub fn progress_bar_warning(_theme: &Theme) -> ProgressBarAppearance {
    ProgressBarAppearance {
        background: Background::Color(colors::SURFACE),
        bar: Background::Color(colors::WARNING),
        border_radius: Radius::from(radius::SMALL),
    }
}

pub fn progress_bar_error(_theme: &Theme) -> ProgressBarAppearance {
    ProgressBarAppearance {
        background: Background::Color(colors::SURFACE),
        bar: Background::Color(colors::ERROR),
        border_radius: Radius::from(radius::SMALL),
    }
}
    