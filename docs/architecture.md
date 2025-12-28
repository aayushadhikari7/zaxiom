# Zaxiom Architecture

## Overview

Zaxiom is a modern, Linux-style terminal emulator for Windows built entirely in Rust using egui. It features a Catppuccin Mocha theme, split panes, intelligent autocomplete, 100+ native commands, and 60+ external dev tool wrappers.

## Project Structure

```
zaxiom/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Main application (egui + pane rendering)
│   │
│   ├── terminal/            # Terminal emulator layer
│   │   ├── mod.rs           # Module exports
│   │   ├── state.rs         # Terminal state (cwd, env, prev_cwd)
│   │   ├── buffer.rs        # Output buffer + scrollback + URL detection + blocks
│   │   ├── history.rs       # Command history with navigation
│   │   ├── autocomplete.rs  # Intelligent autocomplete engine
│   │   ├── split.rs         # Split pane tree management
│   │   ├── session.rs       # Session persistence (autosave/restore)
│   │   ├── syntax.rs        # Syntax highlighting (syntect, 40+ languages)
│   │   ├── ansi.rs          # ANSI escape code parser (colors, bold, italic)
│   │   ├── img.rs           # Inline image display (ASCII art)
│   │   ├── hints.rs         # Smart text extraction (URLs, paths, hashes)
│   │   ├── smart_history.rs # Context-aware history with fuzzy search
│   │   ├── vi_mode.rs       # Vim-style terminal navigation
│   │   ├── fuzzy.rs         # Fuzzy finder (fzf-like search)
│   │   ├── input.rs         # Input handling
│   │   └── render.rs        # Rendering utilities
│   │
│   ├── pty/                 # PTY (Pseudo-Terminal) support
│   │   ├── mod.rs           # Module exports
│   │   ├── session.rs       # PtySession - ConPTY spawning & I/O
│   │   ├── buffer.rs        # PtyBuffer - streaming output
│   │   ├── grid.rs          # TerminalGrid - VT100/ANSI cell-based rendering
│   │   └── input.rs         # InputMode - raw key-to-bytes conversion
│   │
│   ├── shell/               # Shell engine
│   │   ├── parser.rs        # Command parsing (pipes, redirects, quotes)
│   │   └── executor.rs      # Hybrid command execution (native/external/PTY)
│   │
│   ├── commands/            # 160+ Commands (100 native + 60 external)
│   │   ├── nav/             # ls, cd, pwd, tree, clear, help
│   │   ├── files/           # cat, touch, rm, mkdir, cp, mv, chmod, nano, etc.
│   │   ├── text/            # echo, head, tail, grep, sed, awk, sort, etc.
│   │   ├── search/          # grep, find
│   │   ├── net/             # curl, wget, ping, traceroute, netstat
│   │   ├── system/          # whoami, ps, kill, neofetch, man, etc.
│   │   ├── hash/            # md5sum, sha256sum, blake3sum, etc.
│   │   ├── compress/        # tar, zip, gzip, gunzip
│   │   ├── shell/           # alias, env, export, pushd, popd
│   │   ├── fun/             # fortune, cowsay, coffee, matrix, pet
│   │   ├── ai.rs            # Ollama AI integration (# chat, ollama command)
│   │   ├── tools.rs         # 60+ external dev tool wrappers
│   │   ├── registry.rs      # Command registration & help lookup
│   │   └── traits.rs        # Command trait (name, usage, extended_help)
│   │
│   ├── git/                 # Git integration
│   │   └── prompt.rs        # Git branch detection for prompt
│   │
│   ├── config/              # Configuration
│   │   ├── theme.rs         # 20 built-in themes + Nerd Font icons
│   │   └── settings.rs      # Config persistence (~/.config/zaxiom/config.toml)
│   │
│   └── mascot/              # Robot mascot
│       └── mod.rs           # Mascot state machine (14 moods, animations, reactions)
│
├── assets/
│   └── fonts/               # Hurmit Nerd Font Mono
├── docs/
│   └── architecture.md      # This file
└── Cargo.toml               # Dependencies
```

