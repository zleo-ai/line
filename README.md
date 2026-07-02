# Horus

[English](README.md) | [中文](README.zh.md)

A high-performance Claude Code statusline tool written in Rust with Git integration, usage tracking, interactive TUI configuration, and Claude Code enhancement utilities.

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)

> **Fork of [Haleclipse/CCometixLine](https://github.com/Haleclipse/CCometixLine)** — huge thanks to the original author for the elegant Rust statusline foundation. Horus diverges by adding **Codex usage tracking** (reads `~/.codex/sessions/` rate limits) alongside Claude usage, and splits the original `Usage` segment into separate `HourlyUsage` / `WeeklyUsage` views for finer-grained budget pacing. Named after the Egyptian falcon god — the All-Seeing Eye for your Claude + Codex sessions.

## Screenshots

![Horus](assets/img1.png)

The statusline shows: Model | Directory | Git Branch Status | Context Window Information

## Features

### Core Functionality
- **Git integration** with branch, status, and tracking info  
- **Model display** with simplified Claude model names
- **Usage tracking** based on transcript analysis
- **Directory display** showing current workspace
- **Minimal design** using Nerd Font icons

### Interactive TUI Features
- **Interactive main menu** when executed without input
- **TUI configuration interface** with real-time preview
- **Theme system** with multiple built-in presets
- **Segment customization** with granular control
- **Configuration management** (init, check, edit)

### Claude Code Enhancement
- **Context warning disabler** - Remove annoying "Context low" messages
- **Verbose mode enabler** - Enhanced output detail
- **Robust patcher** - Survives Claude Code version updates
- **Automatic backups** - Safe modification with easy recovery

## Installation

### Quick Install (Recommended)

Install via npm (works on all platforms):

```bash
# Install globally
npm install -g @zleo-ai/horus

# Or using yarn
yarn global add @zleo-ai/horus

# Or using pnpm
pnpm add -g @zleo-ai/horus
```

Use npm mirror for faster download:
```bash
npm install -g @zleo-ai/horus --registry https://registry.npmmirror.com
```

After installation:
- ✅ Global command `horus` is available everywhere
- ⚙️ Follow the configuration steps below to integrate with Claude Code
- 🎨 Run `horus -c` to open configuration panel for theme selection

### Claude Code Configuration

Add to your Claude Code `settings.json`:

**Cross-Platform (Recommended)**
```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/horus/horus",
    "padding": 0
  }
}
```

> **Note for Windows users:** Starting from Claude Code v2.1.47+, Unix-style path parsing is supported on Windows. The `~` symbol is automatically expanded to your user home directory. **Do not use `%USERPROFILE%`** - it no longer works reliably in v2.1.47+.
> - Recommended: `~/.claude/horus/horus` (works on all platforms)
> - Alternative: `"horus"` (requires npm global installation)

**Fallback (npm installation):**
```json
{
  "statusLine": {
    "type": "command",
    "command": "horus",
    "padding": 0
  }
}
```
*Use this if npm global installation is available in PATH*

### Update

```bash
npm update -g @zleo-ai/horus
```

<details>
<summary>Manual Installation (Click to expand)</summary>

Alternatively, download from [Releases](https://github.com/zleo-ai/horus/releases):

#### Linux

#### Option 1: Dynamic Binary (Recommended)
```bash
mkdir -p ~/.claude/horus
wget https://github.com/zleo-ai/horus/releases/latest/download/horus-linux-x64.tar.gz
tar -xzf horus-linux-x64.tar.gz
cp horus ~/.claude/horus/
chmod +x ~/.claude/horus/horus
```
*Requires: Ubuntu 22.04+, CentOS 9+, Debian 11+, RHEL 9+ (glibc 2.35+)*

#### Option 2: Static Binary (Universal Compatibility)
```bash
mkdir -p ~/.claude/horus
wget https://github.com/zleo-ai/horus/releases/latest/download/horus-linux-x64-static.tar.gz
tar -xzf horus-linux-x64-static.tar.gz
cp horus ~/.claude/horus/
chmod +x ~/.claude/horus/horus
```
*Works on any Linux distribution (static, no dependencies)*

#### macOS (Intel)

```bash  
mkdir -p ~/.claude/horus
wget https://github.com/zleo-ai/horus/releases/latest/download/horus-macos-x64.tar.gz
tar -xzf horus-macos-x64.tar.gz
cp horus ~/.claude/horus/
chmod +x ~/.claude/horus/horus
```

#### macOS (Apple Silicon)

```bash
mkdir -p ~/.claude/horus  
wget https://github.com/zleo-ai/horus/releases/latest/download/horus-macos-arm64.tar.gz
tar -xzf horus-macos-arm64.tar.gz
cp horus ~/.claude/horus/
chmod +x ~/.claude/horus/horus
```

#### Windows

```powershell
# Create directory and download
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\horus"
Invoke-WebRequest -Uri "https://github.com/zleo-ai/horus/releases/latest/download/horus-windows-x64.zip" -OutFile "horus-windows-x64.zip"
Expand-Archive -Path "horus-windows-x64.zip" -DestinationPath "."
Move-Item "horus.exe" "$env:USERPROFILE\.claude\horus\"
```

</details>

### Build from Source

```bash
git clone https://github.com/zleo-ai/horus.git
cd Horus
cargo build --release

# Linux/macOS
mkdir -p ~/.claude/horus
cp target/release/horus ~/.claude/horus/horus
chmod +x ~/.claude/horus/horus

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\horus"
copy target\release\horus.exe "$env:USERPROFILE\.claude\horus\horus.exe"
```

## Usage

### Theme Override

```bash
# Temporarily use specific theme (overrides config file)
horus --theme cometix
horus --theme minimal
horus --theme gruvbox
horus --theme nord
horus --theme powerline-dark

# Or use custom theme files from ~/.claude/horus/themes/
horus --theme my-custom-theme
```

### Claude Code Enhancement

```bash
# Disable context warnings and enable verbose mode
horus --patch /path/to/claude-code/cli.js

# Example for common installation
horus --patch ~/.local/share/fnm/node-versions/v24.4.1/installation/lib/node_modules/@anthropic-ai/claude-code/cli.js
```

## Default Segments

Displays: `Directory | Git Branch Status | Model | Context Window`

### Git Status Indicators

- Branch name with Nerd Font icon
- Status: `✓` Clean, `●` Dirty, `⚠` Conflicts  
- Remote tracking: `↑n` Ahead, `↓n` Behind

### Model Display

Shows simplified Claude model names:
- `claude-3-5-sonnet` → `Sonnet 3.5`
- `claude-4-sonnet` → `Sonnet 4`

### Context Window Display

Token usage percentage based on transcript analysis with context limit tracking.

## Configuration

Horus supports full configuration via TOML files and interactive TUI:

- **Configuration file**: `~/.claude/horus/config.toml`
- **Interactive TUI**: `horus --config` for real-time editing with preview
- **Theme files**: `~/.claude/horus/themes/*.toml` for custom themes
- **Automatic initialization**: `horus --init` creates default configuration

### Available Segments

All segments are configurable with:
- Enable/disable toggle
- Custom separators and icons
- Color customization
- Format options

Supported segments: Directory, Git, Model, Usage, Time, Cost, OutputStyle

### Model Configuration (`models.toml`)

Location: `~/.claude/horus/models.toml` (auto-created on first run)

This file configures how model IDs are displayed and their context window limits. Claude models (Sonnet, Opus, Haiku) are automatically recognized with version extraction — you only need this file for overrides or third-party models.

```toml
# Model entries: simple substring matching on the model ID
# These take priority over built-in Claude model recognition
[[models]]
pattern = "glm-4.5"
display_name = "GLM-4.5"
context_limit = 128000

[[models]]
pattern = "kimi-k2"
display_name = "Kimi K2"
context_limit = 128000

# Context modifiers: matched independently and composable with model entries
# Overrides context_limit and appends display_suffix to the display name
# e.g., model "Opus 4" + modifier " 1M" = "Opus 4 1M"
[[context_modifiers]]
pattern = "[1m]"
display_suffix = " 1M"
context_limit = 1000000
```


## Requirements

- **Git**: Version 1.5+ (Git 2.22+ recommended for better branch detection)
- **Terminal**: Must support Nerd Fonts for proper icon display
  - Install a [Nerd Font](https://www.nerdfonts.com/) (e.g., FiraCode Nerd Font, JetBrains Mono Nerd Font)
  - Configure your terminal to use the Nerd Font
- **Claude Code**: For statusline integration

## Development

```bash
# Build development version
cargo build

# Run tests
cargo test

# Build optimized release
cargo build --release
```

## Roadmap

- [x] TOML configuration file support
- [x] TUI configuration interface
- [x] Custom themes
- [x] Interactive main menu
- [x] Claude Code enhancement tools

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Related Projects

- [tweakcc](https://github.com/Piebald-AI/tweakcc) - Command-line tool to customize your Claude Code themes, thinking verbs, and more.

## License

This project is licensed under the [MIT License](LICENSE).

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=zleo-ai/horus&type=Date)](https://star-history.com/#zleo-ai/horus&Date)
