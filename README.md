# Zaxiom

A modern, Linux-style terminal emulator for Windows, built entirely in Rust.

```
    ███████╗ █████╗ ██╗  ██╗██╗ ██████╗ ███╗   ███╗
    ╚══███╔╝██╔══██╗╚██╗██╔╝██║██╔═══██╗████╗ ████║
      ███╔╝ ███████║ ╚███╔╝ ██║██║   ██║██╔████╔██║
     ███╔╝  ██╔══██║ ██╔██╗ ██║██║   ██║██║╚██╔╝██║
    ███████╗██║  ██║██╔╝ ██╗██║╚██████╔╝██║ ╚═╝ ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═╝
```

## What is Zaxiom?

Zaxiom is a **terminal emulator** (not just a CLI tool) that brings the Linux terminal experience to Windows. Unlike tools like `git bash` or `WSL`, Zaxiom:

- Runs **natively** on Windows (no Linux subsystem needed)
- Implements shell commands **in pure Rust** (no external executables)
- Provides a **modern UI** with GPU-accelerated rendering via egui
- Features a **cute robot mascot** companion

### Terminal Emulator vs CLI Tool

| Terminal Emulator | CLI Tool |
|-------------------|----------|
| IS the window you type into | Runs INSIDE a terminal |
| Handles rendering, fonts, colors | Outputs text to stdout |
| Examples: iTerm2, Windows Terminal, Alacritty | Examples: git, npm, cargo |

**Zaxiom = Terminal Emulator + Built-in Shell**

## Features

### Core Terminal Features
- **Multi-tab support** - Ctrl+T (new), Ctrl+W (close), Ctrl+Tab (switch)
- **Split panes** - Ctrl+Shift+D (horizontal), Ctrl+Shift+E (vertical), Alt+Arrows (navigate)
- **Intelligent autocomplete** - Tab completion for commands, paths, git branches, flags
- **Smart command history** - Context-aware history with directory tracking, frequency-based suggestions, and fuzzy search
- **Scrollback buffer** - 10,000 lines with smooth scrolling
- **Block-based output** - Commands grouped as discrete blocks with copy buttons
- **Command timing** - Shows execution duration for each command
- **Clickable URLs** - URLs in output are clickable and open in browser
- **Syntax highlighting** - Code files displayed with syntax colors (40+ languages)
- **Clipboard integration** - Ctrl+C/V with visual feedback toast
- **Search** - Ctrl+F to search within terminal output
- **Session persistence** - Auto-saves and restores tabs on restart
- **Status bar** - Shows git branch, current directory, pane count, block count
- **Nerd Font icons** - Beautiful icons for folders, files, git status
- **20 built-in themes** - Catppuccin, Dracula, Nord, Gruvbox, Tokyo Night, and more
- **Theme persistence** - Theme choice saved to config file

### 160+ Commands (100 Native + 60 External Tools)

#### Navigation
`ls` `cd` `pwd` `tree` `clear` `help`

#### File Operations
`cat` `touch` `rm` `mkdir` `cp` `mv` `ln` `stat` `file` `chmod` `readlink` `mktemp`

#### Text Processing
`echo` `head` `tail` `wc` `sort` `uniq` `grep` `find` `cut` `paste` `diff` `tr` `sed` `awk` `rev` `nl` `printf` `xargs` `column` `strings` `split` `join` `comm`

#### System Info
`whoami` `hostname` `uname` `uptime` `free` `date` `cal` `id` `neofetch` `ps` `kill` `du` `df` `which` `printenv` `lscpu` `history` `man` `theme`

#### Network
`curl` `wget` `ping` `netstat` `traceroute` `nslookup` `host` `ifconfig`

#### Hash & Encoding
`md5sum` `sha1sum` `sha224sum` `sha256sum` `sha384sum` `sha512sum` `blake3sum` `crc32` `base64` `xxd`

#### Compression
`tar` `zip` `unzip` `gzip` `gunzip`

