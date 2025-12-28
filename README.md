# Zaxiom

A native terminal for Windows with full PTY support, built in Rust.

```
    ███████╗ █████╗ ██╗  ██╗██╗ ██████╗ ███╗   ███╗
    ╚══███╔╝██╔══██╗╚██╗██╔╝██║██╔═══██╗████╗ ████║
      ███╔╝ ███████║ ╚███╔╝ ██║██║   ██║██╔████╔██║
     ███╔╝  ██╔══██║ ██╔██╗ ██║██║   ██║██║╚██╔╝██║
    ███████╗██║  ██║██╔╝ ██╗██║╚██████╔╝██║ ╚═╝ ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═╝
```

## Overview

Zaxiom is a GPU-accelerated terminal that runs natively on Windows. It combines a built-in shell with full PTY support via ConPTY, allowing you to run interactive CLI tools (vim, ssh, node REPLs, etc.) alongside fast native commands.

**Key capabilities:**
- Full VT100/ANSI terminal emulation with UTF-8 support
- Hybrid execution: native Rust commands for speed, PTY for interactive tools
- Multi-tab and split pane support
- Intelligent autocomplete with git branch and flag awareness
- 20 built-in color themes
- Session persistence

## Installation

### Windows

```powershell
git clone https://github.com/aayushadhikari7/zaxiom
cd zaxiom
.\install.ps1
```

The installer builds the release binary and creates Start Menu/Desktop shortcuts.

### Build from source

```bash
cargo build --release
```

## Features

### Terminal
- **PTY support** - ConPTY integration for interactive applications
- **Tabs & splits** - Ctrl+T/W for tabs, Ctrl+Shift+D/E for splits
- **Autocomplete** - Commands, paths, git branches, flags
- **History** - Context-aware with fuzzy search (Ctrl+R)
- **Search** - Ctrl+F to search output
- **Themes** - `theme <name>` to switch (dracula, nord, gruvbox, tokyo-night, etc.)

### Built-in Commands
Standard shell commands implemented in Rust:
- Navigation: `ls`, `cd`, `pwd`, `tree`
- Files: `cat`, `cp`, `mv`, `rm`, `mkdir`, `touch`, `chmod`
- Text: `grep`, `find`, `head`, `tail`, `wc`, `sort`, `sed`, `awk`
- System: `ps`, `kill`, `df`, `du`, `whoami`, `uname`
- Network: `curl`, `wget`, `ping`
- Compression: `tar`, `zip`, `gzip`
- Hash: `md5sum`, `sha256sum`, `blake3sum`, `base64`

### External Tools
Seamlessly run external development tools:
- npm, yarn, pnpm, bun, node, deno
- cargo, rustc, go, python, pip
- docker, kubectl, terraform
- git, gh, ssh
- And more

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+T | New tab |
| Ctrl+W | Close tab/pane |
| Ctrl+Tab | Next tab |
| Ctrl+Shift+D | Split horizontal |
| Ctrl+Shift+E | Split vertical |
| Alt+Arrows | Navigate panes |
| Ctrl+R | Fuzzy search history |
| Ctrl+F | Search output |
| Tab | Autocomplete |

### Special Modes

- **Vi mode** (Ctrl+Shift+M) - Vim-style navigation in scrollback
- **Hints mode** (Ctrl+Shift+H) - Extract URLs, paths, git hashes
- **Fuzzy finder** (Ctrl+R/Ctrl+Shift+F/Ctrl+G) - Search history, files, branches

## Architecture

```
src/
├── app.rs           # Main application and UI
├── pty/             # PTY session, terminal grid, ANSI parsing
├── terminal/        # Buffer, history, autocomplete, splits
├── shell/           # Parser and executor
├── commands/        # Built-in command implementations
└── config/          # Themes and settings
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| eframe/egui | GPU-accelerated UI |
| portable-pty | PTY support (ConPTY) |
| syntect | Syntax highlighting |
| git2 | Git operations |
| reqwest | HTTP client |

## License

MIT

---

Built with Rust + egui
