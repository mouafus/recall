# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-03-06

### Added
- Clipboard history capture (text, HTML, RTF, images)
- Persistent history stored as JSON across sessions
- Real-time search/filter with `⌘K` / `Ctrl+K` shortcut
- Keyboard navigation (`↑ ↓` to browse, `Enter` to paste)
- Global shortcut to open Recall from anywhere (default: `CmdOrCtrl+Shift+V`)
- Image preview via canvas (RGBA8 encoded as base64)
- System tray icon with contextual menu (Linux, macOS, Windows)
- Settings window: configurable shortcut, history size, autostart
- Autostart at login via `tauri-plugin-autostart`
- Snap packaging for Linux distribution