#### Shell Utilities
`alias` `env` `export` `sleep` `seq` `yes` `true` `false` `expr` `bc` `tee` `timeout` `type` `command` `pushd` `popd` `dirs`

#### Editors
`nano` `vim` `vi` `edit`

#### Fun Commands
`fortune` `cowsay` `coffee` `matrix` `neofetch` `pet`

#### AI Integration (Ollama)
`ollama` `# <prompt>` (AI chat with context)

## Command Usage Examples

### Navigation

```bash
# List files (with colors and icons)
ls
ls -la              # Long format, show hidden
ls -lhS             # Sort by size, human-readable

# Change directory
cd ~/Documents      # Go to Documents
cd ..               # Go up one level
cd -                # Go to previous directory

# Show directory tree
tree                # Current directory
tree -L 2           # Limit depth to 2 levels

# Print working directory
pwd
```

### File Operations

```bash
# View file contents (with syntax highlighting for code files)
cat file.txt
cat -n file.txt     # With line numbers
cat -s main.rs      # Force syntax highlighting
cat -p main.rs      # Plain text (no highlighting)

# Create files
touch newfile.txt
mkdir newfolder
mkdir -p path/to/nested/folder

# Copy, move, remove
cp source.txt dest.txt
cp -r folder/ backup/
mv old.txt new.txt
rm file.txt
rm -rf folder/      # Recursive force delete

# File info
stat file.txt       # Detailed file info
file image.png      # Detect file type
```

### Text Processing

```bash
# Search in files
grep "pattern" file.txt
grep -r "TODO" src/       # Recursive search
grep -i "error" log.txt   # Case insensitive
grep -n "func" *.rs       # With line numbers

# Find files
find . -name "*.rs"       # Find by name
find . -type f -size +1M  # Files larger than 1MB

# View parts of files
head -20 file.txt         # First 20 lines
tail -f log.txt           # Follow file (live)

# Count
wc file.txt               # Lines, words, chars
wc -l *.rs                # Count lines in Rust files

# Transform text
sort names.txt            # Sort lines
uniq duplicates.txt       # Remove duplicates
cut -d',' -f1 data.csv    # Extract first column
tr 'a-z' 'A-Z' < file     # Uppercase

# Diff files
diff file1.txt file2.txt
```

### System Information

```bash
# User & system
whoami
hostname
uname -a              # Full system info

# System stats
uptime                # How long system running
free -h               # Memory usage
df -h                 # Disk usage
du -sh folder/        # Folder size

# Processes
ps                    # List processes
kill 1234             # Kill process by PID

# Date & time
date
cal                   # Show calendar

# Full system info (with ASCII art)
neofetch
```

### Themes

```bash
# List all available themes
theme
theme list

# Switch theme (saved to config file)
theme dracula
theme nord
theme gruvbox-dark
theme tokyo-night
theme one-dark
theme catppuccin-mocha   # Default theme

# Available themes:
# catppuccin-mocha, catppuccin-latte, dracula, nord,
# gruvbox-dark, gruvbox-light, tokyo-night, tokyo-night-storm,
# one-dark, solarized-dark, solarized-light, monokai-pro,
# palenight, ayu-dark, ayu-mirage, kanagawa, rose-pine,
# rose-pine-moon, everforest-dark, night-owl
```

### Hash & Encoding

```bash
# Hash files
md5sum file.txt
sha256sum file.txt
blake3sum file.txt    # Fastest, recommended

# Hash from pipe
echo "hello" | sha256sum

# Base64 encoding
base64 file.txt           # Encode
base64 -d encoded.txt     # Decode

# Hex dump
xxd file.bin
```

### Compression

```bash
# Tar archives
tar -cvf archive.tar folder/     # Create
tar -xvf archive.tar             # Extract
tar -czvf archive.tar.gz folder/ # Create gzipped

# Zip files
zip -r archive.zip folder/
unzip archive.zip

# Gzip
gzip file.txt           # Compress (replaces original)
gunzip file.txt.gz      # Decompress
```

### Network