## Application Architecture

### Tab and Pane Structure

```
ZaxiomApp
├── tabs: Vec<TabSession>     # Multiple terminal tabs
├── active_tab: usize         # Currently focused tab
├── executor: Executor        # Shared command executor
├── theme: Theme              # Catppuccin Mocha colors
├── mascot: Mascot            # Robot companion
└── autocomplete: Autocomplete # Shared autocomplete engine

TabSession
├── id: usize                 # Unique tab ID
├── title: String             # Tab title (from focused pane's cwd)
├── splits: SplitManager      # Split pane tree
└── panes: HashMap<usize, PaneSession>  # Pane ID -> Session

PaneSession
├── state: TerminalState      # cwd, env vars, aliases
├── buffer: OutputBuffer      # Scrollback buffer
├── history: SmartHistory     # Context-aware command history
├── input: String             # Current input line
├── saved_input: String       # Saved input during history navigation
├── suggestions: Vec<Suggestion>  # Autocomplete suggestions
├── fuzzy_finder: FuzzyFinder # fzf-like fuzzy search
├── pty_session: Option<PtySession>  # Active PTY session for interactive commands
├── pty_grid: TerminalGrid    # VT100/ANSI terminal grid for PTY output
├── pty_mode: bool            # Whether PTY is active
└── search_*                  # Search state
```

### Split Pane Tree

```
SplitManager
├── root: SplitNode           # Tree of splits
├── focused_pane: usize       # Currently focused pane ID
└── next_id: usize            # Next pane ID to allocate

SplitNode (enum)
├── Pane(Pane)                # Leaf: single terminal pane
└── Split {
    direction: Horizontal | Vertical,
    ratio: f32,               # Split ratio (0.0 - 1.0)
    first: Box<SplitNode>,
    second: Box<SplitNode>
}
```

## Command Routing Pipeline

```
User Input
    │
    ▼
┌─────────────────┐
│   Autocomplete  │  ← Suggest commands, paths, flags, git branches
└─────────────────┘
    │ (Tab to apply)
    ▼
┌─────────────────┐
│     Parser      │  ← Tokenize, handle quotes, escapes, pipes
└─────────────────┘
    │
    ▼
┌─────────────────┐
│     Router      │  ← Hybrid: Native? External? PTY?
└─────────────────┘
    │
    ├── Native Command ──► Execute Rust implementation (fast, no process spawn)
    ├── External Tool ──► Spawn system process (npm, cargo, docker, etc.)
    ├── PTY Mode ──► Route through ConPTY for full terminal emulation
    ├── User Alias ──► Expand & re-route
    └── Git Shortcut ──► Map to git command
            │
            ▼
┌─────────────────┐
│    Executor     │  ← Run command, capture output
└─────────────────┘
    │
    ▼
┌─────────────────────────────────┐
│  Buffer (Native) or Grid (PTY) │  ← Store output
└─────────────────────────────────┘
```

## PTY Architecture

The PTY system enables full interactive terminal support using ConPTY on Windows:

```
┌─────────────────┐
│  User Input     │
└────────┬────────┘
         │
    ┌────▼────┐
    │ Router  │ ← Is it native? Is it interactive?
    └────┬────┘
         │
   ┌─────┴─────┐
   │           │
┌──▼──┐    ┌───▼───┐
│Built│    │  PTY  │
│-in  │    │Session│ ← ConPTY (Windows) / native PTY (Unix)
└──┬──┘    └───┬───┘
   │           │
   └─────┬─────┘
         │
   ┌─────▼─────┐
   │ Terminal  │ ← Grid for PTY, Buffer for native
   │   Grid    │
   └───────────┘
```

### PTY Session

```rust
PtySession
├── master: Box<dyn MasterPty>    # PTY master for resizing
├── child: Box<dyn Child>         # Child process handle
├── writer: Box<dyn Write>        # Send input to PTY
├── output_rx: Receiver<PtyOutput>  # Receive output from reader thread
└── size: PtySize                 # Current terminal dimensions

PtyOutput (enum)
├── Data(Vec<u8>)                 # Raw output bytes
├── Exited(Option<u32>)           # Process exited
└── Error(String)                 # Error occurred
```

