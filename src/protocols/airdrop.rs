use anyhow::{Result, Context, anyhow};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::net::{TcpStream, TcpListener, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use serde_json;
use uuid::Uuid;
use tracing::{info, warn, error};
use mdns_sd::{ServiceDaemon, ServiceInfo};

use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType};
use tokio_native_tls::{TlsAcceptor, native_tls, TlsConnector};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use socket2::{Socket, Domain, Type, Protocol};
use super::apple_records::AppleRecords;
use super::http_server::AirDropHttpServer;
use mime_guess;

#[derive(Clone, Debug, PartialEq)]
pub enum AirDropStatus {
    Idle,
    Connecting,
    Connected,
    Failed(String),
    Transferring(f32),  // Progress percentage
}

#[derive(Debug, Serialize, Deserialize)]
struct FileTransfer {
    id: String,
    name: String,
    size: u64,
    mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AirDropHandshake {
    sender: String,
    receiver: String,
    files: Vec<FileTransfer>,
}

#[derive(Clone)]
pub struct AirDrop {
    current_file: Arc<Mutex<Option<PathBuf>>>,
    transfer_progress: Arc<Mutex<f32>>,
    connection: Arc<Mutex<Option<TcpStream>>>,
    mdns: Arc<Mutex<Option<ServiceDaemon>>>,
    udp_socket: Arc<Mutex<Option<UdpSocket>>>,
    http_server: Arc<Mutex<Option<AirDropHttpServer>>>,
    pub status: Arc<Mutex<AirDropStatus>>,
}


impl AirDrop {
    pub fn new() -> Self {
        Self {
            current_file: Arc::new(Mutex::new(None)),
            transfer_progress: Arc::new(Mutex::new(0.0)),
            connection: Arc::new(Mutex::new(None)),
            mdns: Arc::new(Mutex::new(None)),
            udp_socket: Arc::new(Mutex::new(None)),
            http_server: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(AirDropStatus::Idle)),
        }
    }

    pub async fn send_file_to(&self, addr: SocketAddr, file_path: PathBuf) -> Result<()> {
        *self.status.lock().await = AirDropStatus::Connecting;

        let file = File::open(&file_path)
            .await
            .context("Failed to open file")?;

        let metadata = file.metadata().await?;
        let transfer = FileTransfer {
            id: Uuid::new_v4().to_string(),
            name: file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            size: metadata.len(),
            mime_type: mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string(),
        };

        // Generate certificate for TLS
        let (identity, _) = Self::generate_certificate().await?;
        let connector = native_tls::TlsConnector::builder()
            .identity(identity)
            .build()?;
        let connector = TlsConnector::from(connector);

        // Establish TCP connection to target peer
        let stream = TcpStream::connect(addr).await?;
        *self.status.lock().await = AirDropStatus::Connected;

        // Perform TLS handshake; server name must match CN used by server cert
        let mut tls_stream = connector.connect("AirWin", stream).await?;

        // Send a simple JSON handshake
        let handshake = AirDropHandshake {
            sender: "AirWin".to_string(),
            receiver: "AirWin".to_string(),
            files: vec![transfer],
        };

        let handshake_json = serde_json::to_string(&handshake)?;
        tls_stream.write_all(handshake_json.as_bytes()).await?;
        tls_stream.write_all(b"\n\n").await?;

        // Stream file contents
        let mut file = File::open(&file_path).await?;
        let mut buffer = vec![0; 8192];
        let mut sent = 0u64;

        *self.status.lock().await = AirDropStatus::Transferring(0.0);

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 { break; }
            tls_stream.write_all(&buffer[..n]).await?;
            sent += n as u64;
            let progress = (sent as f32 / metadata.len() as f32) * 100.0;
            *self.transfer_progress.lock().await = progress;
            *self.status.lock().await = AirDropStatus::Transferring(progress);
        }