```bash
# HTTP requests
curl https://api.example.com
curl -o file.zip https://example.com/file.zip
wget https://example.com/file.zip

# Network diagnostics
ping google.com
traceroute google.com
nslookup example.com

# Network info
netstat
ifconfig
```

### Git Shortcuts

```bash
# Built-in git shortcuts
gs          # git status
gd          # git diff
gl          # git log --oneline -20
gp          # git push
ga          # git add
gc          # git commit
```

### External Development Tools

Zaxiom wraps 60+ external development tools for seamless integration:

```bash
# Node.js ecosystem
npm install           # Package management
npx create-react-app  # Run npm packages
yarn add package      # Yarn package manager
pnpm install          # Fast package manager
bun run dev           # Bun runtime
node script.js        # Node runtime
deno run file.ts      # Deno runtime

# Python ecosystem
python script.py      # Python interpreter
pip install package   # Package manager
uv pip install        # Fast pip alternative
poetry install        # Dependency management

# Rust ecosystem
cargo build           # Build Rust projects
cargo run             # Run Rust projects
rustc file.rs         # Rust compiler
rustup update         # Toolchain manager

# Go
go build              # Build Go projects
go run main.go        # Run Go code

# C/C++ toolchain
gcc main.c            # C compiler
g++ main.cpp          # C++ compiler
clang file.c          # LLVM C compiler
clang++ file.cpp      # LLVM C++ compiler
make                  # Build automation
cmake .               # CMake build system

# Debugging & analysis
gdb ./program         # GNU debugger
lldb ./program        # LLVM debugger
valgrind ./program    # Memory analysis
nm binary             # Symbol listing
objdump -d binary     # Disassembly

# Java/JVM
java -jar app.jar     # Java runtime
javac Main.java       # Java compiler
mvn package           # Maven build
gradle build          # Gradle build
scala script.scala    # Scala
kotlin file.kt        # Kotlin

# .NET
dotnet build          # .NET CLI
dotnet run            # Run .NET apps

# Containers & orchestration
docker build .        # Build images
docker run image      # Run containers
kubectl get pods      # Kubernetes CLI

# Version control
git status            # Full git access

# Cloud & DevOps
aws s3 ls             # AWS CLI
az login              # Azure CLI
gcloud info           # Google Cloud
terraform plan        # Infrastructure
ansible-playbook      # Automation

# Code editors (opens external)
code .                # VS Code
cursor .              # Cursor editor

# Linters & formatters
prettier --write .    # Code formatter
eslint src/           # JS linter
black .               # Python formatter
ruff check .          # Fast Python linter

# Testing
pytest                # Python tests
jest                  # JS tests
vitest                # Vite tests

# Other languages
ruby script.rb        # Ruby
php script.php        # PHP
lua script.lua        # Lua
julia script.jl       # Julia
swift build           # Swift
zig build             # Zig
ghc file.hs           # Haskell
elixir script.exs     # Elixir
```

### Shell Utilities

```bash
# Aliases
alias ll='ls -la'
alias gs='git status'

# Environment
env                   # Show all variables
export MY_VAR=value   # Set variable
printenv PATH         # Print specific var

# Directory stack
pushd /tmp            # Push and change
popd                  # Pop and return
dirs                  # Show stack

# Misc
sleep 5               # Wait 5 seconds
seq 1 10              # Print 1 to 10
yes | head -5         # Print "y" 5 times
expr 5 + 3            # Calculate
bc                    # Calculator
```

### Fun Commands

```bash
# Random programming quote
fortune

# ASCII art cow
cowsay "Hello World!"
cowsay -f robot "Beep boop!"
fortune | cowsay      # Combine them!

# ASCII coffee
coffee
coffee --espresso
coffee --tea

# Matrix digital rain
matrix

# System info with robot mascot
neofetch

# Interact with your kawaii robot companion!
pet              # Give pets
pet hug          # Give a warm hug
pet boop         # Boop the nose sensor
pet feed         # Feed some electricity
pet love         # Express your love!
```

### Editors