### Terminal Grid (VT100/ANSI Emulation)

```rust
TerminalGrid
├── cells: Vec<Vec<Cell>>         # 2D cell array (rows x cols)
├── cursor_row: usize             # Current cursor position
├── cursor_col: usize
├── scroll_region: (usize, usize) # Scrolling region
├── current_fg: Color             # Current foreground color
├── current_bg: Color             # Current background color
├── utf8_buffer: Vec<u8>          # Multi-byte UTF-8 accumulator
└── state: ParserState            # CSI escape sequence parser

Cell
├── char: char                    # Character to display
├── fg: Color                     # Foreground color
└── bg: Color                     # Background color

ParserState (enum)
├── Normal                        # Regular text
├── Escape                        # Saw ESC (0x1B)
└── Csi(Vec<u8>)                  # Collecting CSI sequence
```

### Supported ANSI/VT100 Sequences

| Sequence | Action |
|----------|--------|
| `ESC[H` | Cursor home |
| `ESC[<n>A/B/C/D` | Cursor up/down/forward/back |
| `ESC[<r>;<c>H` | Cursor position |
| `ESC[J` | Clear screen (0=below, 1=above, 2=all) |
| `ESC[K` | Clear line (0=right, 1=left, 2=all) |
| `ESC[<n>m` | SGR (colors, bold, etc.) |
| `ESC[?25h/l` | Show/hide cursor |
| Carriage return | Move to column 0 |
| Newline | Move down, scroll if needed |
| Backspace | Move cursor left |

### UTF-8 Multi-byte Handling

The terminal grid properly handles UTF-8 multi-byte sequences:

```rust
// Detect UTF-8 start bytes and expected length
fn utf8_char_len(first_byte: u8) -> usize {
    if first_byte < 0x80 { 1 }       // ASCII
    else if first_byte < 0xE0 { 2 }  // 2-byte
    else if first_byte < 0xF0 { 3 }  // 3-byte
    else { 4 }                        // 4-byte
}

// Accumulate bytes until complete, then decode
if is_utf8_start(byte) {
    utf8_buffer.push(byte);
} else if is_utf8_continuation(byte) {
    utf8_buffer.push(byte);
    if complete { decode_and_render(); }
}
```

### Hybrid Command Execution

```rust
fn route_command(cmd: &str, args: &[String]) -> ExecutionTarget {
    // 1. Built-in commands → native execution (fastest)
    if registry.has_command(cmd) {
        return ExecutionTarget::Native;
    }

    // 2. Known external tools → PTY for streaming output
    if is_external_tool(cmd) {
        return ExecutionTarget::Pty;
    }

    // 3. Unknown → try PTY
    ExecutionTarget::Pty
}
```

## External Tools System

The `tools.rs` module provides thin wrappers around 60+ external development tools:

```rust
// Generic tool executor
fn run_tool(program: &str, args: &[String], cwd: &Path) -> Result<String> {
    Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        // Returns combined stdout + stderr
}
```

### Supported Tool Categories

| Category | Tools |
|----------|-------|
| **Node.js** | npm, npx, yarn, pnpm, bun, node, deno |
| **Python** | python, pip, uv, poetry |
| **Rust** | cargo, rustc, rustup |
| **Go** | go |
| **C/C++** | gcc, g++, clang, clang++, make, cmake |
| **Debug** | gdb, lldb, valgrind, nm, objdump, ar, ld, nasm, as |
| **Java/JVM** | java, javac, mvn, gradle, scala, sbt, kotlin |
| **.NET** | dotnet |
| **Containers** | docker, kubectl |
| **Cloud/DevOps** | aws, az, gcloud, terraform, ansible |
| **Linters** | prettier, eslint, black, ruff, mypy |
| **Testing** | pytest, jest, vitest |
| **Other** | ruby, php, lua, julia, swift, zig, ghc, elixir, R, ocaml, racket, sbcl, gfortran, cobc |
| **Utils** | git, ssh, scp, rsync, gh, ffmpeg, convert, code, cursor |

