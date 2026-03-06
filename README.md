# Recall

> A fast, lightweight clipboard history manager for Linux and macOS — built with Tauri + SvelteKit + Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Snap Store](https://snapcraft.io/static/images/badges/en/snap-store-black.svg)](https://snapcraft.io/recall)

---

## Features

- **Clipboard history** — automatically captures everything you copy (text, HTML, RTF, images)
- **Instant search** — filter your history in real time (`⌘K` / `Ctrl+K`)
- **Keyboard navigation** — browse with `↑ ↓` and paste with `Enter`
- **Global shortcut** — open from anywhere with `CmdOrCtrl+Shift+V` (configurable)
- **Image support** — preview and re-paste copied images
- **Persistent history** — survives app restarts, configurable size (default: 5000 entries)
- **Autostart** — optionally launch at login
- **System tray** — lives quietly in your tray, out of the way
- **Lightweight** — native performance via Rust backend, no Electron

---

## Installation

### Linux — Snap Store

```bash
sudo snap install recall
```

Or via the [Snap Store page](https://snapcraft.io/recall).

### macOS — DMG

Download the latest `.dmg` from the [Releases](https://github.com/mouafus/recall/releases) page.

---

## Usage

| Action | Shortcut |
|---|---|
| Open Recall | `CmdOrCtrl+Shift+V` |
| Search history | `⌘K` / `Ctrl+K` |
| Navigate items | `↑` / `↓` |
| Paste selected item | `Enter` or double-click |
| Close window | `Escape` |

Settings are accessible from the system tray icon.

---

## Building from source

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 18+
- **Linux only:** `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `libxdo-dev`
- **macOS only:** Xcode Command Line Tools (`xcode-select --install`)

### Steps

```bash
git clone https://github.com/mouafus/recall.git
cd recall
npm install
npm run tauri dev      # development mode
npm run tauri build    # production build
```

Built artifacts are in `target/release/bundle/`.

---

## Configuration

Settings are saved in your app data directory:

- **Linux:** `~/.local/share/recall/`
- **macOS:** `~/Library/Application Support/com.recall.app/`

| Setting | Default | Description |
|---|---|---|
| `shortcut` | `CmdOrCtrl+Shift+V` | Global shortcut to open Recall |
| `max_items` | `5000` | Maximum number of history entries |
| `autostart` | `true` | Launch at login |

---

## Tech stack

| Layer | Technology |
|---|---|
| UI | SvelteKit 5 + TypeScript + Tailwind CSS 4 |
| Backend | Rust + Tauri 2 |
| Clipboard | `arboard` + `clipboard-master` |
| Persistence | JSON (via `serde_json`) |
| Packaging | Snap (Linux), DMG/APP (macOS) |

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

---

## License

[MIT](LICENSE) © 2025 mouafus