```bash
# Open file in built-in nano editor
nano file.txt
vim file.txt
edit file.txt

# Editor controls:
# Ctrl+S - Save file
# Ctrl+X - Exit editor
# Arrow keys - Navigate
```

### AI Chat (Ollama Integration)

```bash
# Quick AI chat with # prefix (context-aware)
# explain what a hashmap is
# how do I list files in Linux?
# write a function to reverse a string

# Ollama command for model management
ollama list            # List installed models
ollama pull llama3.2   # Download a model
ollama run llama3.2    # Chat with specific model
ollama serve           # Start Ollama server
ollama status          # Check server status
ollama models          # Show recommended models
ollama --help          # Full help

# The AI has context about:
# - Your current directory
# - Your recent commands
# - Your operating system
```

### Command Help

```bash
# Get comprehensive help for any command
ls --help              # Detailed usage, examples, tips
grep --help            # Pattern matching guide
tar --help             # Archive recipes (remember -xzvf!)
git --help             # Complete beginner guide with workflow

# Alternative: use man command
man ls
man grep

# Help includes:
# - Usage syntax
# - All available options
# - Real-world examples
# - Common mistakes to avoid
# - Related commands
```

**50+ Commands with Detailed Help:**
- **File ops**: `cp`, `mv`, `rm`, `mkdir`, `touch`, `chmod`
- **Text processing**: `awk`, `tr`, `uniq`, `cut`, `diff`, `sed`, `sort`
- **Hashes**: `md5sum`, `sha256sum`, `sha512sum`, `blake3sum`, `crc32`, `base64`, `xxd`
- **System**: `ps`, `kill`, `df`, `du`, `uptime`, `whoami`, `uname`, `date`
- **Network**: `curl`, `wget`, `ping`
- **Compression**: `tar`, `zip`, `gzip`, `gunzip`
- **Navigation**: `ls`, `tree`, `pwd`, `clear`, `find`
- **Shell**: `alias`, `env`, `export`
- **Git**: Complete beginner workflow guide!

### Piping & Redirection

```bash
# Pipe output between commands
ls -la | grep ".rs"
cat file.txt | sort | uniq
ps | grep "node"

# Redirect output
ls > files.txt           # Overwrite
ls >> files.txt          # Append
grep "error" log.txt 2>&1  # Combine stderr
```

## Special Modes

### Hints Mode (Ctrl+Shift+H)

Extracts clickable/copyable items from terminal output:
- **URLs** - http/https links
- **File paths** - Absolute and relative paths
- **Git hashes** - Commit hashes (7-40 hex chars)
- **Emails** - Email addresses
- **Line references** - `file.rs:42` patterns

Press Escape to exit hints mode.

### Fuzzy Finder (Ctrl+R / Ctrl+Shift+F / Ctrl+G)

fzf-like fuzzy search for history, files, and git branches:
- **Ctrl+R** - Search command history
- **Ctrl+Shift+F** - Search files in current directory
- **Ctrl+G** - Search git branches
- **Type** - Filter results with fuzzy matching
- **Up/Down** - Navigate results
- **Enter** - Insert selected into command line
- **Ctrl+Enter** - Execute selected immediately
- **Escape** - Close fuzzy finder

Features:
- Match highlighting shows which characters matched
- Smart scoring (exact > prefix > contains > fuzzy)
- File search respects common ignore patterns (node_modules, target, .git)
- Git branch search shows both local and remote branches

### Vi Mode (Ctrl+Shift+M)

Vim-style navigation for terminal scrollback:
- **hjkl** - Navigate (left/down/up/right)
- **gg/G** - Jump to top/bottom
- **Ctrl+U/D** - Half page up/down
- **Ctrl+B/F** - Full page up/down
- **v** - Visual selection mode
- **/? ** - Search forward/backward
- **n/N** - Next/previous search match
- **y** - Yank (copy) selection
- **q/Escape** - Exit vi mode

## Keyboard Shortcuts

