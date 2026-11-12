# 🚀 AirWin

<p align="center">
  <img src="https://img.shields.io/badge/rust-202124?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Badge"/>
  <img src="https://img.shields.io/badge/version-v0.1.0b-blueviolet?style=for-the-badge" alt="Version Badge"/>
  <img src="https://img.shields.io/github/stars/seregonwar/AirWin?style=for-the-badge&logo=github&color=yellow" alt="GitHub Stars"/>
  <img src="https://img.shields.io/badge/license-GPL--2.0-2ea44f?style=for-the-badge&logo=open-source-initiative&logoColor=white" alt="License Badge"/>
  <img src="https://img.shields.io/github/downloads/seregonwar/AirWin/total.svg?style=for-the-badge&color=orange&logo=cloud-download" alt="Total Downloads"/>
</p>

---

## 📌 Overview

**AirWin** is a Windows application that implements Apple’s **AirDrop** and **AirPlay** protocols, allowing your PC to communicate and interact seamlessly with Apple devices.

With AirWin you can:
- 📤 Share files using AirDrop
- 📺 Stream your screen using AirPlay  
All operations work **locally** over your network, without cloud or third-party services.

---

## ✨ Features

- 🔁 **AirDrop**: Send and receive files between Windows PCs and Apple devices  
- 🖥️ **AirPlay**: Stream your Windows screen to compatible Apple devices  
- 📡 **Device Discovery**: Automatically discovers Apple devices using **mDNS**  
- 🍏 **Native Integration**: Appears as a native Mac device on the network  
- 🎨 **Modern Interface**: Clean and responsive UI

---

## 💻 System Requirements

- 🧩 Windows 10 or later  
- 🌐 Network adapter with **multicast** support  
- 🔐 Run as administrator (required for mDNS service)

---

## 🧱 Building
### 1. Clone the repository:
```bash
git clone https://github.com/seregonwar/AirWin.git
```

### 2. Build the project:
```bash
cd AirWin
cargo build
cargo run
```

---

## ▶️ Usage

1. Run the application **as administrator**  
2. AirWin will automatically start discovering nearby Apple devices  
3. For **AirDrop**:
   - Click on **“Send File”** to share files with nearby devices  
4. For **AirPlay**:
   - Click on **“Start Screen Streaming”** to broadcast your screen

---

## 🧠 Architecture

- 🔍 **mDNS Discovery**: Uses multicast DNS to discover and advertise services  
- 💾 **AirDrop Protocol**: Implements Apple’s protocol for peer-to-peer file transfer  
- 📡 **AirPlay Engine**: Handles screen capturing and streaming  
- 🧰 **UI**: Modern, responsive user interface (based on egui or custom UI)

---

## 📜 License

This project is licensed under the **GNU General Public License v2.0** (`GPL-2.0-only`).  
See the `LICENSE` file for more details.

---

## 🤝 Contributing

Contributions are **welcome**!  
Feel free to open a **Pull Request** or report issues in the tracker.

---

> Built with ❤️ by [SeregonWar](https://github.com/seregonwar) 




