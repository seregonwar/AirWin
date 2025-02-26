mod interface;
pub use interface::{NetworkInterface, NetworkManager};

pub mod discovery;
pub use discovery::{DeviceDiscovery, DiscoveredDevice, ServiceType};
