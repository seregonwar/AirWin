# AirWin Detailed Documentation

## Overview

AirWin is a cross-platform application designed to bridge the gap between Windows and Apple ecosystems by implementing the AirDrop and AirPlay protocols. This allows users to seamlessly share files and screens between Windows and Apple devices.

## Architecture

AirWin is structured into modular components for maintainability and scalability:

- **Device Discovery (`discovery.rs`):**  This module handles the discovery of nearby Apple devices using the mDNS protocol.  It leverages the `mdns-sd` crate for efficient service discovery and registration.  The PC is advertised as a Mac device to ensure compatibility with Apple's ecosystem.  This includes registering services for AirDrop, AirPlay, and device information.

- **AirDrop (`airdrop.rs`):** This module implements the AirDrop file transfer protocol.  It handles file opening, transfer progress tracking, and network communication using TCP sockets.  The `serde` and `serde_json` crates are used for data serialization and deserialization.

- **AirPlay (`airplay.rs`):** This module implements the AirPlay screen mirroring functionality.  It captures the screen using Windows GDI functions, processes frames, and streams them over TCP to the receiving Apple device.  The `image` crate is used for image manipulation and scaling.

- **Main Application (`main.rs`):** This module integrates the core components with a user-friendly graphical interface built using the `eframe` and `egui` crates.  It manages user interactions, state updates, and provides visual feedback on the status of AirDrop and AirPlay operations.

## Protocol Details

### AirDrop
- **Service Type:** `_airdrop._tcp.local`
- **TXT Records:**  Includes essential information such as flags, model, protocol, services, type, and device identifiers for successful AirDrop connections.

### AirPlay
- **Service Type:** `_airplay._tcp.local`
- **TXT Records:**  Includes device-specific information like device ID, features, model, version, and other relevant details for AirPlay compatibility.

## Network Configuration

AirWin uses mDNS (Multicast DNS) for service discovery and advertisement.  The application binds to port 5353 and listens for multicast traffic on 224.0.0.251.  Specific socket options are configured to ensure reliable multicast communication.

## Dependencies

- `anyhow`: Error handling
- `eframe`/`egui`: GUI framework
- `futures`: Asynchronous programming
- `hostname`: Hostname retrieval
- `if-addrs`: Network interface information
- `image`: Image processing
- `local-ip-address`: Local IP address retrieval
- `mdns-sd`: mDNS implementation
- `mime-guess`: MIME type detection
- `serde`/`serde_json`: Data serialization
- `socket2`: Socket operations
- `tokio`: Asynchronous runtime
- `tokio-stream`: Asynchronous stream handling
- `tracing`/`tracing-subscriber`: Logging
- `uuid`: UUID generation
- `windows`: Windows API access

## Building and Running

1. **Clone the repository:** `git clone https://github.com/your-username/AirWin.git`
2. **Navigate to the project directory:** `cd AirWin`
3. **Build the project:** `cargo build --release`
4. **Run the application (as administrator):** `cargo run --release`

## Contributing

Contributions are welcome!  Please open an issue or submit a pull request.
