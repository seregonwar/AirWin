# AirWin

AirWin is a Windows application that implements Apple's AirDrop and AirPlay protocols, allowing Windows PCs to interact seamlessly with Apple devices. It enables file sharing via AirDrop and screen sharing via AirPlay between Windows and Apple devices.

## DISCALIMER
the application is under development, currently (03/02/2025) the application smoothly finds devices on local network and only if you are connected to the same network, file and video(airdrop) sharing are the main purpose of the project.
## Features

- **AirDrop Support**: Send and receive files between Windows and Apple devices
- **AirPlay Support**: Stream your Windows screen to Apple devices
- **Device Discovery**: Automatic discovery of nearby Apple devices using mDNS
- **Native Integration**: Appears as a native Mac device in the network
- **Modern UI**: Built with egui for a clean and responsive interface

## Requirements

- Windows 10 or later
- Network adapter with multicast support
- Administrator privileges (for mDNS service)

## Building from Source

1. Install Rust toolchain from [rustup.rs](https://rustup.rs/)
2. Clone the repository
3. Build the project:
```bash
cargo build --release
```

## Usage

1. Run the application as administrator (required for mDNS service)
2. The application will automatically start discovering nearby Apple devices
3. For AirDrop:
   - Click "Send File" to share files with nearby Apple devices
4. For AirPlay:
   - Click "Start Screen Receiving" to begin streaming your screen

## Architecture

- **Device Discovery**: Uses mDNS for device discovery and service advertisement
- **AirDrop**: Implements Apple's AirDrop protocol for file transfer
- **AirPlay**: Implements screen capture and streaming functionality
- **UI**: Uses egui for a native-looking interface

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