        *self.status.lock().await = AirDropStatus::Connected;
        *self.current_file.lock().await = Some(file_path);
        Ok(())
    }
    

    pub async fn get_status(&self) -> AirDropStatus {
        self.status.lock().await.clone()
    }

    async fn setup_multicast() -> Result<UdpSocket> {
        // Create socket with socket2 for more control
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        
        // Set socket options
        socket.set_reuse_address(true)?;
        socket.set_multicast_loop_v4(true)?;
        socket.set_multicast_ttl_v4(255)?;
        socket.set_broadcast(true)?;

        
        // Bind to mDNS port
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 7000);  // Changed to 7000
        socket.bind(&addr.into())?;
        
        // Join multicast group on all interfaces
        let multicast_addr: Ipv4Addr = "224.0.0.251".parse()?;
        let interfaces = local_ip_address::list_afinet_netifas()?;
        
        for (name, ip) in interfaces {
            if let IpAddr::V4(interface_addr) = ip {
                // Skip loopback, multicast, and link-local addresses (169.254.x.x)
                if !ip.is_loopback() && !ip.is_multicast() && !interface_addr.is_link_local() {  
                    info!("Joining multicast group on interface {} ({})", name, interface_addr);
                    if let Err(e) = socket.join_multicast_v4(&multicast_addr, &interface_addr) {
                        warn!("Failed to join multicast on {}: {}", name, e);
                    }
                }
            }
        }
        
        // Convert to tokio UdpSocket
        let std_socket: std::net::UdpSocket = socket.into();
        std_socket.set_nonblocking(true)?;
        Ok(UdpSocket::from_std(std_socket)?)
    }



    async fn register_mdns_services(&self) -> Result<()> {
        let mdns = ServiceDaemon::new().map_err(|e| anyhow!("Failed to initialize mDNS: {}", e))?;
        
        // Use Apple-compatible TXT records
        let airdrop_properties = AppleRecords::create_airdrop_txt_records()?;
        let companion_properties = AppleRecords::create_companion_txt_records()?;
        let device_info_properties = AppleRecords::create_device_info_txt_records()?;
        
        let hostname = hostname::get()?.to_string_lossy().to_string();
        
        // Register AirDrop TCP service on standard port
        let airdrop_tcp_service = ServiceInfo::new(
            "_airdrop._tcp.local.",
            &hostname,
            "local.",
            "",
            8771, // Standard AirDrop port
            Some(airdrop_properties.clone())
        )?;

        // Register AirDrop UDP service
        let airdrop_udp_service = ServiceInfo::new(
            "_airdrop._udp.local.",
            &hostname,
            "local.",
            "",
            8771, // Standard AirDrop port
            Some(airdrop_properties)
        )?;

        // Register Companion Link service (for device pairing)
        let companion_service = ServiceInfo::new(
            "_companion-link._tcp.local.",
            &hostname,
            "local.",
            "",
            7001,
            Some(companion_properties)
        )?;

        // Register Device Info service
        let device_info_service = ServiceInfo::new(
            "_device-info._tcp.local.",
            &hostname,
            "local.",
            "",
            7002,
            Some(device_info_properties)
        )?;

        // Register all services
        mdns.register(airdrop_tcp_service)
            .map_err(|e| anyhow!("Failed to register AirDrop TCP service: {}", e))?;
        mdns.register(airdrop_udp_service)
            .map_err(|e| anyhow!("Failed to register AirDrop UDP service: {}", e))?;
        mdns.register(companion_service)
            .map_err(|e| anyhow!("Failed to register Companion Link service: {}", e))?;
        mdns.register(device_info_service)
            .map_err(|e| anyhow!("Failed to register Device Info service: {}", e))?;

        info!("Successfully registered Apple-compatible mDNS services");
        *self.mdns.lock().await = Some(mdns);

        // Setup UDP multicast with explicit binding to all interfaces
        let socket = Self::setup_multicast().await?;
        *self.udp_socket.lock().await = Some(socket);

        Ok(())
    }

    async fn generate_certificate() -> Result<(native_tls::Identity, String)> {
        info!("Generating new TLS certificate...");
        let mut params = CertificateParams::new(vec!["AirWin".to_string()]);
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, "AirWin");
        params.distinguished_name.push(DnType::OrganizationName, "AirWin");
        params.distinguished_name.push(DnType::CountryName, "US");

        let cert = Certificate::from_params(params)?;
        // Keep a PEM string for optional display/diagnostics
        let cert_pem = cert.serialize_pem()?;
        // Use DER for native-tls on Windows when constructing an Identity from PKCS#8
        let cert_der = cert.serialize_der()?;
        let key_der = cert.serialize_private_key_der();

        let identity = native_tls::Identity::from_pkcs8(&cert_der, &key_der)?;

        Ok((identity, cert_pem))
    }

    async fn handle_connection(stream: TcpStream, addr: SocketAddr) -> Result<()> {
        info!("Handling new connection from {}", addr);
        
        // Generate or load certificate
        let (identity, _) = Self::generate_certificate().await?;
        let acceptor = TlsAcceptor::from(native_tls::TlsAcceptor::new(identity)?);

        let mut stream = acceptor.accept(stream).await?;

        // Read handshake
        let mut buffer = Vec::new();
        let mut temp_buf = [0u8; 1024];
        
        loop {
            let n = stream.read(&mut temp_buf).await?;
            if n == 0 { break; }
            buffer.extend_from_slice(&temp_buf[..n]);
            if buffer.windows(2).any(|w| w == b"\n\n") {
                break;
            }
        }

        let handshake: AirDropHandshake = serde_json::from_slice(&buffer)?;
        info!("Received handshake from {}: {:?}", addr, handshake);

        // Accept the transfer
        let response = serde_json::json!({
            "status": "accept",
            "receiver": handshake.receiver,
        });
        
        stream.write_all(serde_json::to_string(&response)?.as_bytes()).await?;
        stream.write_all(b"\n\n").await?;

        // Receive files
        for file in handshake.files {
            let mut file_data = Vec::with_capacity(file.size as usize);
            let mut received = 0u64;
            
            while received < file.size {
                let n = stream.read(&mut temp_buf).await?;
                if n == 0 { break; }
                file_data.extend_from_slice(&temp_buf[..n]);
                received += n as u64;
            }

            // Save file
            let path = std::env::temp_dir().join(&file.name);
            tokio::fs::write(&path, file_data).await?;
            info!("Saved file {} to {:?}", file.name, path);
        }

        Ok(())
    }

    pub async fn start_server(&self) -> Result<()> {
        *self.status.lock().await = AirDropStatus::Connecting;
        
        // Register mDNS services first
        self.register_mdns_services().await?;

        // Initialize and start HTTPS server for AirDrop protocol
        let mut http_server = AirDropHttpServer::new(8771); // Use standard AirDrop port
        http_server.initialize().await?;
        http_server.start().await?;
        
        *self.http_server.lock().await = Some(http_server);
        info!("Started AirDrop HTTPS server on port 8771");

        // Keep the old TCP listener for backward compatibility
        let v4_listener = match TcpListener::bind(("0.0.0.0", 7000)).await {
            Ok(listener) => {
                info!("Started AirDrop IPv4 fallback server on 0.0.0.0:7000");
                listener
            }
            Err(e) => {
                warn!("Failed to start AirDrop IPv4 fallback server: {}", e);
                // Don't fail completely if fallback server can't start
                *self.status.lock().await = AirDropStatus::Connected;
                return Ok(());
            }
        };

        let status = self.status.clone();
        let transfer_progress = self.transfer_progress.clone();
        
        tokio::spawn(async move {
            loop {
                match v4_listener.accept().await {
                    Ok((stream, addr)) => {
                        info!("Accepted IPv4 connection from {}", addr);
                        *status.lock().await = AirDropStatus::Connected;
                        
                        let status = status.clone();
                        let progress = transfer_progress.clone();
                        
                        tokio::spawn(async move {
                            if let Err(e) = Self::handle_connection(stream, addr).await {
                                error!("Error handling connection: {}", e);
                                *status.lock().await = AirDropStatus::Failed(format!("Connection error: {}", e));
                            }
                            let current_progress = *progress.lock().await;
                            *status.lock().await = AirDropStatus::Transferring(current_progress);
                        });
                    }
                    Err(e) => {
                        warn!("IPv4 accept error: {}", e);
                        *status.lock().await = AirDropStatus::Failed(format!("Accept error: {}", e));
                        break;
                    }
                }
            }
        });

        // Try binding to IPv6 as optional
        if let Ok(v6_listener) = TcpListener::bind(("[::1]", 7000)).await {
            info!("Started AirDrop IPv6 server on [::1]:7000");
            let status = self.status.clone();
            let transfer_progress = self.transfer_progress.clone();
            
            tokio::spawn(async move {
                loop {
                    if let Ok((stream, addr)) = v6_listener.accept().await {
                        info!("Accepted IPv6 connection from {}", addr);
                        let status = status.clone();
                        let progress = transfer_progress.clone();
                        
                        tokio::spawn(async move {
                            if let Err(e) = Self::handle_connection(stream, addr).await {
                                error!("Error handling IPv6 connection: {}", e);
                                *status.lock().await = AirDropStatus::Failed(format!("IPv6 connection error: {}", e));
                            }
                            let current_progress = *progress.lock().await;
                            *status.lock().await = AirDropStatus::Transferring(current_progress);
                        });
                    }
                }
            });
        }

        Ok(())
    }

    pub async fn send_file(&self, file_path: PathBuf) -> Result<()> {
        *self.status.lock().await = AirDropStatus::Connecting;
        
        let file = File::open(&file_path)
            .await
            .context("Failed to open file")?;
        
        let metadata = file.metadata().await?;
        let transfer = FileTransfer {
            id: Uuid::new_v4().to_string(),
            name: file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            size: metadata.len(),
            mime_type: mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string(),
        };

        // Generate certificate for TLS
        let (identity, _) = Self::generate_certificate().await?;
        let connector = native_tls::TlsConnector::builder()
            .identity(identity)
            .build()?;
        let connector = tokio_native_tls::TlsConnector::from(connector);

        // Try IPv4 connection first
        let mut connection = self.connection.lock().await;
        if let Some(stream) = connection.take() {
            info!("Sending file over IPv4 connection");
            *self.status.lock().await = AirDropStatus::Connected;
            
            let mut tls_stream = connector.connect("AirDrop", stream).await?;
            
            let handshake = AirDropHandshake {
                sender: "AirWin".to_string(),
                receiver: "AirDrop".to_string(),
                files: vec![transfer],
            };

            let handshake_json = serde_json::to_string(&handshake)?;
            tls_stream.write_all(handshake_json.as_bytes()).await?;
            tls_stream.write_all(b"\n\n").await?;

            let mut file = File::open(&file_path).await?;
            let mut buffer = vec![0; 8192];
            let mut sent = 0u64;
            
            *self.status.lock().await = AirDropStatus::Transferring(0.0);
            
            while let Ok(n) = file.read(&mut buffer).await {
                if n == 0 { break; }
                tls_stream.write_all(&buffer[..n]).await?;
                sent += n as u64;
                let progress = (sent as f32 / metadata.len() as f32) * 100.0;
                *self.transfer_progress.lock().await = progress;
                *self.status.lock().await = AirDropStatus::Transferring(progress);
            }

            *self.status.lock().await = AirDropStatus::Connected;
            
            // After transfer, establish a new connection for future use
            let new_stream = TcpStream::connect(tls_stream.get_ref().get_ref().get_ref().peer_addr()?).await?;
            *connection = Some(new_stream);
        } else {
            *self.status.lock().await = AirDropStatus::Failed("No active connection available".to_string());
            return Err(anyhow!("No active connection available"));
        }
        
        *self.current_file.lock().await = Some(file_path);
        Ok(())
    }
}
