use std::net::{IpAddr, Ipv4Addr};
use anyhow::{Result, Context};
use socket2::{Socket, Domain, Type, Protocol};
use tracing::{info, warn};
use local_ip_address::list_afinet_netifas;

pub struct NetworkInterface {
    name: String,
    ip: IpAddr,
    is_valid: bool,
}

impl NetworkInterface {
    pub fn new(name: String, ip: IpAddr) -> Self {
        let is_valid = match ip {
            IpAddr::V4(addr) => {
                !addr.is_loopback() && 
                !addr.is_link_local() && 
                !addr.is_multicast() &&
                !addr.is_broadcast()
            }
            IpAddr::V6(_) => false, // We'll focus on IPv4 for now
        };

        Self { name, ip, is_valid }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }
}

pub struct NetworkManager {
    multicast_socket: Option<Socket>,
    interfaces: Vec<NetworkInterface>,
}

impl NetworkManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            multicast_socket: None,
            interfaces: Vec::new(),
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        info!("Initializing network manager...");
        
        // Create UDP socket for multicast
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
            .context("Failed to create multicast socket")?;
        
        socket.set_reuse_address(true)
            .context("Failed to set SO_REUSEADDR")?;

        // Get all network interfaces
        let interfaces = list_afinet_netifas()
            .context("Failed to list network interfaces")?;

        self.interfaces = interfaces
            .into_iter()
            .map(|(name, ip)| NetworkInterface::new(name, ip))
            .collect();

        // Filter and log interface status
        let valid_interfaces: Vec<_> = self.interfaces
            .iter()
            .filter(|iface| iface.is_valid)
            .collect();

        if valid_interfaces.is_empty() {
            return Err(anyhow::anyhow!("No valid network interfaces found"));
        }

        for iface in &valid_interfaces {
            info!("Valid network interface: {} - {}", iface.name(), iface.ip());
        }

        self.multicast_socket = Some(socket);
        Ok(())
    }

    pub fn join_multicast_group(&self, multicast_addr: Ipv4Addr) -> Result<()> {
        let socket = self.multicast_socket.as_ref()
            .context("Multicast socket not initialized")?;

        let mut success = false;
        for iface in &self.interfaces {
            if !iface.is_valid {
                continue;
            }

            if let IpAddr::V4(interface_addr) = iface.ip {
                // Skip link-local addresses (169.254.x.x)
                if interface_addr.is_link_local() {
                    continue;
                }
                
                match socket.join_multicast_v4(&multicast_addr, &interface_addr) {
                    Ok(_) => {
                        info!("Successfully joined multicast group on interface {} ({})", 
                            iface.name, interface_addr);
                        success = true;
                    }
                    Err(e) => {
                        warn!("Failed to join multicast on {}: {}", iface.name, e);
                    }
                }
            }
        }

        if !success {
            return Err(anyhow::anyhow!("Failed to join multicast group on any interface"));
        }

        Ok(())
    }

    pub fn get_valid_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.iter()
            .filter(|iface| iface.is_valid)
            .collect()
    }
}