Each tool wrapper:
- Executes in the current working directory
- Passes all arguments directly to the underlying tool
- Captures stdout and stderr
- Provides helpful error messages if tool not found in PATH

## Command Help System

All commands implement the `Command` trait which includes an `extended_help()` method:

```rust
pub trait Command {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn usage(&self) -> &'static str;

    // Comprehensive help with examples, tips, common errors
    fn extended_help(&self) -> String {
        // Default implementation uses name + description + usage
        // Commands can override for detailed help
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String>;
}
```

### How --help Works

The executor intercepts `--help` and `-h` flags before command execution:

```
User Input: "tar --help"
    │
    ▼
┌─────────────────┐
│    Executor     │  ← Check: args contains --help or -h?
└─────────────────┘
    │
    ├── Yes ──► registry.get_help("tar") ──► Return extended_help()
    │
    └── No ──► Normal command execution
```

### Extended Help Content

Each command's extended_help includes:
- **Usage syntax** with all options
- **Real examples** showing common use cases
- **Tips and gotchas** (e.g., "uniq requires sorted input!")
- **Common mistakes** to avoid
- **Related commands** for discovery

### Commands with Extended Help

50+ commands have detailed help pages:
- File operations: `cp`, `mv`, `rm`, `mkdir`, `touch`, `chmod`
- Text processing: `awk`, `tr`, `uniq`, `cut`, `diff`, `sed`, `sort`
- Hash/encoding: `md5sum`, `sha256sum`, `blake3sum`, `base64`, `xxd`
- System: `ps`, `kill`, `df`, `du`, `uptime`, `whoami`, `uname`
- Network: `curl`, `wget`, `ping`
- Compression: `tar`, `zip`, `gzip`, `gunzip`
- Shell: `alias`, `env`, `export`
- **Git**: Complete beginner guide with workflows and error fixes!

## Autocomplete System

The autocomplete engine provides intelligent suggestions:

1. **History** - Previously executed commands matching input
2. **File Paths** - Files and directories in current location
3. **Git Branches** - Local git branches (when in a repo)
4. **Command Flags** - Known flags for common commands (ls, grep, git, find, etc.)
5. **Environment Variables** - When typing `$`
6. **Built-in Commands** - All registered commands

```
Suggestion
├── text: String              # The suggestion text
├── kind: SuggestionKind      # History, File, Directory, GitBranch, Flag, Command, EnvVar
└── description: Option<String>  # Additional context
```

## Smart History System

The SmartHistory module provides context-aware command history:

```
SmartHistory
├── entries: Vec<HistoryEntry>    # All history entries (up to 10,000)
├── frequency: HashMap<String, usize>  # Global command frequency
├── dir_frequency: HashMap<PathBuf, HashMap<String, usize>>  # Per-directory frequency
└── session_id: u64               # Current session identifier

HistoryEntry
├── command: String               # The command executed
├── cwd: PathBuf                  # Working directory at execution time
├── exit_code: Option<i32>        # Command exit code (0 = success)
├── timestamp: SystemTime         # When command was run
├── duration: Option<Duration>    # How long command took
├── project_type: Option<ProjectType>  # Detected project type
├── tags: Vec<String>             # Auto-generated tags (git, cargo, npm, etc.)
└── output_snippet: Option<String>  # First few lines of output
```

### Features

- **Directory-aware suggestions**: Commands run in current directory are prioritized
- **Frequency tracking**: Most-used commands bubble to top
- **Auto-tagging**: Commands automatically tagged (git, cargo, npm, docker, etc.)
- **Exit code tracking**: Failed commands tracked separately
- **Duration tracking**: Command execution time recorded
- **Fuzzy search**: Find commands with partial matches
- **Saved input restoration**: Current input preserved during history navigation

