![Volumetrik Logo](https://raw.githubusercontent.com/xev47/volumetrik/main/static/logo_volumetrik_mini.png) 
## Volumetrik

> **A high-performance, modern disk usage analyzer and monitoring tool built with Rust.**

[![Docker Pulls](https://img.shields.io/docker/pulls/xev47/volumetrik)](https://hub.docker.com/r/xev47/volumetrik)
[![Docker Image Size](https://img.shields.io/docker/image-size/xev47/volumetrik)](https://hub.docker.com/r/xev47/volumetrik)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[**View on Docker Hub**](https://hub.docker.com/r/xev47/volumetrik)

**Volumetrik** is a blazing fast tool designed to visualize disk usage and monitor storage health. Built with performance in mind using Rust's parallel computing capabilities, it provides a sleek, modern web interface to explore your file system, identify large directories, and receive alerts when storage thresholds are breached.

Now built on **Alpine Linux** for an ultra-lightweight container footprint.

---

## ✨ Features

### 🚀 Performance & Core
- **Ultra-Fast Scanning**: Leverages `rayon` for multi-threaded directory traversal, capable of processing millions of files in seconds.
- **Alpine Base**: The Docker image is built on **Alpine Linux**, ensuring a minimal security surface and small download size.
- **Docker Ready**: Optimized for containerized deployment with easy volume mapping.

### 🖥️ Modern Dashboard
- **Customizable Layout**: Drag-and-drop widgets using **GridStack**. Resize and rearrange panels to fit your workflow. Includes a "Reset Layout" option.
- **Interactive Charts**: Visual breakdown of file types using responsive Bar and Doughnut charts.
- **Theming**: 
  - **Dark/Light Mode**: Toggle between light and dark themes.
  - **Color Palettes**: Choose from multiple accent palettes: **Default (Slate), Ocean, Sunset, Forest, and Purple**.
- **File Browser**: Integrated web-based file browser to navigate and select scan targets directly from the UI.

### 🌍 Internationalization
- Fully localized interface available in:
  - 🇺🇸 English
  - 🇫🇷 French
  - 🇪🇸 Spanish
  - 🇩🇪 German
  - 🇮🇹 Italian
  - 🇨🇳 Chinese (Simplified)
  - 🇯🇵 Japanese

### 🔔 Advanced Monitoring & Alerts
- **Background Watchdog**: Continuously monitor multiple paths in the background.
- **Threshold Types**:
  - **Max Used**: Trigger alert when a folder exceeds a specific size (GB).
  - **Min Remaining**: Trigger alert when free disk space falls below a specific limit (GB).
- **Multi-Channel Notifications**:
  - **Telegram**
  - **Slack**
  - **Discord**
  - **Microsoft Teams**
  - **Pushover**
  - **Gotify**
  - **Ntfy**
  - **Generic Webhooks**
- **Customizable Messages**: Define your own alert payload templates using variables like `{path}` and `{threshold}`.

---

## 📸 Screenshots

![Volumetrik Dashboard](https://raw.githubusercontent.com/xev47/Volumetrik/refs/heads/main/static/screenshoot.png)

![Volumetrik Configuration](https://raw.githubusercontent.com/xev47/Volumetrik/refs/heads/main/static/screenshoot2.png)

## 🚀 Quick Start with Docker

The easiest way to run Volumetrik is using Docker.

```bash
docker run -d \
  -p 8080:8080 \
  -v /:/host:ro \
  -v volumetrik_data:/app/settings \
  --name Volumetrik \
  xev47/volumetrik:latest
```

Then open your browser at **[http://localhost:8080](http://localhost:8080)**.

> **Note**: 
> 1. We mount the host root `/` to `/host` inside the container in read-only mode (`:ro`) so Volumetrik can scan your entire drive safely.
> 2. We mount a volume to `/app/settings` to persist your configuration (language, monitoring paths, alerts, layout) across restarts.

---

## 🛠️ Tech Stack

- **Core**: [Rust](https://www.rust-lang.org/) 🦀 (Actix-web, Rayon, Tokio, Serde)
- **Frontend**: HTML5, CSS3, Vanilla JS, [Chart.js](https://www.chartjs.org/), [GridStack.js](https://gridstackjs.com/)
- **Container**: Alpine Linux

## 📖 Usage

1. **Select a Folder**: Click the folder icon 📂. Use the built-in file browser to select a target directory (start at `/host` if using Docker).
2. **Scan**: Hit the "Scan" button. You can stop the scan at any time.
3. **Analyze**:
   - View **Total Size**, **File Count**, and **Disk Availability**.
   - Check the **File Type Distribution** to see which extensions consume the most space.
   - Use the **File List** to sort by size, name, percent usage, or date.
   - Click on any folder in the list or chart to **drill down**.
4. **Configure**:
   - Click the **Settings (⚙️)** icon to access the configuration menu.
   - **General**: Change language, color palette, or reset the dashboard layout.
   - **Monitoring**: Add paths to watch and set check intervals.
   - **Notifications**: Configure your preferred alert services.

## 🐳 Docker Compose

If you prefer `docker-compose.yml`:

```yaml
services:
  Volumetrik:
    image: xev47/volumetrik:latest
    container_name: Volumetrik
    ports:
      - "8080:8080"
    volumes:
      - /:/host:ro
      - ./settings:/app/settings
    restart: unless-stopped
    environment:
      - RUST_LOG=info
      - APP_ENV=docker
```

## 🤝 Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on GitHub.

## 📝 License

This project is licensed under the MIT License.

---