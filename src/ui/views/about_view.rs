//! Vista About dell'applicazione AirWin
//!
//! Questa vista mostra informazioni sull'applicazione, crediti,
//! licenze e collegamenti utili.

use iced::{
    widget::{
        button, column, container, row, scrollable, text, Space,
        horizontal_rule,
    },
    Alignment, Element, Length,
};

use crate::ui::Theme;

use crate::ui::{
    messages::Message,
    styles,
};
 
/// Struttura per la vista About
#[derive(Debug, Clone)]
pub struct AboutView {
    app_version: String,
    build_date: String,
    commit_hash: Option<String>,
} 
 
impl AboutView {
    /// Crea una nuova istanza della vista About
    pub fn new(
        app_version: String, 
        build_date: String,
        commit_hash: Option<String>,
    ) -> Self {
        Self {
            app_version,
            build_date,
            commit_hash,
        }
    }

    /// Renderizza la vista About
    pub fn view(&self, theme: &Theme) -> Element<Message> {
        let header = row![
            button(
                text("← Indietro")
                    .size(14)
            )
            .on_press(Message::ShowMainView)
            .style(iced::theme::Button::Secondary),
            
            Space::with_width(styles::spacing::MEDIUM),
            
            text("Informazioni")
                .size(24)
                .style(styles::colors::TEXT_PRIMARY),
        ]
        .align_items(Alignment::Center)
        .padding(styles::spacing::MEDIUM.0);

        let content = scrollable(
            column![
                // Logo e titolo principale
                self.app_header(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Informazioni versione
                self.version_info(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Descrizione
                self.description(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Funzionalità
                self.features(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Crediti
                self.credits(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Licenze
                self.licenses(theme),
                
                Space::with_height(styles::spacing::LARGE),
                
                // Collegamenti
                self.links(theme),
                
                Space::with_height(styles::spacing::XLARGE),
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



    /// Header dell'applicazione con logo
    fn app_header(&self, _theme: &Theme) -> Element<Message> {
        container(
            column![
                // Logo (emoji come placeholder)
                text("📱")
                    .size(64)
                    .style(styles::colors::TEXT_PRIMARY),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                // Nome applicazione
                text("AirWin")
                    .size(32)
                    .style(styles::colors::TEXT_PRIMARY),
                
                // Sottotitolo
                text("Condivisione wireless per Windows")
                    .size(16)
                    .style(styles::colors::TEXT_MUTED),
            ]
            .align_items(Alignment::Center)
            .spacing(styles::spacing::SMALL)
        )
        .center_x()
        .width(Length::Fill)
        .into()
    }

    /// Informazioni sulla versione
    fn version_info(&self, _theme: &Theme) -> Element<Message> {
        let version_items = column![
            row![
                text("Versione:")
                    .size(14)
                    .style(styles::colors::TEXT_PRIMARY)
                    .width(Length::FillPortion(1)),
                
                text(self.app_version.clone())
                    .size(14)
                    .style(styles::colors::TEXT_MUTED)
                    .width(Length::FillPortion(2)),
            ]
            .align_items(Alignment::Center),
            
            row![
                text("Build:")
                    .size(14)
                    .style(styles::colors::TEXT_PRIMARY)
                    .width(Length::FillPortion(1)),
                
                text(self.build_date.clone())
                    .size(14)
                    .style(styles::colors::TEXT_MUTED)
                    .width(Length::FillPortion(2)),
            ]
            .align_items(Alignment::Center),
        ]
        .spacing(styles::spacing::SMALL);

        let version_with_commit = if let Some(commit) = &self.commit_hash {
            column![
                version_items,
                
                row![
                    text("Commit:")
                        .size(14)
                        .style(styles::colors::TEXT_PRIMARY)
                        .width(Length::FillPortion(1)),
                    
                    text(commit.clone())
                        .size(14)
                        .style(styles::colors::TEXT_MUTED)
                        .width(Length::FillPortion(2)),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(styles::spacing::SMALL)
        } else {
            version_items
        };

        container(
            column![
                text("Versione")
                    .size(18)
                    .style(styles::colors::TEXT_SECONDARY),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                version_with_commit,
            ]
        )
        .style(styles::container_secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Descrizione dell'applicazione
    fn description(&self, _theme: &Theme) -> Element<Message> {
        container(
            column![
                text("Descrizione")
                    .size(18)
                    .style(styles::colors::TEXT_SECONDARY),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                text("AirWin è un'applicazione che porta le funzionalità di AirDrop e AirPlay di Apple su Windows. Permette di condividere file, link e contenuti multimediali tra dispositivi Apple e Windows in modo semplice e intuitivo.")
                    .size(14)
                    .style(styles::colors::TEXT_PRIMARY),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                text("L'applicazione utilizza i protocolli di rete standard per garantire compatibilità e sicurezza nelle comunicazioni wireless.")
                    .size(14)
                    .style(styles::colors::TEXT_PRIMARY),
            ]
        )
        .style(styles::container_secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Funzionalità principali
    fn features(&self, theme: &Theme) -> Element<Message> {
        let features_list = column![
            (&self).feature_item("📁", "Condivisione File", "Invia e ricevi file tramite AirDrop", theme),
            (&self).feature_item("🔗", "Condivisione Link", "Condividi URL e collegamenti web", theme),
            (&self).feature_item("📺", "Streaming AirPlay", "Trasmetti contenuti multimediali", theme),
            (&self).feature_item("🔍", "Scoperta Automatica", "Trova dispositivi compatibili automaticamente", theme),
            (&self).feature_item("🔒", "Sicurezza", "Comunicazioni crittografate e sicure", theme),
            (&self).feature_item("⚡", "Prestazioni", "Trasferimenti veloci e affidabili", theme),
        ]
        .spacing(styles::spacing::MEDIUM);
  
        container(
            column![
                text("Funzionalità")
                    .size(18)
                    .style(styles::colors::TEXT_SECONDARY),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                features_list,
            ]
        )
        .style(styles::container_secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Singola funzionalità
    fn feature_item(
        &self,
        icon: &str,
        title: &str,
        description: &str,
        _theme: &Theme,
    ) -> Element<Message> {
        row![
            text(icon)
                .size(20)
                .width(Length::Fixed(40.0)),
            
            column![
                text(title)
                    .size(14)
                    .style(styles::colors::TEXT_PRIMARY),
                
                text(description)
                    .size(12)
                    .style(styles::colors::TEXT_MUTED),
            ]
            .spacing(iced::Pixels(styles::spacing::SMALL.0 / 2.0)),
        ]
        .align_items(Alignment::Center)
        .spacing(styles::spacing::MEDIUM)
        .into()
    }

    /// Crediti e riconoscimenti
    fn credits(&self, _theme: &Theme) -> Element<Message> {
        container(
            column![
                text("Crediti")
                    .size(18)
                    .style(styles::colors::TEXT_SECONDARY),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                text("Sviluppato con ❤️ utilizzando:")
                    .size(14)
                    .style(styles::colors::TEXT_PRIMARY),
                
                Space::with_height(styles::spacing::SMALL),
                
                column![
                    text("• Rust - Linguaggio di programmazione")
                        .size(12)
                        .style(styles::colors::TEXT_MUTED),
                    
                    text("• Iced - Framework per interfacce grafiche")
                        .size(12)
                        .style(styles::colors::TEXT_MUTED),
                    
                    text("• Tokio - Runtime asincrono")
                        .size(12)
                        .style(styles::colors::TEXT_MUTED),
                    
                    text("• mDNS-SD - Scoperta servizi di rete")
                        .size(12)
                        .style(styles::colors::TEXT_MUTED),
                ]
                .spacing(iced::Pixels(styles::spacing::SMALL.0 / 2.0)),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                text("Ringraziamenti speciali alla comunità open source per i contributi e il supporto.")
                    .size(12)
                    .style(styles::colors::TEXT_MUTED),
            ]
        )
        .style(styles::container_secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Informazioni sulle licenze
    fn licenses(&self, _theme: &Theme) -> Element<Message> {
        container(
            column![
                text("Licenze")
                    .size(18)
                    .style(styles::colors::TEXT_SECONDARY),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                text("AirWin è distribuito sotto licenza MIT.")
                    .size(14)
                    .style(styles::colors::TEXT_PRIMARY),
                
                Space::with_height(styles::spacing::SMALL),
                
                text("Questo software utilizza librerie di terze parti, ciascuna con la propria licenza. Per informazioni dettagliate, consulta il file LICENSE nel repository del progetto.")
                    .size(12)
                    .style(styles::colors::TEXT_MUTED),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                button(
                    text("📄 Visualizza Licenze")
                        .size(14)
                )
                .on_press(Message::OpenLicenses)
                .style(iced::theme::Button::Secondary),
            ]
        )
        .style(styles::container_secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }

    /// Collegamenti utili
    fn links(&self, _theme: &Theme) -> Element<Message> {
        container(
            column![
                text("Collegamenti")
                    .size(18)
                    .style(styles::colors::TEXT_SECONDARY),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                row![
                    button(
                        text("🌐 Sito Web")
                            .size(14)
                    )
                    .on_press(Message::OpenWebsite)
                    .style(iced::theme::Button::Secondary),
                    
                    button(
                        text("📚 Documentazione")
                            .size(14)
                    )
                    .on_press(Message::OpenDocumentation)
                    .style(iced::theme::Button::Secondary),
                ]
                .spacing(styles::spacing::MEDIUM),
                
                row![
                    button(
                        text("🐛 Segnala Bug")
                            .size(14)
                    )
                    .on_press(Message::OpenIssues)
                    .style(iced::theme::Button::Secondary),
                    
                    button(
                        text("💡 Richiedi Funzionalità")
                            .size(14)
                    )
                    .on_press(Message::OpenFeatureRequest)
                    .style(iced::theme::Button::Secondary),
                ]
                .spacing(styles::spacing::MEDIUM),
                
                Space::with_height(styles::spacing::MEDIUM),
                
                text("Per supporto e assistenza, visita il nostro repository GitHub o contatta il team di sviluppo.")
                    .size(12)
                    .style(styles::colors::TEXT_MUTED),
            ]
        )
        .style(styles::container_secondary)
        .padding(styles::spacing::MEDIUM.0)
        .width(Length::Fill)
        .into()
    }
}