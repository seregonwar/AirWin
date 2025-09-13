use anyhow::{Result, anyhow};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::TlsAcceptor;
use tracing::{info, error, debug};
use serde_json;
use std::collections::HashMap;
use std::net::SocketAddr;
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType};
use tokio_rustls::rustls::{Certificate as RustlsCert, PrivateKey as RustlsKey, ServerConfig};
use tokio_rustls::server::TlsStream as RustlsTlsStream;

/// HTTP/HTTPS server for AirDrop protocol
pub struct AirDropHttpServer {
    port: u16,
    tls_acceptor: Option<TlsAcceptor>,
    running: Arc<Mutex<bool>>,
}

impl AirDropHttpServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            tls_acceptor: None,
            running: Arc::new(Mutex::new(false)),
        }
    }

    async fn build_rustls_config() -> Result<Arc<ServerConfig>> {
        info!("Generating self-signed certificate for AirDrop HTTPS server (rustls)...");

        let mut params = CertificateParams::new(vec!["AirWin".to_string()]);
        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, "AirWin");
        dn.push(DnType::OrganizationName, "AirWin");
        dn.push(DnType::CountryName, "US");
        params.distinguished_name = dn;

        let cert = Certificate::from_params(params)?;
        let cert_der = cert.serialize_der()?;
        let key_der = cert.serialize_private_key_der();

        let cert_chain = vec![RustlsCert(cert_der)];
        let key = RustlsKey(key_der);

        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)?;

        Ok(Arc::new(config))
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let config = Self::build_rustls_config().await?;
        let acceptor = TlsAcceptor::from(config);
        self.tls_acceptor = Some(acceptor);
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        let acceptor = self.tls_acceptor.as_ref()
            .ok_or_else(|| anyhow!("TLS acceptor not initialized"))?;

        let listener = TcpListener::bind(("0.0.0.0", self.port)).await?;
        info!("AirDrop HTTPS server listening on port {}", self.port);

        *self.running.lock().await = true;
        let running = self.running.clone();
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            while *running.lock().await {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        let acceptor = acceptor.clone();
                        tokio::spawn(async move {
                            if let Err(e) = Self::handle_connection(stream, addr, acceptor).await {
                                error!("Error handling connection from {}: {}", addr, e);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Error accepting connection: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        addr: SocketAddr,
        acceptor: TlsAcceptor,
    ) -> Result<()> {
        debug!("Handling HTTPS connection from {}", addr);

        let mut tls_stream = acceptor.accept(stream).await?;

        // Read HTTP request
        let mut buffer = Vec::new();
        let mut temp_buf = [0u8; 1024];
        
        loop {
            let n = tls_stream.read(&mut temp_buf).await?;
            if n == 0 { break; }
            buffer.extend_from_slice(&temp_buf[..n]);
            
            // Check for end of HTTP headers
            if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }

        let request = String::from_utf8_lossy(&buffer);
        let lines: Vec<&str> = request.lines().collect();
        
        if lines.is_empty() {
            return Err(anyhow!("Empty HTTP request"));
        }

        let request_line = lines[0];
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        
        if parts.len() < 3 {
            return Err(anyhow!("Invalid HTTP request line"));
        }

        let method = parts[0];
        let path = parts[1];
        
        debug!("HTTP {} request to {}", method, path);

        match (method, path) {
            ("GET", "/") => {
                Self::handle_root_request(&mut tls_stream).await?;
            }
            ("POST", "/Discover") => {
                Self::handle_discover_request(&mut tls_stream, &buffer).await?;
            }
            ("POST", "/Ask") => {
                Self::handle_ask_request(&mut tls_stream, &buffer).await?;
            }
            ("POST", "/Upload") => {
                Self::handle_upload_request(&mut tls_stream, &buffer).await?;
            }
            _ => {
                Self::handle_not_found(&mut tls_stream).await?;
            }
        }

        Ok(())
    }

    async fn handle_root_request(stream: &mut RustlsTlsStream<TcpStream>) -> Result<()> {
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).await?;
        Ok(())
    }

    async fn handle_discover_request(
        stream: &mut RustlsTlsStream<TcpStream>,
        _buffer: &[u8],
    ) -> Result<()> {
        info!("Handling /Discover request");

        // Create discover response for AirDrop protocol
        let mut discover_response = HashMap::new();
        discover_response.insert("ReceiverMediaCapabilities", serde_json::json!({
            "Version": 1,
            "Vendor": {
                "com.microsoft": {
                    "OSVersion": [10, 0],
                    "OSBuildVersion": "22000"
                }
            }
        }));
        discover_response.insert("ReceiverComputerName", serde_json::Value::String(
            hostname::get()?.to_string_lossy().to_string()
        ));
        discover_response.insert("ReceiverModelName", serde_json::Value::String("Windows,1".to_string()));

        let response_json = serde_json::to_string(&discover_response)?;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            response_json.len(),
            response_json
        );

        stream.write_all(response.as_bytes()).await?;
        info!("Sent discover response");
        Ok(())
    }

    async fn handle_ask_request(
        stream: &mut RustlsTlsStream<TcpStream>,
        _buffer: &[u8],
    ) -> Result<()> {
        info!("Handling /Ask request");

        // For now, always accept transfers
        let ask_response = serde_json::json!({
            "ReceiverModelName": "Windows,1",
            "ReceiverComputerName": hostname::get()?.to_string_lossy().to_string()
        });

        let response_json = serde_json::to_string(&ask_response)?;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            response_json.len(),
            response_json
        );

        stream.write_all(response.as_bytes()).await?;
        info!("Accepted file transfer request");
        Ok(())
    }

    async fn handle_upload_request(
        stream: &mut RustlsTlsStream<TcpStream>,
        buffer: &[u8],
    ) -> Result<()> {
        info!("Handling /Upload request");

        // Find the start of the body (after \r\n\r\n)
        let header_end = buffer.windows(4)
            .position(|w| w == b"\r\n\r\n")
            .ok_or_else(|| anyhow!("Could not find end of HTTP headers"))?;

        let body_start = header_end + 4;
        let body = &buffer[body_start..];

        // Save uploaded file to temp directory
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("airdrop_upload_{}.bin", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()));

        tokio::fs::write(&file_path, body).await?;
        info!("Saved uploaded file to {:?}", file_path);

        let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).await?;
        Ok(())
    }

    async fn handle_not_found(stream: &mut RustlsTlsStream<TcpStream>) -> Result<()> {
        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).await?;
        Ok(())
    }

    pub async fn stop(&self) {
        *self.running.lock().await = false;
    }
}