### Tabs
| Shortcut | Action |
|----------|--------|
| `Ctrl+T` | New tab |
| `Ctrl+W` | Close tab (or pane if multiple) |
| `Ctrl+Tab` | Next tab |
| `Ctrl+Shift+Tab` | Previous tab |
| `Ctrl+1-9` | Switch to tab 1-9 |

### Split Panes
| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+D` | Split horizontally |
| `Ctrl+Shift+E` | Split vertically |
| `Alt+←` / `Alt+→` | Navigate between panes |

### Input & History
| Shortcut | Action |
|----------|--------|
| `↑` / `↓` | Navigate command history |
| `Tab` | Apply autocomplete suggestion |
| `Escape` | Close autocomplete / search / hints / vi mode |
| `Ctrl+F` | Search in terminal |
| `Enter` / `Shift+Enter` | Next / previous search match |

### Special Modes
| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+H` | Toggle hints mode (extract URLs, paths, hashes) |
| `Ctrl+Shift+M` | Toggle vi mode (vim-style navigation) |

### Fuzzy Finder
| Shortcut | Action |
|----------|--------|
| `Ctrl+R` | Fuzzy search history |
| `Ctrl+Shift+F` | Fuzzy search files |
| `Ctrl+G` | Fuzzy search git branches |
| `↑` / `↓` | Navigate results |
| `Enter` | Insert selected into command line |
| `Ctrl+Enter` | Execute selected immediately |
| `Escape` | Close fuzzy finder |

### Clipboard & Line Editing
| Shortcut | Action |
|----------|--------|
| `Ctrl+C` | Interrupt (clears current line, shows ^C) |
| `Ctrl+Shift+C` | Copy current input line |
| `Ctrl+V` | Paste (strips newlines for single-line input) |
| `Ctrl+Shift+V` | Paste raw (preserves newlines) |
| `Ctrl+L` | Clear screen |
| `Ctrl+U` | Clear line (delete all input) |
| `Alt+.` | Insert last argument from previous command |

### History Expansion
| Syntax | Expands To |
|--------|------------|
| `!!` | Last command (e.g., `sudo !!`) |
| `!n` | nth command in history (e.g., `!5`) |
| `!-n` | nth-from-last command (e.g., `!-2`) |

## Architecture

```
zaxiom/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Main application & UI (egui)
│   ├── commands/            # All built-in commands
│   │   ├── nav/             # ls, cd, pwd, tree, clear, help
│   │   ├── files/           # cat, touch, rm, mkdir, cp, mv, etc.
│   │   ├── text/            # echo, head, tail, grep, sed, awk, etc.
│   │   ├── search/          # grep, find
│   │   ├── system/          # whoami, ps, kill, neofetch, man, etc.
│   │   ├── net/             # curl, wget, ping, etc.
│   │   ├── hash/            # md5sum, sha256sum, blake3sum, etc.
│   │   ├── compress/        # tar, zip, gzip, etc.
│   │   ├── shell/           # alias, env, export, etc.
│   │   ├── fun/             # fortune, cowsay, coffee, matrix
│   │   ├── tools.rs         # 60+ external dev tool wrappers (npm, cargo, docker, etc.)
│   │   ├── registry.rs      # Command registration
│   │   └── traits.rs        # Command trait definition
│   ├── terminal/            # Terminal emulation layer
│   │   ├── buffer.rs        # Output buffer with blocks & URL detection
│   │   ├── history.rs       # Basic command history
│   │   ├── smart_history.rs # Context-aware history with fuzzy search
│   │   ├── state.rs         # Terminal state (cwd, aliases, etc.)
│   │   ├── autocomplete.rs  # Intelligent autocomplete (commands, paths, git, flags)
│   │   ├── split.rs         # Split pane management (horizontal/vertical)
│   │   ├── session.rs       # Session persistence (autosave/restore)
│   │   ├── syntax.rs        # Syntax highlighting (syntect)
│   │   ├── ansi.rs          # ANSI escape code parser
│   │   ├── hints.rs         # Smart text extraction (URLs, paths, git hashes)
│   │   ├── vi_mode.rs       # Vim-style terminal navigation
│   │   ├── fuzzy.rs         # Fuzzy finder (fzf-like search)
│   │   └── img.rs           # Inline image display support
│   ├── shell/               # Shell parsing & execution
│   │   ├── parser.rs        # Command parsing (pipes, redirects)
│   │   └── executor.rs      # Command execution
│   ├── config/              # Configuration
│   │   ├── theme.rs         # 20 built-in themes (Catppuccin, Dracula, Nord, etc.)
│   │   └── settings.rs      # Config file management (~/.config/zaxiom/config.toml)
│   ├── git/                 # Git integration
│   │   └── prompt.rs        # Git branch detection
│   └── mascot/              # Robot mascot
│       └── mod.rs           # Mascot animations & reactions
├── assets/
│   └── fonts/               # Nerd Font for icons
└── Cargo.toml               # Dependencies
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `eframe` / `egui` | GUI framework (GPU-accelerated) |
| `tokio` | Async runtime |
| `serde` / `toml` | Configuration |
| `regex` | Pattern matching |
| `walkdir` | Directory traversal |
| `git2` | Git operations |
| `reqwest` | HTTP client (curl/wget) |
| `blake3` | BLAKE3 hashing |
| `sha2` / `md-5` / `sha1` | SHA/MD5 hashing |
| `flate2` / `tar` / `zip` | Compression |
| `chrono` | Date/time |
| `sysinfo` | System information |
| `image` | Image processing |
| `syntect` | Syntax highlighting (40+ languages) |
| `arboard` | Cross-platform clipboard |
| `open` | Open URLs in default browser |

## Installation

### Quick Install (Windows)

```powershell
# Clone and install
git clone https://github.com/jacky/zaxiom
cd zaxiom

