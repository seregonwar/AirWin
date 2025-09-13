use std::sync::Arc;
use tokio::sync::Mutex;

// Expose crate modules
mod network;
mod protocols;
mod ui;
mod utils;

use network::discovery::DeviceDiscovery;
use network::ble::BleManager;
use protocols::airdrop::AirDrop;
use protocols::airplay::AirPlay;
use protocols::awdl::{AwdlManager, AwdlManagerConfig};

/// Struttura principale dell'applicazione AirWin
pub struct AirWinServices {
    pub device_discovery: Arc<Mutex<DeviceDiscovery>>,
    pub airdrop: Arc<Mutex<AirDrop>>,
    pub airplay: Arc<Mutex<AirPlay>>,
    pub ble: Arc<Mutex<BleManager>>,
    pub awdl: Arc<Mutex<AwdlManager>>,
}

impl AirWinServices {
    /// Crea una nuova istanza dei servizi AirWin
    pub async fn new() -> anyhow::Result<Self> {
        // Construct services with correct constructors
        let discovery = DeviceDiscovery::new()?;
        let airdrop = AirDrop::new();
        let airplay = AirPlay::new();
        let ble = BleManager::new().await?;
        let awdl = AwdlManager::new(AwdlManagerConfig::default());

        Ok(Self {
            device_discovery: Arc::new(Mutex::new(discovery)),
            airdrop: Arc::new(Mutex::new(airdrop)),
            airplay: Arc::new(Mutex::new(airplay)),
            ble: Arc::new(Mutex::new(ble)),
            awdl: Arc::new(Mutex::new(awdl)),
        })
    }
    
    /// Inizializza tutti i servizi
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Inizializza device discovery (mDNS)
        {
            let discovery = self.device_discovery.lock().await;
            discovery.start_discovery().await?;
        }

        // Avvia AirDrop HTTPS server e servizi mDNS
        {
            let airdrop = self.airdrop.lock().await;
            airdrop.start_server().await?;
        }

        // Avvia server AirPlay per ricezione
        {
            let airplay = self.airplay.lock().await;
            airplay.start_server().await?;
        }

        // Inizializza BLE
        {
            let mut ble = self.ble.lock().await;
            ble.initialize().await?;
        }

        // Inizializza e avvia AWDL
        {
            let mut awdl = self.awdl.lock().await;
            awdl.initialize().await?;
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inizializza il logger
    env_logger::init();
    
    // Crea un runtime separato per i servizi di background
    let runtime = tokio::runtime::Runtime::new()?;
    
    // Crea i servizi AirWin nel runtime
    let services = runtime.block_on(async {
        match AirWinServices::new().await {
            Ok(s) => Arc::new(s),
            Err(e) => {
                eprintln!("Errore nella creazione dei servizi: {}", e);
                std::process::exit(1);
            }
        }
    });
    
    // Inizializza i servizi in background
    let services_clone = services.clone();
    runtime.spawn(async move {
        if let Err(e) = services_clone.initialize().await {
            eprintln!("Errore nell'inizializzazione dei servizi: {}", e);
            // Non terminiamo l'app, continuiamo con funzionalità limitate
        }
    });
    
    // Mantieni il runtime attivo in un thread separato
    std::thread::spawn(move || {
        runtime.block_on(async {
            // Mantieni il runtime attivo
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });
    });
    
    // Avvia l'interfaccia utente Iced nel thread principale
    // Iced gestisce il proprio event loop, quindi non serve async qui
    ui::run()?;
    
    Ok(())
}