## Keyboard Shortcuts

### Tabs
| Shortcut | Action |
|----------|--------|
| Ctrl+T | New tab |
| Ctrl+W | Close tab/pane |
| Ctrl+Tab | Next tab |
| Ctrl+1-9 | Switch to tab N |

### Split Panes
| Shortcut | Action |
|----------|--------|
| Ctrl+Shift+D | Split horizontal |
| Ctrl+Shift+E | Split vertical |
| Alt+Arrow | Navigate panes |

### Input
| Shortcut | Action |
|----------|--------|
| Tab | Apply suggestion |
| Up/Down | History / Navigate suggestions |
| Ctrl+F | Search |
| Escape | Close suggestions/search/hints/vi |

### Special Modes
| Shortcut | Action |
|----------|--------|
| Ctrl+Shift+H | Toggle hints mode |
| Ctrl+Shift+M | Toggle vi mode |

### Fuzzy Finder
| Shortcut | Action |
|----------|--------|
| Ctrl+R | Fuzzy search history |
| Ctrl+Shift+F | Fuzzy search files |
| Ctrl+G | Fuzzy search git branches |
| Up/Down | Navigate results |
| Enter | Insert selected |
| Ctrl+Enter | Execute selected |
| Escape | Close fuzzy finder |

### Clipboard & Line Editing
| Shortcut | Action |
|----------|--------|
| Ctrl+C | Interrupt (clears line, shows ^C) |
| Ctrl+Shift+C | Copy input line |
| Ctrl+V | Paste (strips newlines) |
| Ctrl+Shift+V | Paste raw (preserves newlines) |
| Ctrl+L | Clear screen |
| Ctrl+U | Clear line |
| Alt+. | Insert last argument |

### History Expansion
| Syntax | Expands To |
|--------|------------|
| `!!` | Last command |
| `!n` | nth command in history |
| `!-n` | nth-from-last command |

## AI Integration (Ollama)

Zaxiom includes built-in AI chat powered by Ollama's local API:

### Quick Chat (`#` prefix)
```
# how do I list files?
# explain what a hashmap is
```

The AI receives terminal context:
- Current working directory
- Operating system
- Recent command history (last 10 commands)

### Ollama Command
```
ollama list      # List installed models
ollama pull      # Download models
ollama run       # Chat with model
ollama serve     # Start server
ollama status    # Check status
```

### Architecture
```
User Input (# prompt)
    │
    ▼
┌─────────────────┐
│  Context Build  │  ← Gather cwd, OS, history
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Ollama API     │  ← POST /api/generate (localhost:11434)
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Response       │  ← Display in terminal
└─────────────────┘
```

## Fuzzy Finder System

The FuzzyFinder module provides fzf-like search functionality:

```
FuzzyFinder
├── active: bool              # Whether fuzzy finder is open
├── mode: FuzzyMode           # History, Files, or GitBranches
├── query: String             # Current search query
├── all_items: Vec<FuzzyItem> # All available items
├── items: Vec<FuzzyItem>     # Filtered/scored results
├── selected: usize           # Currently selected index
├── max_display: usize        # Max items to show (10)
└── scroll_offset: usize      # For scrolling long lists

FuzzyItem
├── display: String           # What user sees
├── value: String             # Value to insert/execute
├── preview: Option<String>   # Right-side description
├── score: i32                # Match score (higher = better)
├── match_positions: Vec<usize>  # For highlighting
└── icon: &'static str        # Item icon

FuzzyMode (enum)
├── History                   # Command history (Ctrl+R)
├── Files                     # File search (Ctrl+Shift+F)
└── GitBranches               # Git branches (Ctrl+G)

FuzzyAction (enum)
├── None                      # Still searching
├── Insert(String)            # Enter - insert into command line
├── Execute(String)           # Ctrl+Enter - run immediately
└── Cancelled                 # Escape pressed
```

### Scoring Algorithm