# Run the installer (builds + creates shortcuts)
.\install.ps1
```

The installer will:
1. Build the release binary
2. Install to `%LOCALAPPDATA%\Zaxiom\`
3. Create Start Menu and Desktop shortcuts
4. Optionally launch the app

### Updating After Changes

```powershell
# Quick update (rebuild + copy to install location)
.\update.ps1
```

### Uninstalling

```powershell
.\install.ps1 -Uninstall
```

## Building (Development)

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run
cargo run
```

## Themes

Zaxiom includes **20 built-in themes** with the default being **Catppuccin Mocha**:

| Theme | Description |
|-------|-------------|
| `catppuccin-mocha` | Modern pastel dark (default) |
| `catppuccin-latte` | Modern pastel light |
| `dracula` | Popular dark theme |
| `nord` | Arctic, bluish colors |
| `gruvbox-dark` | Retro groove colors |
| `gruvbox-light` | Retro groove light |
| `tokyo-night` | Clean dark theme |
| `tokyo-night-storm` | Darker Tokyo Night |
| `one-dark` | Atom-inspired dark |
| `solarized-dark` | Ethan Schoonover's dark |
| `solarized-light` | Ethan Schoonover's light |
| `monokai-pro` | Sublime Text inspired |
| `palenight` | Material palenight |
| `ayu-dark` | Ayu dark theme |
| `ayu-mirage` | Ayu mirage theme |
| `kanagawa` | Inspired by Katsushika Hokusai |
| `rose-pine` | Soho vibes |
| `rose-pine-moon` | Rose Pine darker |
| `everforest-dark` | Comfortable green |
| `night-owl` | For night owls |

Use `theme <name>` to switch. Your choice is saved to `~/.config/zaxiom/config.toml`.

## Inspired By

- [Warp](https://warp.dev) - Block-based output, AI features
- [Kitty](https://sw.kovidgoyal.net/kitty/) - GPU rendering, kittens
- [Alacritty](https://alacritty.org/) - Performance, Vi mode
- [WezTerm](https://wezfurlong.org/wezterm/) - Multiplexing, Lua config
- [Hyper](https://hyper.is/) - Plugin ecosystem, themes

## License

MIT

---

*Built with Rust + egui — blazingly fast!*
