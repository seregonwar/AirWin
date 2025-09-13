use std::collections::HashMap;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use rand::Rng;
use anyhow::Result;

/// Apple-specific TXT record generator for AirDrop mDNS services
pub struct AppleRecords;

impl AppleRecords {
    /// Generate a proper Apple device hash for AirDrop
    pub fn generate_device_hash() -> String {
        let mut rng = rand::thread_rng();
        let random_data: [u8; 16] = rng.gen();
        
        let mut hasher = Sha256::new();
        hasher.update(&random_data);
        hasher.update(b"AirWin");
        hasher.update(&std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_le_bytes());
        
        let result = hasher.finalize();
        hex::encode(&result[..6])
    }

    /// Generate Apple-compatible computer ID
    pub fn generate_computer_id() -> String {
        Uuid::new_v4().simple().to_string().to_uppercase()
    }

    /// Generate Apple-compatible system ID  
    pub fn generate_system_id() -> String {
        Uuid::new_v4().simple().to_string().to_uppercase()
    }

    /// Generate session hash for AirDrop session
    pub fn generate_session_hash() -> String {
        let mut rng = rand::thread_rng();
        let session_data: [u8; 8] = rng.gen();
        hex::encode(session_data).to_uppercase()
    }

    /// Generate AirWin-compatible service ID (6-byte random hex string)
    pub fn generate_service_id() -> String {
        let mut rng = rand::thread_rng();
        let service_data: [u8; 6] = rng.gen();
        hex::encode(service_data).to_lowercase()
    }

    /// Create complete AirDrop TXT records compatible with Apple devices
    pub fn create_airdrop_txt_records() -> Result<HashMap<String, String>> {
        let mut properties = HashMap::new();
        
        // Core AirDrop properties - using Apple compatible flags
        // 0x3fb = SUPPORTS_URL | SUPPORTS_DVZIP | SUPPORTS_PIPELINING | SUPPORTS_MIXED_TYPES | 
        //         SUPPORTS_UNKNOWN1 | SUPPORTS_UNKNOWN2 | SUPPORTS_IRIS | SUPPORTS_DISCOVER_MAYBE | 
        //         SUPPORTS_UNKNOWN3 | SUPPORTS_ASSET_BUNDLE
        properties.insert("flags".to_string(), "1019".to_string()); // 0x3fb in decimal
        properties.insert("protocol_version".to_string(), "2".to_string());
        properties.insert("service_id".to_string(), Self::generate_service_id());
        properties.insert("service_type".to_string(), "1".to_string()); // AirDrop service
        
        // Device identification
        properties.insert("computerid".to_string(), Self::generate_computer_id());
        properties.insert("systemid".to_string(), Self::generate_system_id());
        properties.insert("machine_id".to_string(), Self::generate_computer_id());
        
        // Device info
        properties.insert("model".to_string(), "Windows,1".to_string());
        properties.insert("name".to_string(), 
            hostname::get()?.to_string_lossy().to_string());
        properties.insert("system_version".to_string(), "10.0".to_string());
        
        // Capabilities
        properties.insert("supports_url".to_string(), "1".to_string());
        properties.insert("supports_dvzip".to_string(), "1".to_string());
        properties.insert("supports_dv".to_string(), "1".to_string());
        properties.insert("supports_pipelining".to_string(), "1".to_string());
        properties.insert("supports_mixed_types".to_string(), "1".to_string());
        properties.insert("supports_contacts".to_string(), "1".to_string());
        properties.insert("supports_discover".to_string(), "1".to_string());
        properties.insert("supports_airdrop".to_string(), "1".to_string());
        properties.insert("supports_sharing".to_string(), "1".to_string());
        
        // Transport capabilities
        properties.insert("supports_awdl".to_string(), "1".to_string());
        properties.insert("supports_ble".to_string(), "1".to_string());
        properties.insert("supports_wifi_direct".to_string(), "1".to_string());
        
        // Security and privacy
        properties.insert("phash".to_string(), Self::generate_device_hash());
        properties.insert("discoverable".to_string(), "1".to_string());
        properties.insert("status_flags".to_string(), "0x1".to_string());
        
        // Session management
        properties.insert("session_id".to_string(), Self::generate_session_hash());
        properties.insert("timestamp".to_string(), 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string());
        
        Ok(properties)
    }

    /// Create Companion Link TXT records (for device pairing)
    pub fn create_companion_txt_records() -> Result<HashMap<String, String>> {
        let mut properties = HashMap::new();
        
        properties.insert("rpMRtID".to_string(), Self::generate_computer_id());
        properties.insert("rpAD".to_string(), Self::generate_device_hash());
        properties.insert("rpVr".to_string(), "350.92.4".to_string()); // Companion version
        properties.insert("rpFl".to_string(), "0x20000".to_string()); // Flags
        properties.insert("rpHA".to_string(), Self::generate_device_hash());
        properties.insert("rpHI".to_string(), Self::generate_computer_id());
        properties.insert("rpMd".to_string(), "Windows,1".to_string());
        properties.insert("rpNm".to_string(), 
            hostname::get()?.to_string_lossy().to_string());
        
        Ok(properties)
    }

    /// Create Device Info TXT records
    pub fn create_device_info_txt_records() -> Result<HashMap<String, String>> {
        let mut properties = HashMap::new();
        
        properties.insert("model".to_string(), "Windows,1".to_string());
        properties.insert("osxvers".to_string(), "10".to_string());
        properties.insert("srcvers".to_string(), "350.92.4".to_string());
        properties.insert("features".to_string(), "0x445F8A00,0x1C340".to_string());
        properties.insert("flags".to_string(), "0x4".to_string());
        properties.insert("vv".to_string(), "2".to_string());
        properties.insert("pk".to_string(), Self::generate_device_hash());
        
        Ok(properties)
    }

    /// Update TXT records with current session information
    pub fn update_session_records(properties: &mut HashMap<String, String>) {
        properties.insert("session_id".to_string(), Self::generate_session_hash());
        properties.insert("timestamp".to_string(), 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string());
        properties.insert("phash".to_string(), Self::generate_device_hash());
    }

    /// Validate if TXT records look Apple-compatible
    pub fn validate_apple_compatibility(properties: &HashMap<String, String>) -> bool {
        let required_keys = [
            "flags", "protocol_version", "computerid", "systemid", 
            "model", "supports_airdrop", "phash"
        ];
        
        required_keys.iter().all(|key| properties.contains_key(*key))
    }
}
