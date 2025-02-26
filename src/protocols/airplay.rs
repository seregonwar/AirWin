use anyhow::{Result, Context, anyhow};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpStream, TcpListener};
use tokio::io::AsyncWriteExt;
use windows::Win32::Graphics::Gdi::{GetDC, BitBlt, SRCCOPY};
use windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use windows::Win32::UI::WindowsAndMessaging::{SM_CXSCREEN, SM_CYSCREEN};
use image::{ImageBuffer, Rgba};
use tokio::time::Duration;
use tracing::{debug, info, error};
#[derive(Clone, Debug, PartialEq)]

pub enum AirPlayStatus {
    Idle,
    Connecting,
    Connected,
    Failed(String),
}

#[derive(Clone)]
pub struct ScreenFrame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct AirPlay {
    is_receiving: Arc<AtomicBool>,
    fps: Arc<Mutex<u32>>,
    stream: Arc<Mutex<Option<TcpStream>>>,
    current_frame: Arc<Mutex<Option<ScreenFrame>>>,
    listener: Arc<Mutex<Option<TcpListener>>>,
    status: Arc<Mutex<AirPlayStatus>>,
}



impl AirPlay {
    pub fn new() -> Self {
        Self {
            is_receiving: Arc::new(AtomicBool::new(false)),
            fps: Arc::new(Mutex::new(60)),
            stream: Arc::new(Mutex::new(None)),
            current_frame: Arc::new(Mutex::new(None)),
            listener: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(AirPlayStatus::Idle)),
        }
    }

    pub async fn get_status(&self) -> AirPlayStatus {
        self.status.lock().await.clone()
    }


    pub async fn start_server(&self) -> Result<()> {
        // Check if server is already running
        if self.listener.lock().await.is_some() {
            info!("AirPlay server already running");
            return Ok(());
        }

        // Try binding to all interfaces directly
        match TcpListener::bind(("0.0.0.0", 7100)).await {
            Ok(listener) => {
                info!("Started AirPlay server on 0.0.0.0:7100");
                *self.listener.lock().await = Some(listener);

                // Also bind to IPv6 if available
                if let Ok(_v6_listener) = TcpListener::bind(("[::]", 7100)).await {
                    info!("Started AirPlay server on [::]:7100");
                    // Store the IPv6 listener or handle it as needed
                }

                Ok(())
            }
            Err(e) => {
                error!("Failed to start AirPlay server: {}", e);
                Err(anyhow!("Failed to start AirPlay server. Try running as administrator or check firewall settings."))
            }
        }
    }


    pub async fn start_receiving(&self) -> Result<()> {
        if self.is_receiving.load(Ordering::Relaxed) {
            *self.status.lock().await = AirPlayStatus::Failed("Already receiving a stream".to_string());
            return Err(anyhow!("A stream is already in progress"));
        }

        *self.status.lock().await = AirPlayStatus::Connecting;

        if self.listener.lock().await.is_none() {
            match self.start_server().await {
                Ok(_) => info!("AirPlay server started successfully"),
                Err(e) => {
                    let error_msg = format!("Failed to start server. Please check your network settings: {}", e);
                    *self.status.lock().await = AirPlayStatus::Failed(error_msg.clone());
                    return Err(anyhow!(error_msg));
                }
            }
        }

        info!("Starting screen receiving...");
        *self.stream.lock().await = None;
        
        match tokio::time::timeout(Duration::from_secs(15), self.setup_stream()).await {
            Ok(Ok(_)) => {
                self.is_receiving.store(true, Ordering::Relaxed);
                *self.status.lock().await = AirPlayStatus::Connected;

                
                let status = self.status.clone();
                let is_receiving = self.is_receiving.clone();
                let fps = self.fps.clone();
                let this = self.clone();
                
                tokio::spawn(async move {
                    while is_receiving.load(Ordering::Relaxed) {
                        if let Err(e) = this.capture_screen().await {
                            error!("Screen capture error: {}", e);
                            *status.lock().await = AirPlayStatus::Failed(format!("Capture error: {}", e));
                            is_receiving.store(false, Ordering::Relaxed);
                            break;
                        }
                        let current_fps = *fps.lock().await;
                        tokio::time::sleep(Duration::from_millis(1000u64 / current_fps as u64)).await;
                    }
                    info!("Screen receiving stopped");
                    *status.lock().await = AirPlayStatus::Idle;
                });
                
                Ok(())
            }
            Ok(Err(e)) => {
                self.is_receiving.store(false, Ordering::Relaxed);
                let error_msg = format!("Failed to setup stream: {}", e);
                *self.status.lock().await = AirPlayStatus::Failed(error_msg.clone());
                Err(anyhow!(error_msg))
            }
            Err(_) => {
                self.is_receiving.store(false, Ordering::Relaxed);
                let error_msg = "Connection timeout after 15 seconds".to_string();
                *self.status.lock().await = AirPlayStatus::Failed(error_msg.clone());
                Err(anyhow!(error_msg))
            }
        }
    }

    async fn setup_stream(&self) -> Result<()> {
        info!("Waiting for AirPlay connection... Please connect from your iOS/macOS device");
        let listener = self.listener.lock().await;
        
        if let Some(listener) = &*listener {
            let _ = listener;
            
            match tokio::time::timeout(Duration::from_secs(15), async {
                if let Some(listener) = &*self.listener.lock().await {
                    match listener.accept().await {
                        Ok((stream, addr)) => {
                            info!("Accepted AirPlay connection from {}", addr);
                            stream.set_nodelay(true)?;
                            Ok((stream, addr))
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "Server not ready"))
                }
            }).await {
                Ok(Ok((stream, addr))) => {
                    info!("Successfully established connection with {}", addr);
                    *self.stream.lock().await = Some(stream);
                    Ok(())
                }
                Ok(Err(e)) => {
                    error!("Failed to accept connection: {}", e);
                    Err(anyhow!("Connection failed. Please check your network settings and try again."))
                }
                Err(_) => {
                    error!("Connection attempt timed out");
                    Err(anyhow!("Connection timeout. Please ensure your device is on the same network and try connecting again."))
                }
            }
        } else {
            error!("AirPlay server not started");
            Err(anyhow!("Server not ready. Please restart the application and try again."))
        }
    }


    async fn capture_screen(&self) -> Result<()> {
        if self.stream.lock().await.is_none() {
            return Err(anyhow!("No active connection"));
        }

        unsafe {
            let screen_dc = GetDC(None);
            if !screen_dc.is_invalid() {
                let width = GetSystemMetrics(SM_CXSCREEN);
                let height = GetSystemMetrics(SM_CYSCREEN);
                
                let buffer = vec![0u8; (width * height * 4) as usize];
                
                if BitBlt(
                    screen_dc,
                    0,
                    0,
                    width,
                    height,
                    screen_dc,
                    0,
                    0,
                    SRCCOPY,
                ).as_bool() {
                    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(
                        width as u32,
                        height as u32,
                        buffer,
                    ).context("Failed to create image buffer")?;
                    
                    let frame = ScreenFrame {
                        data: img.into_raw(),
                        width: width as u32,
                        height: height as u32,
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };

                    // Use frame dimensions for stream header
                    if let Some(stream) = &mut *self.stream.lock().await {
                        // Write frame header with dimensions and timestamp
                        let header = format!(
                            "{}x{}@{}\n",
                            frame.width,
                            frame.height,
                            frame.timestamp
                        );
                        stream.write_all(header.as_bytes()).await?;
                        stream.write_all(&frame.data).await?;

                        debug!(
                            "Sent frame: {}x{} at timestamp {}",
                            frame.width,
                            frame.height,
                            frame.timestamp
                        );

                    }
                    
                    *self.current_frame.lock().await = Some(frame);
                }
            }
        }
        Ok(())
    }

    pub async fn stop_receiving(&self) -> Result<()> {
        info!("Stopping screen receiving...");
        self.is_receiving.store(false, Ordering::Relaxed);
        // Clear current frame and stream
        *self.current_frame.lock().await = None;
        *self.stream.lock().await = None;
        *self.status.lock().await = AirPlayStatus::Idle;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_current_frame(&self) -> Option<ScreenFrame> {
        self.current_frame.lock().await.clone()
    }

    pub async fn get_frame_info(&self) -> Option<(u32, u32, u64)> {
        if let Some(frame) = self.current_frame.lock().await.as_ref() {
            Some((frame.width, frame.height, frame.timestamp))
        } else {
            None
        }
    }
}