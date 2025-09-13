use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::runtime::Handle; 
use tracing::{info, warn, debug};
use uuid::Uuid;
use std::time::Duration;
use rand::Rng;
use sha2::{Sha256, Digest};

// Apple AirDrop BLE Service UUIDs
const AIRDROP_SERVICE_UUID: &str = "7ba94d80-ca9b-4d8d-b1db-21e8a4e6b256";

// Apple Continuity Service UUID (used for device identification)
const CONTINUITY_SERVICE_UUID: &str = "d0611e78-bbb4-4591-a5f8-487910ae4366";

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct BleDevice {
    pub id: String,
    pub name: String,
    pub rssi: i16,
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
    pub service_data: HashMap<Uuid, Vec<u8>>,
    pub last_seen: std::time::Instant,
}

pub struct BleManager {
    manager: Manager,
    adapter: Option<Adapter>,
    discovered_devices: Arc<Mutex<HashMap<String, BleDevice>>>,
    is_scanning: Arc<Mutex<bool>>,
    is_advertising: Arc<Mutex<bool>>,
}

impl BleManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing BLE Manager for AirDrop discovery");
        
        let manager = Manager::new().await?;
        
        Ok(Self {
            manager,
            adapter: None,
            discovered_devices: Arc::new(Mutex::new(HashMap::new())),
            is_scanning: Arc::new(Mutex::new(false)),
            is_advertising: Arc::new(Mutex::new(false)),
        })
    }

    pub async fn initialize(&mut self) -> Result<()> {
        info!("Getting BLE adapters...");
        let adapters = self.manager.adapters().await?;
        
        if adapters.is_empty() {
            return Err(anyhow!("No BLE adapters found"));
        }

        // Use the first available adapter
        let adapter = adapters.into_iter().next().unwrap();
        let info = adapter.adapter_info().await?;
        info!("Using BLE adapter: {}", info);
        
        self.adapter = Some(adapter);
        Ok(())
    }

    // Generate Apple-compatible device hash for AirDrop
    #[allow(dead_code)]
    fn generate_device_hash() -> String {
        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 6] = rng.gen();
        
        let mut hasher = Sha256::new();
        hasher.update(&random_bytes);
        hasher.update(b"AirWin");
        
        let result = hasher.finalize();
        hex::encode(&result[..6])
    }

    // Create Apple-compatible manufacturer data for AirDrop
    #[allow(dead_code)]
    fn create_airdrop_manufacturer_data() -> Vec<u8> {
        let mut data = Vec::new();
        
        // Apple Company ID (0x004C)
        data.extend_from_slice(&[0x4C, 0x00]);
        
        // AirDrop Advertisement Type (0x05)
        data.push(0x05);
        
        // Flags (discoverable, available)
        data.push(0x01);
        
        // Device hash (6 bytes)
        let hash = Self::generate_device_hash();
        let hash_bytes = hex::decode(&hash).unwrap_or_else(|_| vec![0; 6]);
        data.extend_from_slice(&hash_bytes[..6]);
        
        // Additional Apple-specific data
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Reserved
        
        data
    }

    pub async fn start_scanning(&self) -> Result<()> {
        let adapter = self.adapter.as_ref()
            .ok_or_else(|| anyhow!("BLE adapter not initialized"))?;

        let mut is_scanning = self.is_scanning.lock().await;
        if *is_scanning {
            return Ok(());
        }

        info!("Starting BLE scan for AirDrop devices...");

        // Create scan filter for AirDrop services
        let scan_filter = ScanFilter {
            services: vec![
                Uuid::parse_str(AIRDROP_SERVICE_UUID)?,
                Uuid::parse_str(CONTINUITY_SERVICE_UUID)?,
            ],
        };

        adapter.start_scan(scan_filter).await?;
        *is_scanning = true;

        // Start device discovery loop
        let adapter_clone = adapter.clone();
        let devices = self.discovered_devices.clone();
        let scanning_flag = self.is_scanning.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            
            while *scanning_flag.lock().await {
                interval.tick().await;
                
                match adapter_clone.peripherals().await {
                    Ok(peripherals) => {
                        for peripheral in peripherals {
                            if let Ok(properties) = peripheral.properties().await {
                                if let Some(props) = properties {
                                    let device_id = peripheral.id().to_string();
                                    
                                    // Check if this looks like an AirDrop device
                                    let is_airdrop_device = props.manufacturer_data
                                        .get(&0x004C) // Apple Company ID
                                        .map(|data| data.len() >= 3 && data[2] == 0x05) // AirDrop type
                                        .unwrap_or(false);

                                    if is_airdrop_device || 
                                       props.services.contains(&Uuid::parse_str(AIRDROP_SERVICE_UUID).unwrap()) {
                                        
                                        let local_name = props
                                        .local_name
                                        .clone()
                                        .unwrap_or_else(|| "Unknown AirDrop Device".to_string());

                                    let device = BleDevice {
                                        id: device_id.clone(),
                                        name: local_name.clone(),
                                        rssi: props.rssi.unwrap_or(0),
                                        manufacturer_data: props.manufacturer_data,
                                        service_data: props.service_data,
                                        last_seen: std::time::Instant::now(),
                                    };

                                        let mut devices_lock = devices.lock().await;
                                        devices_lock.insert(device_id, device);
                                            
                                    debug!("Discovered AirDrop BLE device: {}", local_name);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error getting BLE peripherals: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn stop_scanning(&self) -> Result<()> {
        let adapter = self.adapter.as_ref()
            .ok_or_else(|| anyhow!("BLE adapter not initialized"))?;

        let mut is_scanning = self.is_scanning.lock().await;
        if !*is_scanning {
            return Ok(());
        }

        info!("Stopping BLE scan...");
        adapter.stop_scan().await?;
        *is_scanning = false;

        Ok(())
    }

    pub async fn start_advertising(&self) -> Result<()> {
        let mut is_advertising = self.is_advertising.lock().await;
        if *is_advertising {
            return Ok(());
        }

        info!("Starting BLE advertising for AirDrop discovery...");
        
        // Note: btleplug doesn't support advertising on Windows directly
        // This would need platform-specific Windows BLE advertising APIs
        // For now, we'll mark as advertising but actual implementation
        // would require Windows.Devices.Bluetooth.Advertisement APIs
        
        warn!("BLE advertising not fully implemented on Windows - requires platform-specific APIs");
        *is_advertising = true;

        Ok(())
    }

    pub async fn stop_advertising(&self) -> Result<()> {
        let mut is_advertising = self.is_advertising.lock().await;
        if !*is_advertising {
            return Ok(());
        }

        info!("Stopping BLE advertising...");
        *is_advertising = false;

        Ok(())
    }

    pub async fn get_discovered_devices(&self) -> Vec<BleDevice> {
        let devices = self.discovered_devices.lock().await;
        let now = std::time::Instant::now();
        
        // Filter out devices not seen in the last 30 seconds
        devices.values()
            .filter(|device| now.duration_since(device.last_seen).as_secs() < 30)
            .cloned()
            .collect()
    }

    pub async fn is_scanning(&self) -> bool {
        *self.is_scanning.lock().await
    }

    pub async fn is_advertising(&self) -> bool {
        *self.is_advertising.lock().await
    }
}

impl Drop for BleManager {
    fn drop(&mut self) {
        // Do NOT create or block a new runtime here. This object is often
        // dropped from within an existing Tokio runtime worker thread, and
        // calling Runtime::new().block_on(...) would panic with:
        // "Cannot start a runtime from within a runtime".
        if let Some(adapter) = self.adapter.clone() {
            let scanning = self.is_scanning.clone();
            let advertising = self.is_advertising.clone();

            if let Ok(handle) = Handle::try_current() {
                // Best-effort async cleanup on the existing runtime.
                handle.spawn(async move {
                    // Stop scanning if it is still running
                    if *scanning.lock().await {
                        let _ = adapter.stop_scan().await;
                        let mut s = scanning.lock().await;
                        *s = false;
                    }

                    // Advertising isn't implemented on Windows in this layer,
                    // but ensure the flag is cleared to keep internal state consistent.
                    let mut adv = advertising.lock().await;
                    *adv = false;
                });
            } else {
                // No runtime available; skip cleanup to avoid panics.
                // OS / driver will clean up scanning resources when the process exits.
            }
        }
    }
}