| Match Type | Score |
|------------|-------|
| Exact match | +1000 |
| Starts with query | +500 |
| Contains as substring | +200 |
| Fuzzy (chars in order) | +10/char |
| Consecutive char bonus | +5 |
| Word boundary bonus | +10 |

### Data Loaders

- **History**: Populated from SmartHistory entries (command + cwd preview)
- **Files**: Uses walkdir, max depth 4, ignores common patterns (node_modules, target, .git, __pycache__, dist, build)
- **Git Branches**: Reads from .git/refs/heads (local) and .git/refs/remotes (remote)

### UI Features

- Bottom-anchored overlay (like real fzf)
- Match highlighting with accent color
- Keyboard hints shown at bottom
- Match count display (filtered/total)

## Built-in Editor (nano/vim/vi/edit)

Full-screen text editor overlay:
- **Ctrl+S** - Save file
- **Ctrl+X / Esc** - Exit editor
- **Arrow keys** - Navigate
- **PgUp / PgDn** - Scroll page
- **Home / End** - Start/end of line
- **Ctrl+Home** - Start of file
- **Ctrl+End** - End of file
- Line numbers display
- Modified indicator `[+]`
- Cursor position (Line X, Col Y)

## Output Rendering Pipeline

Terminal output goes through several processing stages:

```
Command Output
    │
    ▼
┌─────────────────┐
│  Output Buffer  │  ← Store in blocks, detect URLs
└─────────────────┘
    │
    ▼
┌─────────────────┐
│   ANSI Parser   │  ← Parse escape codes for colors/styles
└─────────────────┘
    │
    ▼
┌─────────────────┐
│    Renderer     │  ← Convert to egui RichText
└─────────────────┘
    │
    ├── Plain text ──► Default foreground color
    ├── ANSI colored ──► RGB colors from escape codes
    ├── URLs ──► Clickable, underlined, open in browser
    └── Syntax highlighted ──► Language-specific colors
```

### Syntax Highlighting

The `cat` command uses syntect for syntax highlighting:
- Automatically detects language from file extension
- Supports 40+ languages (Rust, Python, JS, Go, etc.)
- Uses Monokai theme for dark terminal compatibility
- Can be forced with `-s` or disabled with `-p`

### Block-Based Output

Each command creates a CommandBlock with:
- Copy button (copies output to clipboard)
- Execution duration display
- Start/end line tracking
- Success/failure status

## Theme System

Zaxiom includes 20 built-in themes. Default is Catppuccin Mocha:

```rust
ThemeName (enum) {
    CatppuccinMocha,   // Default - modern pastel dark
    CatppuccinLatte,   // Modern pastel light
    Dracula,           // Popular dark theme
    Nord,              // Arctic, bluish colors
    GruvboxDark,       // Retro groove dark
    GruvboxLight,      // Retro groove light
    TokyoNight,        // Clean dark theme
    TokyoNightStorm,   // Darker Tokyo Night
    OneDark,           // Atom-inspired
    SolarizedDark,     // Ethan Schoonover's dark
    SolarizedLight,    // Ethan Schoonover's light
    MonokaiPro,        // Sublime Text inspired
    Palenight,         // Material palenight
    AyuDark,           // Ayu dark
    AyuMirage,         // Ayu mirage
    Kanagawa,          // Hokusai-inspired
    RosePine,          // Soho vibes
    RosePineMoon,      // Rose Pine darker
    EverforestDark,    // Comfortable green
    NightOwl,          // For night owls
}

Theme {
    // Base colors (example: Catppuccin Mocha)
    background: #1e1e2e,
    foreground: #cdd6f4,

    // Syntax colors
    path_color: #94e2d5,      // teal
    branch_color: #fab387,    // peach
    command_color: #cba6f7,   // mauve
    flag_color: #89dceb,      // sky

    // Status colors
    error_color: #f38ba8,     // red
    success_color: #a6e3a1,   // green
    warning_color: #f9e2af,   // yellow
    info_color: #89b4fa,      // blue

    // Accents
    accent: #cba6f7,          // mauve
    folder_color: #f9e2af,    // yellow
}
```

