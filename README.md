# Volumetrik (Windows Edition)

> **A high-performance, native disk usage analyzer built with Rust and egui.**

**Volumetrik** is a blazing fast desktop tool designed to visualize disk usage on Windows. Built with performance in mind using Rust's parallel computing capabilities and the `eframe` GUI library, it provides a sleek, native interface to explore your file system and identify large directories.

---

## ✨ Features

### 🚀 Performance & Core
- **Ultra-Fast Scanning**: Leverages `rayon` for multi-threaded directory traversal, capable of processing millions of files in seconds.
- **Native Windows App**: Built with `eframe` (egui) for a responsive, lightweight desktop experience.
- **Low Memory Footprint**: Efficient resource usage compared to Electron-based alternatives.

### 🖥️ Modern Interface
- **Dark & Light Mode**: Toggle between themes to match your system preference.
- **Interactive File Browser**: Navigate your file system directly within the app.
- **Sorting & Filtering**: Sort files by size, name, file count, or modification date.
- **Visual Feedback**: Progress bars for storage usage and scanning status.

---

## 📥 Installation

### 🚀 Download Latest Version
Get the latest release for Windows from our GitHub Releases page:

[**⬇️ Download Volumetrik Installer**](https://github.com/xev47/Volumetrik/blob/Windows/installers/VolumetrikSetup.exe)

### Option 1: Installer (Recommended)
Download and run the `VolumetrikSetup.exe` installer linked above. This will install Volumetrik on your system and create shortcuts in your Start Menu and Desktop.

### Option 2: Portable Executable
If you prefer not to install anything, you can download the standalone `volumetrik.exe` from the [Releases Page](https://github.com/xev47/Volumetrik/releases) and run it directly.

---

## 🛠️ Build from Source

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Inno Setup](https://jrsoftware.org/isinfo.php) (optional, for creating the installer)

### Steps

1. **Clone the repository**
   ```powershell
   git clone https://github.com/xev47/Volumetrik.git
   cd volumetrik
   ```

2. **Build for Release**
   This command compiles the application and embeds the application icon.
   ```powershell
   cargo build --release
   ```
   The executable will be located at `target/release/volumetrik.exe`.

3. **Create Installer (Optional)**
   If you have Inno Setup installed, you can generate the installer:
   - Open `volumetrik.iss`
   - Click "Compile"
   - The installer will be generated in the `installers/` directory.

---

## 📖 Usage

1. **Select a Folder**: 
   - Type the path in the top bar and press Enter.
   - Or click the **Folder (📂)** icon to browse.
2. **Scan**: Click the **Scan (🔍)** button.
3. **Analyze**:
   - View **Size**, **Usage %**, **File Count**, and **Last Modified** date.
   - Click on folder names to navigate into them.
   - Click "Up" to go to the parent directory.
   - Click column headers to sort the results.
4. **Theme**: Click the Sun/Moon icon in the top right to toggle themes.

---

## 🛠️ Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) 🦀
- **GUI Framework**: [eframe / egui](https://github.com/emilk/egui)
- **Parallelism**: [Rayon](https://github.com/rayon-rs/rayon)
- **Image Handling**: [image](https://github.com/image-rs/image)

## 📝 License

This project is licensed under the MIT License.
