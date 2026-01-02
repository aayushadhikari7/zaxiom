<div align="center">

# Zaxiom

**A modern terminal for Windows, built in Rust**

[![GitHub release](https://img.shields.io/github/v/release/aayushadhikari7/zaxiom?style=flat-square&color=green)](https://github.com/aayushadhikari7/zaxiom/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/aayushadhikari7/zaxiom/total?style=flat-square)](https://github.com/aayushadhikari7/zaxiom/releases)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-b7410e?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)
[![Windows](https://img.shields.io/badge/Platform-Windows-0078D6?style=flat-square&logo=windows)](https://github.com/aayushadhikari7/zaxiom)

```
 ███████╗ █████╗ ██╗  ██╗██╗ ██████╗ ███╗   ███╗
 ╚══███╔╝██╔══██╗╚██╗██╔╝██║██╔═══██╗████╗ ████║
   ███╔╝ ███████║ ╚███╔╝ ██║██║   ██║██╔████╔██║
  ███╔╝  ██╔══██║ ██╔██╗ ██║██║   ██║██║╚██╔╝██║
 ███████╗██║  ██║██╔╝ ██╗██║╚██████╔╝██║ ╚═╝ ██║
 ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═╝
```

*Linux vibes on Windows* ✨

![Zaxiom Terminal](assets/zaxiom.png)

</div>

---

## 💡 Why Zaxiom?

Most Windows terminals feel clunky or lack proper Unix-style tooling. Zaxiom brings the best of both worlds:

- ⚡ **Blazingly fast** — 100+ commands implemented natively in Rust
- 🖥️ **Full PTY support** — Run vim, ssh, node REPLs, and interactive CLI tools seamlessly
- 🛠️ **Developer-friendly** — Git integration, intelligent autocomplete, syntax highlighting
- 🎨 **Beautiful** — 20 built-in themes and a kawaii robot companion

---

## 🚀 Quick Start

### Using Make (cross-platform)

```bash
git clone https://github.com/aayushadhikari7/zaxiom
cd zaxiom
make release      # Build optimized binary
make install      # Windows: install to system with shortcuts
```

### Using PowerShell (Windows)

```powershell
git clone https://github.com/aayushadhikari7/zaxiom
cd zaxiom
.\run\install.ps1
```

### Available Make Commands

| Command | Description |
|---------|-------------|
| `make build` | Debug build |
| `make release` | Optimized release build |
| `make run` | Build and run |
| `make install` | Install to system (Windows) |
| `make update` | Rebuild and update installed version |
| `make ci` | Run all checks (fmt, lint, test) |
| `make help` | Show all commands |

---

## ✨ Features

### Terminal Emulation
| | Feature | Description |
|--|---------|-------------|
| 🖥️ | **PTY Support** | Full ConPTY integration for interactive apps |
| 📑 | **Tabs & Splits** | Multi-pane workflow with keyboard shortcuts |
| 💬 | **Autocomplete** | Context-aware suggestions for commands, paths, git branches |
| 🔍 | **Fuzzy Search** | Ctrl+R for history, Ctrl+Shift+F for files, Ctrl+G for branches |
| ⌨️ | **Vi Mode** | Vim-style navigation in scrollback buffer |

### 📦 Built-in Commands

All your favorite Unix commands, implemented in Rust for speed:

| | Category | Commands |
|--|----------|----------|
| 📂 | Navigation | `ls` `cd` `pwd` `tree` `clear` |
| 📄 | Files | `cat` `cp` `mv` `rm` `mkdir` `touch` `chmod` `nano` |
| 📝 | Text | `grep` `find` `head` `tail` `wc` `sort` `sed` `awk` `cut` `diff` |
| 💻 | System | `ps` `kill` `df` `du` `whoami` `uname` `neofetch` |
| 🌐 | Network | `curl` `wget` `ping` |
| 🗜️ | Compression | `tar` `zip` `gzip` `gunzip` |
| 🔐 | Hash | `md5sum` `sha256sum` `blake3sum` `base64` |

### 🔧 External Tool Support

Seamlessly run your development tools with full TTY support:

| | Category | Tools |
|--|----------|-------|
| 🟨 | JavaScript | `npm` `yarn` `pnpm` `bun` `node` `deno` |
| 🦀 | Rust | `cargo` `rustc` `rustup` |
| 🐍 | Python | `python` `pip` `uv` `poetry` |
| 🐳 | Containers | `docker` `kubectl` `terraform` |
| 🔀 | Version Control | `git` `gh` `ssh` |
| 🤖 | AI Assistants | `aider` `gh copilot` |

### 🤖 AI Chat

Chat with AI directly from your terminal using the `#` prefix:

```bash
# explain what a hashmap is
# --claude write a rust function to reverse a string
# --gpt help me debug this error
# --deepseek optimize this code
```

**10 Providers Supported:**

| Provider | Flag | Default Model | Environment Variable |
|----------|------|---------------|---------------------|
| Ollama | `--ollama` | llama3.2 | *(local, no key)* |
| Groq | `--groq` | llama-3.3-70b-versatile | `GROQ_API_KEY` |
| OpenAI | `--gpt` | gpt-5.2 | `OPENAI_API_KEY` |
| Anthropic | `--claude` | claude-sonnet-4-5 | `ANTHROPIC_API_KEY` |
| Google Gemini | `--gemini` | gemini-2.5-flash | `GEMINI_API_KEY` |
| DeepSeek | `--deepseek` | deepseek-chat (V3.2) | `DEEPSEEK_API_KEY` |
| Mistral | `--mistral` | mistral-large-latest | `MISTRAL_API_KEY` |
| xAI Grok | `--grok` | grok-2-latest | `XAI_API_KEY` |
| Cohere | `--cohere` | command-r-plus | `COHERE_API_KEY` |
| Perplexity | `--pplx` | llama-3.1-sonar-large | `PERPLEXITY_API_KEY` |

**Easy Key Setup:** When you use a provider without a key configured, Zaxiom shows setup instructions with the signup URL and how to configure your key.

Set your preferred provider: `export AI_PROVIDER=openai`

---

## ⌨️ Keyboard Shortcuts

### Navigation
| Shortcut | Action |
|----------|--------|
| `Ctrl+T` | New tab |
| `Ctrl+W` | Close tab/pane |
| `Ctrl+Tab` | Next tab |
| `Ctrl+1-9` | Jump to tab |

### Splits
| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+D` | Split horizontal |
| `Ctrl+Shift+E` | Split vertical |
| `Alt+Arrows` | Navigate panes |

### Productivity
| Shortcut | Action |
|----------|--------|
| `Tab` | Autocomplete |
| `Ctrl+R` | Fuzzy search history |
| `Ctrl+F` | Search output |
| `Ctrl+Shift+M` | Vi mode |
| `Ctrl+Shift+H` | Hints mode (extract URLs, paths) |

---

## 🎨 Themes

Switch themes instantly with `theme <name>`:

| | | |
|--|--|--|
| 🌸 Catppuccin Mocha *(default)* | ❄️ Nord | 🧛 Dracula |
| 🌃 Tokyo Night | 🟤 Gruvbox | ⚫ One Dark |
| ☀️ Solarized | 🎨 Monokai Pro | 🌹 Rose Pine |
| 🌊 Kanagawa | 🌲 Everforest | 🦉 Night Owl |

Enable kawaii mode for extra flair: `theme --kawaii` ✨

---

## 🏗️ Architecture

```
zaxiom/
├── src/
│   ├── app.rs           # Main application and UI
│   ├── ai/              # Multi-provider AI integration
│   ├── pty/             # PTY session, terminal grid, ANSI parsing
│   ├── terminal/        # Buffer, history, autocomplete, splits
│   ├── shell/           # Parser and executor
│   ├── commands/        # Built-in command implementations
│   └── config/          # Themes and settings
├── run/
│   ├── install.ps1      # Windows installer script
│   └── update.ps1       # Quick update script
└── Makefile             # Cross-platform build commands
```

---

## 🤝 Contributing

Contributions are welcome! Feel free to:

- 🐛 Report bugs or request features via [Issues](https://github.com/aayushadhikari7/zaxiom/issues)
- 🔧 Submit pull requests
- 💬 Share feedback

---

## 🔮 Roadmap

Stay tuned for more updates and features! This project is actively developed and there's more to come.

---

## 📄 License

[MIT](LICENSE) — Built with 🦀 Rust + egui

See [CHANGELOG.md](CHANGELOG.md) for version history.
