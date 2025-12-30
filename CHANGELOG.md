# Changelog

All notable changes to Zaxiom will be documented in this file.

## [0.3.1] - 2025-12-30

### Added
- Functional split pane system with working input in all panes
- Click-to-focus support for switching between panes
- Text wrapping in split pane buffers
- AI commands in command palette (ai status, ai providers, ollama commands)
- Split/Vi Mode/Hints Mode/History Search actions in command palette

### Fixed
- Split pane input handling - all panes now accept keyboard input
- Enter key execution in split panes
- Layout overlap issues in split mode

## [0.3.0] - 2025-12-30

### Added
- Multi-provider AI integration with 10 providers (Ollama, Groq, OpenAI, Anthropic, Gemini, Mistral, DeepSeek, xAI, Cohere, Perplexity)
- AI chat via `#` prefix: `# explain what a hashmap is`
- Provider flags: `# --claude`, `# --gpt`, `# --ollama`, etc.
- Provider switching: `# --provider` sets default, `# --provider msg` uses once
- AI config in `config.toml` for default provider
- Setup instructions when API key is missing
- `.env.example` template for API keys

### Fixed
- Ollama no longer opens separate terminal window on Windows
- PTY newline cursor position
- Syntax highlighting theme (base16-ocean.dark)
- Hints test assertion

## [0.2.0] - 2025-12-29

### Added
- Full PTY support via ConPTY for interactive applications
- Terminal grid with ANSI/VT100 escape sequence handling
- Run vim, ssh, node REPL, and other interactive CLI tools
- GitHub Actions CI workflow
- Makefile for cross-platform builds
- Reorganized scripts into `run/` directory

### Fixed
- Crash on window resize
- Editor and overlay keyboard handling
- Install/update scripts hanging

## [0.1.0] - 2025-12-28

### Added
- Initial release of Zaxiom
- 100+ built-in Unix commands (ls, cd, cat, grep, find, etc.)
- Command palette with fuzzy search (Ctrl+Shift+P)
- Kawaii mode with robot mascot
- 20 built-in themes
- Git integration and shortcuts
- Syntax highlighting
- Smart command history with fuzzy search (Ctrl+R)
- Tab completion for commands, paths, and git branches
- Vi mode for scrollback navigation
- Hints mode for extracting URLs and paths
- Split panes and tabs

[0.3.1]: https://github.com/aayushadhikari7/zaxiom/releases/tag/v0.3.1
[0.3.0]: https://github.com/aayushadhikari7/zaxiom/releases/tag/v0.3.0
[0.2.0]: https://github.com/aayushadhikari7/zaxiom/releases/tag/v0.2.0
[0.1.0]: https://github.com/aayushadhikari7/zaxiom/releases/tag/v0.1.0