Theme selection is persisted to `~/.config/zaxiom/config.toml`.

Use `theme <name>` command to switch themes at runtime.

### Enhanced UI Mode

Toggle enhanced visual mode with softer aesthetics:
- `theme --kawaii` - Enable enhanced mode (pastel accents, rounded corners)
- `theme --normal` - Restore default appearance

When enabled:
- Accent colors shift to pastel pink/lavender
- UI corner radii increase for softer appearance
- Prompt symbol changes from `❯` to `♡`
- Git branch icon changes from `` to `🌸`

## Mascot System

The robot mascot provides contextual visual feedback through a state machine:

```
Mascot
├── mood: MascotMood          # Current emotional state
├── frame: u64                # Animation frame counter
├── mood_timer: Duration      # Time remaining in current mood
├── activity_timer: Duration  # Time since last interaction
└── particles: Vec<Particle>  # Special effect particles (confetti, etc.)

MascotMood (enum) - 14 Expressions
├── Idle         # Default resting state (._. eyes)
├── Thinking     # Processing animation
├── Happy        # Success reactions (^_^ eyes)
├── Sad          # Failure reactions
├── Excited      # High-activity state
├── Sleepy       # After inactivity (-.-)
├── Waving       # Interactive greeting
├── Love         # Heart eyes (◕‿◕)♡
├── Surprised    # Unexpected events (O_O)
├── Proud        # Build/test success (★_★)
├── Confused     # Unknown commands (?_?)
├── Dancing      # Celebration (^o^)
├── Celebrating  # Major success with confetti (★▽★)
└── Typing       # While user is typing (._ .)
```

### Mood Triggers

| Event | Triggered Mood |
|-------|----------------|
| Command success | Happy |
| Command failure | Sad |
| Unknown command | Confused |
| Build/test pass | Celebrating |
| Extended inactivity | Sleepy |
| `pet` command | Love, Dancing, etc. |
| Keywords (party, dance) | Dancing |

### Animation System

- **Bounce**: Vertical oscillation using `sin(frame * speed)`
- **Sway**: Horizontal movement for Dancing/Confused moods
- **Eye Expressions**: Per-mood eye rendering (sparkles, spirals, hearts)
- **Arm Poses**: Contextual arm positions (typing, waving, celebrating)
- **Particle Effects**: Confetti for Celebrating, question marks for Confused

## Session Persistence

Sessions are saved as JSON in `~/.local/share/zaxiom/sessions/`:

```json
{
  "name": "default",
  "tabs": [
    {
      "title": "projects",
      "cwd": "C:\\Users\\user\\projects",
      "history": ["ls", "cd src", "cargo build"],
      "scroll_position": 0
    }
  ],
  "active_tab": 0,
  "saved_at": 1703123456
}
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| eframe/egui | GPU-accelerated UI framework |
| tokio | Async runtime |
| portable-pty | Cross-platform PTY (ConPTY on Windows) |
| serde/serde_json | JSON serialization |
| regex | Pattern matching |
| walkdir | Directory traversal |
| git2 | Git operations |
| reqwest | HTTP client |
| blake3/sha2/md-5 | Hashing |
| flate2/tar/zip | Compression |
| chrono | Date/time |
| sysinfo | System information |
| image | Image processing |
| dirs | Platform directories |
| syntect | Syntax highlighting (40+ languages) |
| arboard | Cross-platform clipboard access |
| open | Open URLs in default browser |

## Installation Scripts

| Script | Purpose |
|--------|---------|
| `install.ps1` | Full installer - builds release, copies to AppData, creates shortcuts |
| `update.ps1` | Quick update - rebuilds and copies to install location |
| `build.rs` | Embeds Windows icon into executable |

### Install Locations

- **Executable**: `%LOCALAPPDATA%\Zaxiom\zaxiom.exe`
- **Icon**: `%LOCALAPPDATA%\Zaxiom\icon.ico`
- **Config**: `~/.config/zaxiom/config.toml`
- **Sessions**: `~/.local/share/zaxiom/sessions/`
