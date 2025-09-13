use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use tracing::{info, error, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use socket2::{Socket, Domain, Type, Protocol};
use super::interface::NetworkManager;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct DiscoveredDevice {
	pub name: String,
	pub address: IpAddr,
	#[allow(dead_code)]
	pub port: u16,
	pub service_type: ServiceType,
	pub txt_records: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServiceType {
	AirPlay,
	AirDrop,
	Raop,
	Companion,
	DeviceInfo,
	#[allow(dead_code)]
	IosMobile,
	#[allow(dead_code)]
	IosPairable,
	#[allow(dead_code)]
	IosContinuity,
	#[allow(dead_code)]
	DnsService,
	#[allow(dead_code)]
	Homekit,
	#[allow(dead_code)]
	AirPrint,
	#[allow(dead_code)]
	AppleTV,
	#[allow(dead_code)]
	RemoteDevice,
	#[allow(dead_code)]
	HomeSharing,
	#[allow(dead_code)]
	AppleMidi,
	#[allow(dead_code)]
	AirPort,
	#[allow(dead_code)]
	AppleAuth,
	#[allow(dead_code)]
	Presence,
}

#[allow(dead_code)]
pub struct DeviceDiscovery {
	mdns: Arc<ServiceDaemon>,
	devices: Arc<Mutex<HashMap<String, DiscoveredDevice>>>,
	running: Arc<AtomicBool>,
	network_manager: NetworkManager,
}

impl DeviceDiscovery {
	#[allow(dead_code)]
	fn check_network_availability() -> Result<()> {
		info!("Checking network availability for mDNS...");
		
		let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
		socket.set_reuse_address(true)?;
		
		let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
		if let Err(e) = socket.bind(&addr.into()) {
			error!("Network binding error: {}", e);
			return Err(e.into());
		}

		// Check multicast capability
		let multicast_addr: Ipv4Addr = "224.0.0.251".parse()?;
		let interfaces = local_ip_address::list_afinet_netifas()?;
		let mut has_valid_interface = false;

		for (name, ip) in interfaces {
			if let IpAddr::V4(interface_addr) = ip {
				if !ip.is_loopback() && !interface_addr.is_link_local() {
					has_valid_interface = true;
					if let Err(e) = socket.join_multicast_v4(&multicast_addr, &interface_addr) {
						warn!("Failed to join multicast on {}: {}", name, e);
					} else {
						info!("Successfully joined multicast group on interface {} ({})", name, interface_addr);
					}
				}
			}
		}

		if !has_valid_interface {
			error!("No valid network interfaces found for multicast");
			return Err(anyhow::anyhow!("No valid network interfaces"));
		}

		info!("Network is available with multicast support");
		Ok(())
	}

	pub fn new() -> Result<Self> {
		info!("Initializing device discovery service");
		
		// Initialize network manager
		let mut network_manager = NetworkManager::new()?;
		network_manager.initialize()?;

		// Initialize mDNS
		let mdns = ServiceDaemon::new()?;
		info!("Successfully created mDNS service daemon");

		// Join multicast group for mDNS
		let multicast_addr: Ipv4Addr = "224.0.0.251".parse()?;
		network_manager.join_multicast_group(multicast_addr)?;

		Ok(Self {
			mdns: Arc::new(mdns),
			devices: Arc::new(Mutex::new(HashMap::new())),
			running: Arc::new(AtomicBool::new(false)),
			network_manager,
		})
	}

	pub async fn start_discovery(&self) -> Result<()> {
		if self.running.load(Ordering::SeqCst) {
			return Ok(());
		}
		self.running.store(true, Ordering::SeqCst);

		info!("Starting device discovery service...");
		
		let service_types = [
			"_airplay._tcp.local.",
			"_raop._tcp.local.",
			"_airdrop._tcp.local.",
			"_companion-link._tcp.local.",
			"_device-info._tcp.local.",
		];

		for &service_type in &service_types {
			match self.mdns.browse(service_type) {
				Ok(receiver) => {
					let devices = self.devices.clone();
					let service_type = service_type.to_string();
					let running = self.running.clone();
					
					tokio::spawn(async move {
						while running.load(Ordering::SeqCst) {
							match receiver.recv_async().await {
								Ok(event) => {
									if let ServiceEvent::ServiceResolved(info) = event {
										let addresses = info.get_addresses();
										if !addresses.is_empty() {
											if let Some(addr) = addresses.iter().next() {
												let mut devices = devices.lock().await;
												let device = DiscoveredDevice {
													name: info.get_fullname().to_string(),
													address: IpAddr::V4(*addr),
													port: info.get_port(),
													service_type: match service_type.as_str() {
														"_airplay._tcp.local." => ServiceType::AirPlay,
														"_raop._tcp.local." => ServiceType::Raop,
														"_airdrop._tcp.local." => ServiceType::AirDrop,
														"_companion-link._tcp.local." => ServiceType::Companion,
														_ => ServiceType::DeviceInfo,
													},
													txt_records: info.get_properties().iter().map(|prop| {
														(prop.key().to_string(), prop.val_str().to_string())
													}).collect(),
												};
												devices.insert(device.name.clone(), device);
											}
										}
									}
								}
								Err(e) => {
									error!("Error receiving mDNS event: {}", e);
									break;
								}
							}
						}
					});
				}
				Err(e) => {
					error!("Failed to browse for service {}: {}", service_type, e);
				}
			}
		}

		Ok(())

	}

	pub async fn stop_discovery(&self) {

		self.running.store(false, Ordering::SeqCst);
		self.devices.lock().await.clear();
	}


	pub async fn get_devices(&self) -> Result<Vec<DiscoveredDevice>> {
		let devices = self.devices.lock().await;
		Ok(devices.values().cloned().collect())
	}
}
