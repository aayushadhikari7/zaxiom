//! Main application struct and egui rendering loop

use std::collections::HashMap;
use arboard::Clipboard;
use eframe::egui;

use crate::commands::files::EditorState;
use crate::config::theme::{Theme, ThemeName};
use crate::config::settings::Config;
use crate::mascot::Mascot;
use crate::shell::executor::Executor;
use crate::terminal::ansi;
use crate::terminal::autocomplete::{Autocomplete, Suggestion, SuggestionKind};
use crate::terminal::buffer::{LineType, OutputBuffer};
use crate::terminal::hints::{HintsExtractor, HintsMode, HintType};
use crate::terminal::smart_history::SmartHistory;
use crate::terminal::session::{SavedSession, SavedTab, SessionManager};
use crate::terminal::split::{SplitDirection, SplitManager};
use crate::terminal::state::TerminalState;
use crate::terminal::vi_mode::{ViMode, ViAction, ViState};
use crate::terminal::palette::CommandPalette;
use crate::terminal::fuzzy::{FuzzyFinder, FuzzyMode, FuzzyAction};

/// ASCII art logo only (shown after clear)
const ASCII_LOGO: &str = r#"
    ███████╗ █████╗ ██╗  ██╗██╗ ██████╗ ███╗   ███╗
    ╚══███╔╝██╔══██╗╚██╗██╔╝██║██╔═══██╗████╗ ████║
      ███╔╝ ███████║ ╚███╔╝ ██║██║   ██║██╔████╔██║
     ███╔╝  ██╔══██║ ██╔██╗ ██║██║   ██║██║╚██╔╝██║
    ███████╗██║  ██║██╔╝ ██╗██║╚██████╔╝██║ ╚═╝ ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═╝
"#;

/// Startup ASCII banner (full version with help text)
const STARTUP_BANNER: &str = r#"
    ███████╗ █████╗ ██╗  ██╗██╗ ██████╗ ███╗   ███╗
    ╚══███╔╝██╔══██╗╚██╗██╔╝██║██╔═══██╗████╗ ████║
      ███╔╝ ███████║ ╚███╔╝ ██║██║   ██║██╔████╔██║
     ███╔╝  ██╔══██║ ██╔██╗ ██║██║   ██║██║╚██╔╝██║
    ███████╗██║  ██║██╔╝ ██╗██║╚██████╔╝██║ ╚═╝ ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═╝

        ╭──────────────────────────────────────────╮
        │  ♡  Welcome to Zaxiom v0.1.0~            │
        │      Linux vibes on Windows! (◕‿◕)♡     │
        │                                          │
        │  Your kawaii robot companion is here! →  │
        ╰──────────────────────────────────────────╯

    ✨ Type 'help' to see all commands~
    🎀 Fun: fortune, cowsay, coffee, matrix, neofetch
    ⚡ Git shortcuts: gs, gd, gl, gp, ga, gc
    🚀 Built with Rust + egui — blazingly fast!

    ⌨️  Tabs: Ctrl+T | Ctrl+W | Ctrl+Tab
    🔲 Splits: Ctrl+Shift+D | Ctrl+Shift+E
    📋 Clipboard: Ctrl+Shift+C (copy) | Ctrl+V (paste)
    ⚡ Ctrl+C: Interrupt │ Hints: Ctrl+Shift+H │ Vi: Ctrl+Shift+M

    ♪(´ε` ) Let's have fun together~
"#;

/// A single terminal pane within a tab
pub struct PaneSession {
    /// Terminal state
    pub state: TerminalState,
    /// Output buffer
    pub buffer: OutputBuffer,
    /// Smart command history with context tracking
    pub history: SmartHistory,
    /// Current input
    pub input: String,
    /// Saved input when navigating history
    pub saved_input: String,
    /// Whether this pane needs to scroll to bottom
    pub scroll_to_bottom: bool,
    /// Search mode (Ctrl+F)
    pub search_mode: bool,
    /// Search query
    pub search_query: String,
    /// Search matches (line indices)
    pub search_matches: Vec<usize>,
    /// Current search match index
    pub current_match: usize,
    /// Autocomplete suggestions
    pub suggestions: Vec<Suggestion>,
    /// Selected suggestion index
    pub selected_suggestion: usize,
    /// Whether autocomplete popup is visible
    pub show_suggestions: bool,
    /// Last input that triggered suggestions
    pub last_suggestion_input: String,
    /// Suppress suggestions for one update cycle (after applying)
    pub suppress_suggestions: bool,
    /// Move cursor to end on next frame (after autocomplete)
    pub cursor_to_end: bool,
    /// Hints mode for URL/path extraction
    pub hints_mode: HintsMode,
    /// Vi mode for terminal navigation
    pub vi_mode: ViMode,
    /// Scroll to selected hint on next frame (after Tab cycling)
    pub hints_scroll_to_selected: bool,
    /// Fuzzy finder (Ctrl+R/Ctrl+Shift+F/Ctrl+G)
    pub fuzzy_finder: FuzzyFinder,
}

impl PaneSession {
    /// Create a new pane session
    pub fn new(show_banner: bool) -> Self {
        let state = TerminalState::new();
        let mut buffer = OutputBuffer::new(10_000);

        if show_banner {
            for line in STARTUP_BANNER.lines() {
                buffer.push_line(line);
            }
        } else {
            // Random kawaii greeting for new panes!
            let greetings = [
                "✨ New pane spawned~ (◕‿◕)✧",
                "🎀 Hello new pane! ヾ(◕‿◕)ノ",
                "💫 Pane ready! Let's go~ ٩(◕‿◕)۶",
                "🌸 Fresh pane here! (ﾉ´ヮ`)ﾉ*: ・゚✧",
                "⭐ New workspace~ ♪(´ε` )",
                "🎵 Beep boop! New pane! (◠‿◠)",
                "💜 Ready to work~ (◕ᴗ◕✿)",
                "🌟 Let's do this! ᕙ(◕‿◕)ᕗ",
            ];
            let idx = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as usize % greetings.len())
                .unwrap_or(0);
            buffer.push_line(greetings[idx]);
            buffer.push_line(&format!("📂 {}", state.cwd().display()));
        }

        Self {
            state,
            buffer,
            history: SmartHistory::new(10_000),
            input: String::new(),
            saved_input: String::new(),
            scroll_to_bottom: false,
            search_mode: false,
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: 0,
            suggestions: Vec::new(),
            selected_suggestion: 0,
            show_suggestions: false,
            last_suggestion_input: String::new(),
            suppress_suggestions: false,
            cursor_to_end: false,
            hints_mode: HintsMode::new(),
            vi_mode: ViMode::new(),
            hints_scroll_to_selected: false,
            fuzzy_finder: FuzzyFinder::new(),
        }
    }

    /// Toggle search mode
    pub fn toggle_search(&mut self) {
        self.search_mode = !self.search_mode;
        if !self.search_mode {
            self.search_query.clear();
            self.search_matches.clear();
            self.current_match = 0;
        }
    }

    /// Update search results
    pub fn update_search(&mut self) {
        if self.search_query.is_empty() {
            self.search_matches.clear();
            self.current_match = 0;
        } else {
            self.search_matches = self.buffer.search(&self.search_query);
            if self.current_match >= self.search_matches.len() {
                self.current_match = 0;
            }
        }
    }

    /// Go to next search match
    pub fn next_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.search_matches.len();
        }
    }

    /// Go to previous search match
    pub fn prev_match(&mut self) {
        if !self.search_matches.is_empty() {
            if self.current_match == 0 {
                self.current_match = self.search_matches.len() - 1;
            } else {
                self.current_match -= 1;
            }
        }
    }
}

/// A single terminal tab containing one or more panes
pub struct TabSession {
    /// Unique tab ID
    #[allow(dead_code)]
    pub id: usize,
    /// Tab title (directory name or custom)
    pub title: String,
    /// Split pane manager
    pub splits: SplitManager,
    /// Pane sessions by pane ID
    pub panes: HashMap<usize, PaneSession>,
}

impl TabSession {
    /// Create a new tab session with a single pane
    pub fn new(id: usize, show_banner: bool) -> Self {
        let first_pane = PaneSession::new(show_banner);
        let title = first_pane.state.cwd()
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "~".to_string());

        let mut panes = HashMap::new();
        panes.insert(0, first_pane); // Pane ID 0 is the first pane

        Self {
            id,
            title,
            splits: SplitManager::new(),
            panes,
        }
    }

    /// Get the currently focused pane
    pub fn focused_pane(&self) -> Option<&PaneSession> {
        self.panes.get(&self.splits.focused_pane_id())
    }

    /// Get the currently focused pane mutably
    pub fn focused_pane_mut(&mut self) -> Option<&mut PaneSession> {
        let pane_id = self.splits.focused_pane_id();
        self.panes.get_mut(&pane_id)
    }

    /// Split the focused pane
    pub fn split(&mut self, direction: SplitDirection) {
        let new_pane_id = self.splits.split(direction);
        let new_pane = PaneSession::new(true);  // Show banner in new panes
        self.panes.insert(new_pane_id, new_pane);
    }

    /// Close the focused pane
    pub fn close_focused_pane(&mut self) -> bool {
        let pane_id = self.splits.focused_pane_id();
        if self.splits.close_pane(pane_id) {
            self.panes.remove(&pane_id);
            true
        } else {
            false
        }
    }

    /// Update title based on focused pane's directory
    pub fn update_title(&mut self) {
        if let Some(pane) = self.focused_pane() {
            self.title = pane.state.cwd()
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "~".to_string());
        }
    }

    /// Get pane count
    pub fn pane_count(&self) -> usize {
        self.panes.len()
    }
}

/// Main Zaxiom application
pub struct ZaxiomApp {
    /// All terminal tabs
    tabs: Vec<TabSession>,
    /// Currently active tab index
    active_tab: usize,
    /// Next tab ID
    next_tab_id: usize,
    /// Command executor (shared)
    executor: Executor,
    /// Theme configuration
    theme: Theme,
    /// Current theme name (for saving)
    theme_name: ThemeName,
    /// App configuration
    config: Config,
    /// Whether to exit the app
    should_exit: bool,
    /// Our cute robot mascot
    mascot: Mascot,
    /// Editor state (Some when editing a file)
    #[allow(dead_code)]
    editor: Option<EditorState>,
    /// Autocomplete engine
    autocomplete: Autocomplete,
    /// Session manager for persistence
    session_manager: SessionManager,
    /// Frame counter for periodic autosave
    frame_count: u64,
    /// Clipboard for copy/paste
    clipboard: Option<Clipboard>,
    /// Visual feedback for clipboard operations
    clipboard_feedback: Option<(String, std::time::Instant)>,
    /// Command palette (Ctrl+P)
    command_palette: CommandPalette,
    /// Kawaii mode - cuter UI elements
    kawaii_mode: bool,
}

impl ZaxiomApp {
    /// Create a new Zaxiom application
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure custom fonts
        let mut fonts = egui::FontDefinitions::default();

        // Try to load Hurmit Nerd Font Mono
        let font_path = std::path::Path::new("assets/fonts/HurmitNerdFontMono-Regular.otf");
        if font_path.exists() {
            if let Ok(font_data) = std::fs::read(font_path) {
                fonts.font_data.insert(
                    "Hurmit".to_owned(),
                    egui::FontData::from_owned(font_data).into(),
                );

                // Set as default monospace font
                fonts
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, "Hurmit".to_owned());

                // Also set as default proportional for consistency
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "Hurmit".to_owned());
            }
        }

        cc.egui_ctx.set_fonts(fonts);

        // Set dark theme
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        // Load configuration
        let config = Config::load();

        // Load theme from config or use default
        let theme_name = config.theme.name
            .as_ref()
            .and_then(|name| ThemeName::from_string(name))
            .unwrap_or_default();
        let kawaii_mode = config.kawaii_mode;
        let theme = if kawaii_mode {
            Theme::from_name(theme_name).apply_kawaii()
        } else {
            Theme::from_name(theme_name)
        };
        let mut style = (*cc.egui_ctx.style()).clone();

        // Set base font sizes using theme settings
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(theme.font_size, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(theme.font_size, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            egui::FontId::new(theme.font_size * 0.85, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(theme.font_size, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(theme.font_size * 1.25, egui::FontFamily::Monospace),
        );

        // Set comfortable spacing
        style.spacing.item_spacing = egui::vec2(8.0, theme.font_size * (theme.line_height - 1.0));

        // Apply kawaii mode visual adjustments
        if kawaii_mode {
            // More rounded corners for a cuter look
            style.visuals.window_corner_radius = egui::CornerRadius::same(12);
            style.visuals.menu_corner_radius = egui::CornerRadius::same(10);
            // Softer widget styling
            style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(8);
            style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(8);
            style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(8);
            style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(8);
        }

        cc.egui_ctx.set_style(style);

        // Always start fresh (no session restore - like a normal terminal)
        let session_manager = SessionManager::new();
        let (tabs, active_tab, next_tab_id) = (vec![TabSession::new(0, true)], 0, 1);

        // Try to create clipboard (may fail on some systems)
        let clipboard = Clipboard::new().ok();

        Self {
            tabs,
            active_tab,
            next_tab_id,
            executor: Executor::new(),
            theme,
            theme_name,
            config,
            should_exit: false,
            mascot: Mascot::new(),
            editor: None,
            autocomplete: Autocomplete::new(),
            session_manager,
            frame_count: 0,
            clipboard,
            clipboard_feedback: None,
            command_palette: CommandPalette::new(),
            kawaii_mode,
        }
    }

    /// Restore a tab from saved data
    #[allow(dead_code)]
    fn restore_tab(id: usize, saved: &SavedTab) -> Option<TabSession> {
        let mut pane = PaneSession::new(false);

        // Restore working directory
        if saved.cwd.exists() {
            pane.state.set_cwd(saved.cwd.clone());
        }

        // Restore history with SmartHistory (commands are added with restored cwd)
        let cwd = pane.state.cwd().clone();
        for cmd in &saved.history {
            pane.history.add(cmd, cwd.clone(), None);
        }

        // Clear the default "New pane" message and show restore message
        pane.buffer.clear();
        pane.buffer.push_line(&format!(" Session restored: {}", saved.cwd.display()));

        let mut panes = HashMap::new();
        panes.insert(0, pane);

        Some(TabSession {
            id,
            title: saved.title.clone(),
            splits: SplitManager::new(),
            panes,
        })
    }

    /// Create a SavedSession from the current app state
    fn create_saved_session(&self) -> SavedSession {
        let mut session = SavedSession::new("autosave");

        for tab in &self.tabs {
            // Get the focused pane's data (or first pane if none focused)
            if let Some(pane) = tab.focused_pane() {
                // Get commands from SmartHistory entries
                let history: Vec<String> = pane.history.all()
                    .take(100) // Limit to last 100 commands
                    .map(|entry| entry.command.clone())
                    .collect();

                let saved_tab = SavedTab {
                    title: tab.title.clone(),
                    cwd: pane.state.cwd().to_path_buf(),
                    history,
                    scroll_position: 0,
                };
                session.add_tab(saved_tab);
            }
        }

        session.active_tab = self.active_tab;
        session
    }

    /// Autosave the current session
    fn autosave(&self) {
        let session = self.create_saved_session();
        if let Err(e) = self.session_manager.autosave(&session) {
            eprintln!("Failed to autosave session: {}", e);
        }
    }

    /// Create a new tab
    fn new_tab(&mut self) {
        let tab = TabSession::new(self.next_tab_id, true);  // Show banner in new tabs
        self.next_tab_id += 1;
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    /// Close current tab
    fn close_current_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.tabs.remove(self.active_tab);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    /// Switch to next tab
    fn next_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }

    /// Switch to previous tab
    fn prev_tab(&mut self) {
        if self.tabs.len() > 1 {
            if self.active_tab == 0 {
                self.active_tab = self.tabs.len() - 1;
            } else {
                self.active_tab -= 1;
            }
        }
    }

    /// Copy text to clipboard
    fn copy_to_clipboard(&mut self, text: &str) {
        if let Some(ref mut clipboard) = self.clipboard {
            if clipboard.set_text(text.to_string()).is_ok() {
                self.clipboard_feedback = Some((
                    format!("✨ Copied~ (◕‿◕) {}", if text.len() > 30 {
                        format!("{}...", &text[..30])
                    } else {
                        text.to_string()
                    }),
                    std::time::Instant::now(),
                ));
            }
        }
    }

    /// Paste from clipboard
    fn paste_from_clipboard(&mut self) -> Option<String> {
        if let Some(ref mut clipboard) = self.clipboard {
            if let Ok(text) = clipboard.get_text() {
                self.clipboard_feedback = Some((
                    "📋 Pasted~ (ﾉ◕ヮ◕)ﾉ*:・゚✧".to_string(),
                    std::time::Instant::now(),
                ));
                return Some(text);
            }
        }
        None
    }

    /// Copy current input line
    fn copy_current_input(&mut self) {
        if let Some(pane) = self.tabs[self.active_tab].focused_pane() {
            if !pane.input.is_empty() {
                let text = pane.input.clone();
                self.copy_to_clipboard(&text);
            } else {
                // If input is empty, clear it as a "cancel" action
                self.clipboard_feedback = Some((
                    "✧ Input cleared~ (￣▽￣)".to_string(),
                    std::time::Instant::now(),
                ));
            }
        }
    }

    /// Paste into current input
    fn paste_to_input(&mut self) {
        if let Some(text) = self.paste_from_clipboard() {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                // For Ctrl+V, strip newlines and paste as single line
                let clean_text = text.replace('\n', " ").replace('\r', "");
                pane.input.push_str(&clean_text);
            }
        }
    }

    /// Paste raw (including newlines) - Ctrl+Shift+V
    fn paste_raw_to_input(&mut self) {
        if let Some(text) = self.paste_from_clipboard() {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.input.push_str(&text);
            }
        }
    }

    /// Expand history references in command (!! and !n)
    fn expand_history(command: &str, history_commands: &[String]) -> String {
        let mut result = command.to_string();

        // !! expands to last command
        if result.contains("!!") {
            if let Some(last) = history_commands.last() {
                result = result.replace("!!", last);
            }
        }

        // !-n expands to nth-from-last command (e.g., !-1 = last, !-2 = second to last)
        if let Ok(re_neg) = regex::Regex::new(r"!-(\d+)") {
            let result_clone = result.clone();
            for cap in re_neg.captures_iter(&result_clone) {
                if let Ok(n) = cap[1].parse::<usize>() {
                    if n > 0 && n <= history_commands.len() {
                        let idx = history_commands.len() - n;
                        result = result.replace(&cap[0], &history_commands[idx]);
                    }
                }
            }
        }

        // !n expands to nth command (1-indexed)
        if let Ok(re_pos) = regex::Regex::new(r"!(\d+)") {
            let result_clone = result.clone();
            for cap in re_pos.captures_iter(&result_clone) {
                if let Ok(n) = cap[1].parse::<usize>() {
                    if n > 0 && n <= history_commands.len() {
                        result = result.replace(&cap[0], &history_commands[n - 1]);
                    }
                }
            }
        }

        result
    }

    /// Process a command in the focused pane of the current tab
    fn execute_command(&mut self, command: &str) {
        if command.trim().is_empty() {
            return;
        }

        let tab = &mut self.tabs[self.active_tab];
        let pane_id = tab.splits.focused_pane_id();

        let theme_to_apply = if let Some(pane) = tab.panes.get_mut(&pane_id) {
            // History expansion: !! = last command, !n = nth command
            let history_commands: Vec<String> = pane.history.all()
                .map(|e| e.command.clone())
                .collect();
            let command = Self::expand_history(command, &history_commands);
            let command = command.as_str();

            // Add command to smart history with context
            let cwd = pane.state.cwd().clone();
            pane.history.add(command, cwd, None);

            // Reset history navigation position
            pane.history.reset_position();
            pane.saved_input.clear();

            // Start a new command block
            pane.buffer.start_block(command);

            // Show the prompt + command in output
            let prompt = pane.state.format_prompt();
            pane.buffer.push_line(&format!("{}{}", prompt, command));

            // Execute the command (pass history for AI context)
            let history = pane.history.recent_commands(10);
            let success = match self.executor.execute_with_history(command, &mut pane.state, Some(&history)) {
                Ok(output) => {
                    // Check for special command markers
                    if output.starts_with("\x1b[CLEAR]") {
                        pane.buffer.clear();
                        // Show just the ASCII logo after clear (not full banner)
                        for line in ASCII_LOGO.lines() {
                            pane.buffer.push_line(line);
                        }
                    } else if output.starts_with("\x1b[EXIT") {
                        self.should_exit = true;
                    } else if output.starts_with("\x1b[EDIT]") {
                        // Open the editor with the specified file
                        let file_path = output.trim_start_matches("\x1b[EDIT]");
                        let path = std::path::PathBuf::from(file_path);
                        match EditorState::new(path.clone()) {
                            Ok(editor_state) => {
                                self.editor = Some(editor_state);
                                pane.buffer.push_line(&format!("📝 Opening {} ...", path.display()));
                            }
                            Err(e) => {
                                pane.buffer.push_error(&format!("Failed to open file: {}", e));
                            }
                        }
                    } else if !output.is_empty() {
                        for line in output.lines() {
                            pane.buffer.push_line(line);
                        }
                    }
                    true
                }
                Err(e) => {
                    // Kawaii error messages!
                    let sad_faces = ["(´;ω;`)", "(◞‸◟)", "(´・ω・`)", "(｡•́︿•̀｡)", "(っ◞‸◟c)"];
                    let idx = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_nanos() as usize % sad_faces.len())
                        .unwrap_or(0);
                    pane.buffer.push_error(&format!("{} Oopsie~ {}", sad_faces[idx], e));
                    false
                }
            };

            // End the command block
            pane.buffer.end_block(success);

            // Update smart history with command result
            if let Some(block) = pane.buffer.blocks().last() {
                let exit_code = if success { 0 } else { 1 };
                let duration = block.duration.unwrap_or_default();
                pane.history.complete_last(exit_code, duration, None);
            }

            // Show command duration for commands that took significant time
            if let Some(duration_str) = pane.buffer.last_block_duration() {
                // Only show for commands that took more than 100ms
                if let Some(block) = pane.buffer.blocks().last() {
                    if let Some(dur) = block.duration {
                        if dur.as_millis() >= 100 {
                            let status_icon = if success { "" } else { "" };
                            pane.buffer.push_line(&format!(
                                " {} completed in {}",
                                status_icon, duration_str
                            ));
                        }
                    }
                }
            }

            // Let mascot react to command
            self.mascot.on_command(command, success);

            // Occasionally show kawaii tips (roughly 1 in 20 commands)
            if success {
                let tip_chance = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as usize % 20)
                    .unwrap_or(1);

                if tip_chance == 0 {
                    let tips = [
                        "💡 Tip: Try 'pet' to interact with your robot companion! (◕‿◕)",
                        "✨ Tip: Use Ctrl+Shift+D to split panes horizontally~",
                        "🎀 Tip: 'fortune | cowsay' for extra fun! ♪(´ε` )",
                        "💜 Tip: Your mascot reacts differently to various commands!",
                        "⭐ Tip: 'theme list' shows all 20 beautiful themes~",
                        "🌸 Tip: Ctrl+C interrupts, Ctrl+Shift+C copies! (◕ᴗ◕✿)",
                        "🎵 Tip: Try 'neofetch' to see system info with style!",
                        "💫 Tip: 'cat -s file.rs' shows syntax highlighting!",
                    ];
                    let tip_idx = (tip_chance + tips.len()) % tips.len();
                    pane.buffer.push_line("");
                    pane.buffer.push_line(tips[tip_idx]);
                }
            }

            // Check for theme change request and store it
            let theme_change = pane.state.requested_theme.take();
            pane.scroll_to_bottom = true;
            theme_change
        } else {
            None
        };

        // Handle theme change after pane borrow ends
        if let Some(new_theme_name) = theme_to_apply {
            let base_theme = Theme::from_name(new_theme_name);
            self.theme = if self.kawaii_mode {
                base_theme.apply_kawaii()
            } else {
                base_theme
            };
            self.theme_name = new_theme_name;
            // Update current_theme on all panes
            for tab in &mut self.tabs {
                for pane in tab.panes.values_mut() {
                    pane.state.current_theme = new_theme_name;
                }
            }
            // Save to config file
            if let Err(e) = self.config.set_theme(new_theme_name.config_key()) {
                eprintln!("Failed to save theme config: {}", e);
            }
        }

        // Check for kawaii mode change from any pane
        let focused_tab = &self.tabs[self.active_tab];
        let focused_pane_id = focused_tab.splits.focused_pane_id();
        if let Some(pane) = focused_tab.panes.get(&focused_pane_id) {
            if pane.state.kawaii_mode != self.kawaii_mode {
                self.kawaii_mode = pane.state.kawaii_mode;
                // Re-apply theme with kawaii mode
                let base_theme = Theme::from_name(self.theme_name);
                self.theme = if self.kawaii_mode {
                    base_theme.apply_kawaii()
                } else {
                    base_theme
                };
            }
        }

        // Update tab title based on focused pane's cwd
        self.tabs[self.active_tab].update_title();
    }

    /// Update autocomplete suggestions for the focused pane
    fn update_suggestions(&mut self) {
        let tab = &mut self.tabs[self.active_tab];
        let pane_id = tab.splits.focused_pane_id();

        if let Some(pane) = tab.panes.get_mut(&pane_id) {
            // Skip if suppressed (just applied a suggestion)
            if pane.suppress_suggestions {
                pane.suppress_suggestions = false;
                return;
            }

            // Only update if input changed
            if pane.input == pane.last_suggestion_input {
                return;
            }

            pane.last_suggestion_input = pane.input.clone();

            // Get context-aware history suggestions from SmartHistory
            let cwd = pane.state.cwd().clone();
            let smart_suggestions = pane.history.suggest(&pane.input, &cwd, 5);

            // Convert to Vec<String> for autocomplete
            let history: Vec<String> = smart_suggestions;

            // Get suggestions (SmartHistory provides context-aware ordering)
            pane.suggestions = self.autocomplete.suggest(
                &pane.input,
                pane.input.len(),
                pane.state.cwd(),
                &history,
            );

            // Show suggestions if we have any and input is not empty
            pane.show_suggestions = !pane.suggestions.is_empty() && !pane.input.is_empty();
            pane.selected_suggestion = 0;

            // Reset history position when input changes
            pane.history.reset_position();
            pane.saved_input.clear();
        }
    }

    /// Apply the selected suggestion to the focused pane
    fn apply_suggestion(&mut self) {
        let tab = &mut self.tabs[self.active_tab];
        let pane_id = tab.splits.focused_pane_id();

        if let Some(pane) = tab.panes.get_mut(&pane_id) {
            // Apply if there are any suggestions (don't require show_suggestions flag)
            if !pane.suggestions.is_empty() {
                let idx = pane.selected_suggestion.min(pane.suggestions.len() - 1);
                let suggestion = pane.suggestions[idx].clone();
                let (new_input, _) = self.autocomplete.apply_suggestion(
                    &pane.input,
                    pane.input.len(),
                    &suggestion,
                );
                pane.input = new_input;
                pane.show_suggestions = false;
                pane.suppress_suggestions = true;
                pane.cursor_to_end = true;
                pane.last_suggestion_input = pane.input.clone();
            }
        }
    }

    /// Render the tab bar
    fn render_tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            let mut tab_to_close: Option<usize> = None;
            let mut tab_to_switch: Option<usize> = None;

            for (i, tab) in self.tabs.iter().enumerate() {
                let is_active = i == self.active_tab;

                let bg_color = if is_active {
                    self.theme.background
                } else {
                    self.theme.background_tertiary
                };

                let text_color = if is_active {
                    self.theme.foreground
                } else {
                    self.theme.comment_color
                };

                egui::Frame::new()
                    .fill(bg_color)
                    .inner_margin(egui::Margin::symmetric(8, 4))
                    .corner_radius(egui::CornerRadius {
                        nw: 4,
                        ne: 4,
                        sw: 0,
                        se: 0,
                    })
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Tab icon
                            let icon = if is_active { "◉" } else { "○" };
                            ui.add(egui::Label::new(
                                egui::RichText::new(icon)
                                    .color(self.theme.accent)
                                    .size(10.0),
                            ));

                            // Tab title (clickable)
                            let title_response = ui.add(egui::Label::new(
                                egui::RichText::new(&tab.title)
                                    .color(text_color)
                                    .size(12.0),
                            ).sense(egui::Sense::click()));

                            if title_response.clicked() {
                                tab_to_switch = Some(i);
                            }

                            // Close button (only if more than 1 tab)
                            if self.tabs.len() > 1 {
                                let close_response = ui.add(egui::Label::new(
                                    egui::RichText::new("×")
                                        .color(self.theme.comment_color)
                                        .size(14.0),
                                ).sense(egui::Sense::click()));

                                if close_response.clicked() {
                                    tab_to_close = Some(i);
                                }

                                if close_response.hovered() {
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                }
                            }
                        });
                    });
            }

            // New tab button
            let new_tab_response = ui.add(egui::Label::new(
                egui::RichText::new(" + ")
                    .color(self.theme.accent)
                    .size(14.0),
            ).sense(egui::Sense::click()));

            if new_tab_response.clicked() {
                self.new_tab();
            }

            if new_tab_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            // Handle tab actions after iteration
            if let Some(idx) = tab_to_switch {
                self.active_tab = idx;
            }
            if let Some(idx) = tab_to_close {
                self.tabs.remove(idx);
                if self.active_tab >= self.tabs.len() {
                    self.active_tab = self.tabs.len().saturating_sub(1);
                }
            }
        });
    }
}

impl eframe::App for ZaxiomApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if we should exit
        if self.should_exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Track search toggle
        let mut toggle_search = false;
        let mut search_next = false;
        let mut search_prev = false;
        let mut split_horizontal = false;
        let mut split_vertical = false;
        let mut copy_input = false;
        let mut paste_input = false;
        let mut paste_raw = false;
        let mut interrupt_input = false;
        let mut clear_screen = false;
        let mut clear_line_to_start = false;
        let mut insert_last_arg = false;
        let mut close_pane = false;
        let mut focus_next_pane = false;
        let mut focus_prev_pane = false;
        let mut toggle_hints = false;
        let mut toggle_vi_mode = false;
        let mut hints_filter_char: Option<char> = None;
        let mut hints_backspace = false;
        let mut hints_tab = false;
        let mut hints_enter = false;
        let mut vi_key_char: Option<char> = None;
        let mut vi_ctrl_key: Option<char> = None;
        let mut palette_up = false;
        let mut palette_down = false;
        let mut palette_enter = false;
        let mut palette_ctrl_enter = false;
        let mut palette_escape = false;
        let mut palette_char: Option<char> = None;
        let mut palette_backspace = false;
        let mut editor_save = false;
        let mut editor_exit = false;
        let mut editor_char: Option<char> = None;
        let mut editor_backspace = false;
        let mut editor_enter = false;
        let mut editor_up = false;
        let mut editor_down = false;
        let mut editor_left = false;
        let mut editor_right = false;
        let mut editor_page_up = false;
        let mut editor_page_down = false;
        let mut editor_home = false;
        let mut editor_end = false;
        let mut editor_ctrl_home = false;
        let mut editor_ctrl_end = false;
        let editor_is_open = self.editor.is_some();
        let palette_was_open = self.command_palette.is_open;
        let mut fuzzy_history = false;
        let mut fuzzy_files = false;
        let mut fuzzy_branches = false;
        let mut fuzzy_up = false;
        let mut fuzzy_down = false;
        let mut fuzzy_enter = false;
        let mut fuzzy_ctrl_enter = false;
        let mut fuzzy_escape = false;
        let mut fuzzy_char: Option<char> = None;
        let mut fuzzy_backspace = false;

        // Check focused pane's mode states
        let focused_in_search = self.tabs[self.active_tab]
            .focused_pane()
            .map(|p| p.search_mode)
            .unwrap_or(false);
        let focused_in_hints = self.tabs[self.active_tab]
            .focused_pane()
            .map(|p| p.hints_mode.active)
            .unwrap_or(false);
        let focused_in_vi = self.tabs[self.active_tab]
            .focused_pane()
            .map(|p| p.vi_mode.active)
            .unwrap_or(false);
        let focused_in_fuzzy = self.tabs[self.active_tab]
            .focused_pane()
            .map(|p| p.fuzzy_finder.active)
            .unwrap_or(false);

        // Handle keyboard shortcuts
        ctx.input(|i| {
            // Handle editor keyboard when open - editor consumes all input FIRST
            if editor_is_open {
                if i.key_pressed(egui::Key::Escape) { editor_exit = true; }
                if i.modifiers.ctrl && i.key_pressed(egui::Key::X) { editor_exit = true; }
                if i.modifiers.ctrl && i.key_pressed(egui::Key::S) { editor_save = true; }
                if i.key_pressed(egui::Key::Backspace) { editor_backspace = true; }
                if i.key_pressed(egui::Key::Enter) { editor_enter = true; }
                if i.key_pressed(egui::Key::ArrowUp) { editor_up = true; }
                if i.key_pressed(egui::Key::ArrowDown) { editor_down = true; }
                if i.key_pressed(egui::Key::ArrowLeft) { editor_left = true; }
                if i.key_pressed(egui::Key::ArrowRight) { editor_right = true; }
                if i.key_pressed(egui::Key::PageUp) { editor_page_up = true; }
                if i.key_pressed(egui::Key::PageDown) { editor_page_down = true; }
                if i.key_pressed(egui::Key::Home) {
                    if i.modifiers.ctrl { editor_ctrl_home = true; } else { editor_home = true; }
                }
                if i.key_pressed(egui::Key::End) {
                    if i.modifiers.ctrl { editor_ctrl_end = true; } else { editor_end = true; }
                }
                // Capture text input
                for event in &i.events {
                    if let egui::Event::Text(text) = event {
                        for ch in text.chars() {
                            editor_char = Some(ch);
                        }
                    }
                }
                // Early return - editor consumes all keyboard input
                return;
            }

            // Ctrl+T: New tab
            if i.modifiers.ctrl && i.key_pressed(egui::Key::T) {
                self.new_tab();
            }
            // Ctrl+P: Toggle command palette
            if i.modifiers.ctrl && i.key_pressed(egui::Key::P) {
                self.command_palette.toggle();
            }
            // Handle command palette keyboard when open - palette consumes all input
            if self.command_palette.is_open {
                if i.key_pressed(egui::Key::Escape) { palette_escape = true; }
                if i.key_pressed(egui::Key::Enter) {
                    if i.modifiers.ctrl {
                        palette_ctrl_enter = true;
                    } else {
                        palette_enter = true;
                    }
                }
                if i.key_pressed(egui::Key::ArrowUp) { palette_up = true; }
                if i.key_pressed(egui::Key::ArrowDown) { palette_down = true; }
                if i.key_pressed(egui::Key::Backspace) { palette_backspace = true; }
                // Capture letter/number keys for filtering
                for (key, ch) in [
                    (egui::Key::A, 'a'), (egui::Key::B, 'b'), (egui::Key::C, 'c'),
                    (egui::Key::D, 'd'), (egui::Key::E, 'e'), (egui::Key::F, 'f'),
                    (egui::Key::G, 'g'), (egui::Key::H, 'h'), (egui::Key::I, 'i'),
                    (egui::Key::J, 'j'), (egui::Key::K, 'k'), (egui::Key::L, 'l'),
                    (egui::Key::M, 'm'), (egui::Key::N, 'n'), (egui::Key::O, 'o'),
                    (egui::Key::P, 'p'), (egui::Key::Q, 'q'), (egui::Key::R, 'r'),
                    (egui::Key::S, 's'), (egui::Key::T, 't'), (egui::Key::U, 'u'),
                    (egui::Key::V, 'v'), (egui::Key::W, 'w'), (egui::Key::X, 'x'),
                    (egui::Key::Y, 'y'), (egui::Key::Z, 'z'),
                    (egui::Key::Num0, '0'), (egui::Key::Num1, '1'), (egui::Key::Num2, '2'),
                    (egui::Key::Num3, '3'), (egui::Key::Num4, '4'), (egui::Key::Num5, '5'),
                    (egui::Key::Num6, '6'), (egui::Key::Num7, '7'), (egui::Key::Num8, '8'),
                    (egui::Key::Num9, '9'),
                ] {
                    if i.key_pressed(key) && !i.modifiers.ctrl {
                        palette_char = Some(ch);
                        break;
                    }
                }
                if i.key_pressed(egui::Key::Space) { palette_char = Some(' '); }
                if i.key_pressed(egui::Key::Minus) { palette_char = Some('-'); }
                // Early return - palette consumes all keyboard input
                return;
            }
            // Ctrl+W: Close current tab (or pane if multiple)
            if i.modifiers.ctrl && i.key_pressed(egui::Key::W) {
                if self.tabs[self.active_tab].pane_count() > 1 {
                    close_pane = true;
                } else {
                    self.close_current_tab();
                }
            }
            // Ctrl+F: Toggle search
            if i.modifiers.ctrl && i.key_pressed(egui::Key::F) {
                toggle_search = true;
            }
            // Escape: Close search if open
            if i.key_pressed(egui::Key::Escape) && focused_in_search {
                toggle_search = true;
            }
            // Enter in search: next match, Shift+Enter: previous match
            if focused_in_search && i.key_pressed(egui::Key::Enter) {
                if i.modifiers.shift {
                    search_prev = true;
                } else {
                    search_next = true;
                }
            }
            // Ctrl+Tab: Next tab
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Tab) {
                if i.modifiers.shift {
                    self.prev_tab();
                } else {
                    self.next_tab();
                }
            }
            // Ctrl+Shift+D: Split horizontal
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::D) {
                split_horizontal = true;
            }
            // Ctrl+Shift+E: Split vertical
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::E) {
                split_vertical = true;
            }
            // Alt+Arrow: Navigate between panes
            if i.modifiers.alt && i.key_pressed(egui::Key::ArrowRight) {
                focus_next_pane = true;
            }
            if i.modifiers.alt && i.key_pressed(egui::Key::ArrowLeft) {
                focus_prev_pane = true;
            }
            // Ctrl+Shift+C: Copy current input (terminal style)
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::C) {
                copy_input = true;
            }
            // Ctrl+C: Interrupt (clear current line, like real terminal)
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::C) {
                interrupt_input = true;
            }
            // Ctrl+V: Paste (clean - strip newlines)
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::V) {
                paste_input = true;
            }
            // Ctrl+Shift+V: Paste raw (keep newlines)
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::V) {
                paste_raw = true;
            }
            // Ctrl+L: Clear screen (like real terminal)
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::L) {
                clear_screen = true;
            }
            // Ctrl+U: Clear line from cursor to beginning
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::U) {
                clear_line_to_start = true;
            }
            // Alt+.: Insert last argument from previous command
            if i.modifiers.alt && i.key_pressed(egui::Key::Period) {
                insert_last_arg = true;
            }
            // Ctrl+Shift+H: Toggle hints mode
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::H) {
                toggle_hints = true;
            }
            // Escape: Exit hints/vi/fuzzy mode
            if i.key_pressed(egui::Key::Escape) {
                if focused_in_fuzzy {
                    fuzzy_escape = true;
                } else if focused_in_hints {
                    toggle_hints = true;
                } else if focused_in_vi {
                    toggle_vi_mode = true;
                }
            }
            // Ctrl+Shift+M: Toggle vi mode (like Alacritty)
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::M) {
                toggle_vi_mode = true;
            }
            // Ctrl+R: Fuzzy history search
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::R) && !focused_in_fuzzy {
                fuzzy_history = true;
            }
            // Ctrl+Shift+F: Fuzzy file search (Ctrl+F is for search in buffer)
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::F) {
                fuzzy_files = true;
            }
            // Ctrl+G: Fuzzy git branches
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::G) && !focused_in_fuzzy {
                fuzzy_branches = true;
            }
            // Handle fuzzy finder keyboard input - fuzzy finder consumes all input
            if focused_in_fuzzy {
                if i.key_pressed(egui::Key::ArrowUp) { fuzzy_up = true; }
                if i.key_pressed(egui::Key::ArrowDown) { fuzzy_down = true; }
                if i.key_pressed(egui::Key::Enter) {
                    if i.modifiers.ctrl {
                        fuzzy_ctrl_enter = true;
                    } else {
                        fuzzy_enter = true;
                    }
                }
                if i.key_pressed(egui::Key::Backspace) { fuzzy_backspace = true; }
                // Capture text input for query
                for event in &i.events {
                    if let egui::Event::Text(text) = event {
                        for ch in text.chars() {
                            if !ch.is_control() {
                                fuzzy_char = Some(ch);
                            }
                        }
                    }
                }
                // Early return - fuzzy finder consumes all keyboard input
                return;
            }
            // Handle hints mode keyboard input
            if focused_in_hints && !i.modifiers.ctrl && !i.modifiers.alt {
                // Backspace to remove last filter character
                if i.key_pressed(egui::Key::Backspace) {
                    hints_backspace = true;
                }
                // Tab to cycle through hints
                if i.key_pressed(egui::Key::Tab) {
                    hints_tab = true;
                }
                // Enter to select current hint
                if i.key_pressed(egui::Key::Enter) {
                    hints_enter = true;
                }
                // Capture letter keys for filtering
                for (key, ch) in [
                    (egui::Key::A, 'a'), (egui::Key::B, 'b'), (egui::Key::C, 'c'),
                    (egui::Key::D, 'd'), (egui::Key::E, 'e'), (egui::Key::F, 'f'),
                    (egui::Key::G, 'g'), (egui::Key::H, 'h'), (egui::Key::I, 'i'),
                    (egui::Key::J, 'j'), (egui::Key::K, 'k'), (egui::Key::L, 'l'),
                    (egui::Key::M, 'm'), (egui::Key::N, 'n'), (egui::Key::O, 'o'),
                    (egui::Key::P, 'p'), (egui::Key::Q, 'q'), (egui::Key::R, 'r'),
                    (egui::Key::S, 's'), (egui::Key::T, 't'), (egui::Key::U, 'u'),
                    (egui::Key::V, 'v'), (egui::Key::W, 'w'), (egui::Key::X, 'x'),
                    (egui::Key::Y, 'y'), (egui::Key::Z, 'z'),
                ] {
                    if i.key_pressed(key) {
                        hints_filter_char = Some(ch);
                        break;
                    }
                }
            }
            // Handle vi mode keyboard input
            if focused_in_vi && !i.modifiers.alt {
                // Ctrl key combinations
                if i.modifiers.ctrl {
                    if i.key_pressed(egui::Key::F) { vi_ctrl_key = Some('\x06'); } // Ctrl+F page down
                    else if i.key_pressed(egui::Key::B) { vi_ctrl_key = Some('\x02'); } // Ctrl+B page up
                    else if i.key_pressed(egui::Key::D) { vi_ctrl_key = Some('\x04'); } // Ctrl+D half page down
                    else if i.key_pressed(egui::Key::U) { vi_ctrl_key = Some('\x15'); } // Ctrl+U half page up
                    else if i.key_pressed(egui::Key::O) { vi_ctrl_key = Some('\x0f'); } // Ctrl+O jump back
                    else if i.key_pressed(egui::Key::I) { vi_ctrl_key = Some('\x09'); } // Ctrl+I jump forward
                } else {
                    // Regular keys
                    for (key, ch) in [
                        (egui::Key::A, 'a'), (egui::Key::B, 'b'), (egui::Key::C, 'c'),
                        (egui::Key::D, 'd'), (egui::Key::E, 'e'), (egui::Key::F, 'f'),
                        (egui::Key::G, 'g'), (egui::Key::H, 'h'), (egui::Key::I, 'i'),
                        (egui::Key::J, 'j'), (egui::Key::K, 'k'), (egui::Key::L, 'l'),
                        (egui::Key::M, 'm'), (egui::Key::N, 'n'), (egui::Key::O, 'o'),
                        (egui::Key::P, 'p'), (egui::Key::Q, 'q'), (egui::Key::R, 'r'),
                        (egui::Key::S, 's'), (egui::Key::T, 't'), (egui::Key::U, 'u'),
                        (egui::Key::V, 'v'), (egui::Key::W, 'w'), (egui::Key::X, 'x'),
                        (egui::Key::Y, 'y'), (egui::Key::Z, 'z'),
                    ] {
                        if i.key_pressed(key) {
                            vi_key_char = Some(if i.modifiers.shift { ch.to_ascii_uppercase() } else { ch });
                            break;
                        }
                    }
                    // Number keys
                    for (key, ch) in [
                        (egui::Key::Num0, '0'), (egui::Key::Num1, '1'), (egui::Key::Num2, '2'),
                        (egui::Key::Num3, '3'), (egui::Key::Num4, '4'), (egui::Key::Num5, '5'),
                        (egui::Key::Num6, '6'), (egui::Key::Num7, '7'), (egui::Key::Num8, '8'),
                        (egui::Key::Num9, '9'),
                    ] {
                        if i.key_pressed(key) {
                            vi_key_char = Some(ch);
                            break;
                        }
                    }
                    // Special keys
                    if i.key_pressed(egui::Key::Escape) { vi_key_char = Some('\x1b'); }
                    if i.key_pressed(egui::Key::Enter) { vi_key_char = Some('\n'); }
                    if i.key_pressed(egui::Key::Backspace) { vi_key_char = Some('\x08'); }
                    if i.key_pressed(egui::Key::Minus) { vi_key_char = Some('-'); }
                    if i.key_pressed(egui::Key::Space) { vi_key_char = Some(' '); }
                    // Shift+4 = $, Shift+6 = ^, Shift+/ = ?
                    if i.modifiers.shift {
                        if i.key_pressed(egui::Key::Num4) { vi_key_char = Some('$'); }
                        if i.key_pressed(egui::Key::Num6) { vi_key_char = Some('^'); }
                        if i.key_pressed(egui::Key::Slash) { vi_key_char = Some('?'); }
                    } else {
                        if i.key_pressed(egui::Key::Slash) { vi_key_char = Some('/'); }
                    }
                }
            }
            // Ctrl+1-9: Switch to specific tab
            for (key, idx) in [
                (egui::Key::Num1, 0),
                (egui::Key::Num2, 1),
                (egui::Key::Num3, 2),
                (egui::Key::Num4, 3),
                (egui::Key::Num5, 4),
                (egui::Key::Num6, 5),
                (egui::Key::Num7, 6),
                (egui::Key::Num8, 7),
                (egui::Key::Num9, 8),
            ] {
                if i.modifiers.ctrl && i.key_pressed(key) && idx < self.tabs.len() {
                    self.active_tab = idx;
                }
            }
        });

        // Handle search toggle/navigation
        if toggle_search {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.toggle_search();
            }
        }
        if search_next {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.next_match();
            }
        }
        if search_prev {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.prev_match();
            }
        }

        // Handle split pane actions
        if split_horizontal {
            self.tabs[self.active_tab].split(SplitDirection::Horizontal);
        }
        if split_vertical {
            self.tabs[self.active_tab].split(SplitDirection::Vertical);
        }
        if close_pane {
            self.tabs[self.active_tab].close_focused_pane();
        }
        if focus_next_pane {
            self.tabs[self.active_tab].splits.focus_next();
        }
        if focus_prev_pane {
            self.tabs[self.active_tab].splits.focus_prev();
        }

        // Handle clipboard actions
        if copy_input {
            self.copy_current_input();
        }
        if paste_input {
            self.paste_to_input();
        }
        if paste_raw {
            self.paste_raw_to_input();
        }

        // Handle Ctrl+C interrupt (clear line like real terminal)
        if interrupt_input {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if !pane.input.is_empty() {
                    // Show ^C and the interrupted input
                    let interrupted = format!("{}^C", pane.input);
                    pane.buffer.push_line(&interrupted);
                    pane.input.clear();
                    self.clipboard_feedback = Some((
                        "^C".to_string(),
                        std::time::Instant::now(),
                    ));
                } else {
                    // Empty input, just show ^C
                    pane.buffer.push_line("^C");
                    self.clipboard_feedback = Some((
                        "^C (interrupt)".to_string(),
                        std::time::Instant::now(),
                    ));
                }
            }
        }

        // Handle Ctrl+L clear screen (same as clear command - show logo after)
        if clear_screen {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.buffer.clear();
                // Show just the ASCII logo after clear (like the clear command)
                for line in ASCII_LOGO.lines() {
                    pane.buffer.push_line(line);
                }
            }
        }

        // Handle Ctrl+U clear line to beginning
        if clear_line_to_start {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.input.clear();
            }
        }

        // Handle Alt+. insert last argument
        if insert_last_arg {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                // Get last command from history and extract last argument
                if let Some(last_entry) = pane.history.all().last() {
                    let parts: Vec<&str> = last_entry.command.split_whitespace().collect();
                    if let Some(last_arg) = parts.last() {
                        pane.input.push_str(last_arg);
                    }
                }
            }
        }

        // Handle hints mode toggle
        if toggle_hints {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if pane.hints_mode.active {
                    pane.hints_mode.deactivate();
                } else {
                    // Extract hints from visible buffer content
                    let extractor = HintsExtractor::new();
                    let mut all_hints = Vec::new();
                    for (line_num, line) in pane.buffer.output_lines().enumerate() {
                        let hints = extractor.extract(&line.text, line_num);
                        all_hints.extend(hints);
                    }
                    pane.hints_mode.activate(all_hints);
                }
            }
        }

        // Handle hints filter input
        if let Some(ch) = hints_filter_char {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if pane.hints_mode.active {
                    pane.hints_mode.update_filter(ch);
                }
            }
        }
        if hints_backspace {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if pane.hints_mode.active {
                    pane.hints_mode.backspace();
                }
            }
        }

        // Handle hints Tab (cycle through hints)
        if hints_tab {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if pane.hints_mode.active && !pane.hints_mode.hints.is_empty() {
                    let len = pane.hints_mode.hints.len();
                    pane.hints_mode.selected = Some(match pane.hints_mode.selected {
                        Some(i) => (i + 1) % len,
                        None => 0,
                    });
                    // Request scroll to selected hint
                    pane.hints_scroll_to_selected = true;
                }
            }
        }

        // Handle hints Enter (select current hint)
        let mut hints_enter_action: Option<(String, HintType)> = None;
        if hints_enter {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if pane.hints_mode.active {
                    if let Some(idx) = pane.hints_mode.selected {
                        if let Some(hint) = pane.hints_mode.hints.get(idx) {
                            hints_enter_action = Some((hint.text.clone(), hint.hint_type.clone()));
                        }
                    } else if pane.hints_mode.hints.len() == 1 {
                        // Auto-select if only one hint
                        let hint = &pane.hints_mode.hints[0];
                        hints_enter_action = Some((hint.text.clone(), hint.hint_type.clone()));
                    }
                }
            }
        }

        // Execute hints enter action
        if let Some((text, hint_type)) = hints_enter_action {
            match hint_type {
                HintType::Url => {
                    let _ = open::that(&text);
                    self.clipboard_feedback = Some((
                        format!("🌐 Opening~ {}", if text.len() > 40 { format!("{}...", &text[..37]) } else { text }),
                        std::time::Instant::now(),
                    ));
                }
                _ => {
                    if let Some(ref mut clipboard) = self.clipboard {
                        if clipboard.set_text(text.clone()).is_ok() {
                            let preview = if text.len() > 40 {
                                format!("{}...", &text[..37])
                            } else {
                                text.clone()
                            };
                            self.clipboard_feedback = Some((
                                format!("🔗 Copied~ {}", preview),
                                std::time::Instant::now(),
                            ));
                        }
                    }
                }
            }
            // Deactivate hints mode
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.hints_mode.deactivate();
            }
        }

        // Handle vi mode toggle
        if toggle_vi_mode {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if pane.vi_mode.active {
                    pane.vi_mode.exit();
                } else {
                    let total_lines = pane.buffer.len();
                    pane.vi_mode.enter(total_lines);
                }
            }
        }

        // Handle vi mode key input
        let vi_action = if let Some(ch) = vi_key_char.or(vi_ctrl_key) {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if pane.vi_mode.active {
                    // Update max_col based on current line
                    if let Some(line) = pane.buffer.get_line(pane.vi_mode.cursor.line) {
                        pane.vi_mode.max_col = line.text.len().saturating_sub(1);
                    }
                    Some(pane.vi_mode.handle_key(ch))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Process vi mode action
        if let Some(action) = vi_action {
            match action {
                ViAction::Exit => {
                    // Already handled by handle_key
                }
                ViAction::MoveCursor => {
                    // Cursor movement handled in vi_mode, overlay will render the cursor
                }
                ViAction::ScrollDown(n) => {
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        pane.vi_mode.cursor.line = pane.vi_mode.cursor.line.saturating_sub(n);
                    }
                }
                ViAction::ScrollUp(n) => {
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        let max = pane.vi_mode.total_lines.saturating_sub(1);
                        pane.vi_mode.cursor.line = (pane.vi_mode.cursor.line + n).min(max);
                    }
                }
                ViAction::YankLine | ViAction::YankSelection | ViAction::YankToEnd => {
                    // Copy to clipboard
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        if let Some(line) = pane.buffer.get_line(pane.vi_mode.cursor.line) {
                            let text = match action {
                                ViAction::YankToEnd => {
                                    line.text[pane.vi_mode.cursor.col..].to_string()
                                }
                                _ => line.text.clone(),
                            };
                            pane.vi_mode.yank_register = text.clone();
                            if let Some(ref mut clipboard) = self.clipboard {
                                let _ = clipboard.set_text(text.clone());
                                self.clipboard_feedback = Some((
                                    format!("📋 Yanked: {}", if text.len() > 30 { format!("{}...", &text[..27]) } else { text }),
                                    std::time::Instant::now(),
                                ));
                            }
                        }
                    }
                }
                ViAction::StartSelection => {
                    // Selection is started in handle_key
                }
                ViAction::UpdateSelection | ViAction::CancelSelection => {
                    // Handled by vi_mode internally
                }
                ViAction::StartSearch => {
                    // Search UI will be shown in vi mode overlay
                }
                ViAction::ExecuteSearch(query) => {
                    // Find matches in buffer
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        let mut matches = Vec::new();
                        for (i, line) in pane.buffer.lines().enumerate() {
                            if let Some(pos) = line.find(&query) {
                                matches.push((i, pos, pos + query.len()));
                            }
                        }
                        pane.vi_mode.set_search_matches(matches);
                    }
                }
                ViAction::JumpToMatch => {
                    // Jump is handled in vi_mode
                }
                ViAction::OpenUnderCursor => {
                    // Try to open URL or path under cursor
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane() {
                        if let Some(line) = pane.buffer.get_line(pane.vi_mode.cursor.line) {
                            // Simple extraction - find word under cursor
                            let text = &line.text;
                            if text.contains("http://") || text.contains("https://") {
                                // Find URL
                                if let Some(start) = text.find("http") {
                                    let end = text[start..].find(|c: char| c.is_whitespace()).unwrap_or(text.len() - start);
                                    let url = &text[start..start + end];
                                    let _ = open::that(url);
                                    self.clipboard_feedback = Some((
                                        format!("🌐 Opening: {}", url),
                                        std::time::Instant::now(),
                                    ));
                                }
                            }
                        }
                    }
                }
                ViAction::Paste => {
                    // Paste not applicable in read-only vi mode
                }
                _ => {}
            }
        }

        // Handle fuzzy finder activation
        if fuzzy_history {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                let cwd = pane.state.cwd().to_path_buf();
                pane.fuzzy_finder.activate(FuzzyMode::History, &cwd);
                // Populate with history items
                let history_items: Vec<(String, Option<String>)> = pane.history.all()
                    .map(|e| (e.command.clone(), Some(e.cwd.display().to_string())))
                    .collect();
                pane.fuzzy_finder.set_history_items(history_items);
            }
        }
        if fuzzy_files {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                let cwd = pane.state.cwd().to_path_buf();
                pane.fuzzy_finder.activate(FuzzyMode::Files, &cwd);
            }
        }
        if fuzzy_branches {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                let cwd = pane.state.cwd().to_path_buf();
                pane.fuzzy_finder.activate(FuzzyMode::GitBranches, &cwd);
            }
        }

        // Handle fuzzy finder input
        if fuzzy_escape {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.fuzzy_finder.deactivate();
            }
        }
        if fuzzy_up {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.fuzzy_finder.select_up();
            }
        }
        if fuzzy_down {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.fuzzy_finder.select_down();
            }
        }
        if fuzzy_backspace {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.fuzzy_finder.pop_char();
            }
        }
        if let Some(ch) = fuzzy_char {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.fuzzy_finder.push_char(ch);
            }
        }

        // Handle fuzzy finder selection
        let mut fuzzy_action: Option<FuzzyAction> = None;
        if fuzzy_enter || fuzzy_ctrl_enter {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if let Some(item) = pane.fuzzy_finder.get_selected() {
                    if fuzzy_ctrl_enter {
                        fuzzy_action = Some(FuzzyAction::Execute(item.value.clone()));
                    } else {
                        fuzzy_action = Some(FuzzyAction::Insert(item.value.clone()));
                    }
                }
                pane.fuzzy_finder.deactivate();
            }
        }

        // Process fuzzy finder action
        if let Some(action) = fuzzy_action {
            match action {
                FuzzyAction::Insert(value) => {
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        pane.input = value;
                        pane.cursor_to_end = true;
                    }
                }
                FuzzyAction::Execute(value) => {
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        pane.input = value.clone();
                        // Execute immediately
                        let cwd_display = pane.state.cwd().display().to_string();
                        let prompt = format!("{} ❯ {}", cwd_display, value);
                        pane.buffer.push_line(&prompt);
                        let history_cmds = pane.history.recent_commands(10);
                        match self.executor.execute_with_history(&value, &mut pane.state, Some(&history_cmds)) {
                            Ok(output) => {
                                for line in output.lines() {
                                    pane.buffer.push_line(line);
                                }
                            }
                            Err(e) => {
                                pane.buffer.push_error(&format!("Error: {}", e));
                            }
                        }
                        pane.history.add(&value, pane.state.cwd().to_path_buf(), None);
                        pane.input.clear();
                        pane.scroll_to_bottom = true;
                    }
                }
                _ => {}
            }
        }

        // Handle command palette actions
        let mut palette_command: Option<String> = None;
        if palette_escape {
            self.command_palette.close();
        }
        if palette_up {
            self.command_palette.select_up();
        }
        if palette_down {
            self.command_palette.select_down();
        }
        if palette_backspace {
            self.command_palette.query.pop();
            self.command_palette.update_search();
        }
        if let Some(ch) = palette_char {
            self.command_palette.query.push(ch);
            self.command_palette.update_search();
        }
        // Enter: copy command to input (don't execute)
        if palette_enter && self.command_palette.is_open {
            if let Some(cmd) = self.command_palette.get_selected_command() {
                // Just copy to input, don't execute
                if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                    pane.input = cmd;
                }
            }
            self.command_palette.close();
        }

        // Ctrl+Enter: execute command immediately
        if palette_ctrl_enter && self.command_palette.is_open {
            if let Some(cmd) = self.command_palette.get_selected_command() {
                palette_command = Some(cmd);
            }
            self.command_palette.close();
        }

        // Execute palette command (only on Ctrl+Enter)
        if let Some(cmd) = palette_command {
            // Handle special actions
            match cmd.as_str() {
                "New Tab" => self.new_tab(),
                "Close Tab" => self.close_current_tab(),
                "Search" => {
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        pane.toggle_search();
                    }
                }
                "Clear" => {
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        pane.buffer.clear();
                        for line in ASCII_LOGO.lines() {
                            pane.buffer.push_line(line);
                        }
                    }
                }
                _ => {
                    // Execute as terminal command
                    if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                        let cwd = pane.state.cwd().clone();
                        pane.history.add(&cmd, cwd, None);
                        pane.history.reset_position();
                        pane.saved_input.clear();
                        pane.buffer.start_block(&cmd);
                        let prompt = pane.state.format_prompt();
                        pane.buffer.push_line(&format!("{}{}", prompt, cmd));
                        let history = pane.history.recent_commands(10);
                        match self.executor.execute_with_history(&cmd, &mut pane.state, Some(&history)) {
                            Ok(output) => {
                                if !output.is_empty() && !output.starts_with("\x1b[") {
                                    for line in output.lines() {
                                        pane.buffer.push_line(line);
                                    }
                                }
                            }
                            Err(e) => {
                                pane.buffer.push_error(&e.to_string());
                            }
                        }
                    }
                }
            }
        }

        // Handle editor actions
        if let Some(ref mut editor) = self.editor {
            if editor_exit {
                // Close editor without saving
                self.editor = None;
                if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                    pane.buffer.push_line("📝 Editor closed.");
                }
            } else if editor_save {
                // Save file
                match editor.save() {
                    Ok(()) => {
                        if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                            pane.buffer.push_line(&format!("✅ Saved: {}", editor.file_path.display()));
                        }
                    }
                    Err(e) => {
                        if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                            pane.buffer.push_error(&format!("Failed to save: {}", e));
                        }
                    }
                }
            } else if editor_enter {
                // Insert newline
                let lines: Vec<&str> = editor.content.lines().collect();
                let mut new_content = String::new();
                for (i, line) in lines.iter().enumerate() {
                    new_content.push_str(line);
                    if i == editor.cursor_line {
                        // Split line at cursor
                        new_content.push('\n');
                    }
                    if i < lines.len() - 1 {
                        new_content.push('\n');
                    }
                }
                if editor.cursor_line >= lines.len() {
                    new_content.push('\n');
                }
                editor.content = new_content;
                editor.cursor_line += 1;
                editor.cursor_col = 0;
                editor.modified = true;
            } else if editor_backspace {
                // Delete character before cursor
                let lines: Vec<&str> = editor.content.lines().collect();
                if editor.cursor_col > 0 {
                    // Delete character in current line
                    if let Some(line) = lines.get(editor.cursor_line) {
                        let mut new_line = line.to_string();
                        if editor.cursor_col <= new_line.len() {
                            new_line.remove(editor.cursor_col - 1);
                            let mut new_content = String::new();
                            for (i, l) in lines.iter().enumerate() {
                                if i == editor.cursor_line {
                                    new_content.push_str(&new_line);
                                } else {
                                    new_content.push_str(l);
                                }
                                if i < lines.len() - 1 {
                                    new_content.push('\n');
                                }
                            }
                            editor.content = new_content;
                            editor.cursor_col -= 1;
                            editor.modified = true;
                        }
                    }
                } else if editor.cursor_line > 0 {
                    // Merge with previous line
                    editor.cursor_line -= 1;
                    editor.cursor_col = lines.get(editor.cursor_line).map(|l| l.len()).unwrap_or(0);
                    editor.modified = true;
                }
            } else if let Some(ch) = editor_char {
                // Insert character at cursor
                let lines: Vec<&str> = editor.content.lines().collect();
                let mut new_content = String::new();
                for (i, line) in lines.iter().enumerate() {
                    if i == editor.cursor_line {
                        let mut new_line = line.to_string();
                        let insert_pos = editor.cursor_col.min(new_line.len());
                        new_line.insert(insert_pos, ch);
                        new_content.push_str(&new_line);
                        editor.cursor_col += 1;
                    } else {
                        new_content.push_str(line);
                    }
                    if i < lines.len() - 1 {
                        new_content.push('\n');
                    }
                }
                // Handle empty file or cursor past last line
                if lines.is_empty() || editor.cursor_line >= lines.len() {
                    new_content.push(ch);
                    editor.cursor_col = 1;
                }
                editor.content = new_content;
                editor.modified = true;
            } else if editor_up {
                editor.cursor_up();
            } else if editor_down {
                editor.cursor_down();
            } else if editor_left && editor.cursor_col > 0 {
                editor.cursor_col -= 1;
            } else if editor_right {
                let lines: Vec<&str> = editor.content.lines().collect();
                if let Some(line) = lines.get(editor.cursor_line) {
                    if editor.cursor_col < line.len() {
                        editor.cursor_col += 1;
                    }
                }
            } else if editor_page_up {
                editor.page_up();
            } else if editor_page_down {
                editor.page_down();
            } else if editor_home {
                editor.go_to_line_start();
            } else if editor_end {
                editor.go_to_line_end();
            } else if editor_ctrl_home {
                editor.go_to_start();
            } else if editor_ctrl_end {
                editor.go_to_end();
            }
        }

        // Clear clipboard feedback after 2 seconds
        if let Some((_, instant)) = &self.clipboard_feedback {
            if instant.elapsed().as_secs() >= 2 {
                self.clipboard_feedback = None;
            }
        }

        // Update mascot animations
        self.mascot.update();

        // Apply theme colors
        let bg_color = self.theme.background;

        // Get window size for responsive layout
        let screen_rect = ctx.screen_rect();
        let window_width = screen_rect.width();
        let show_mascot = window_width > 900.0; // Hide mascot on narrow windows

        // Command Palette overlay (centered, modal)
        if self.command_palette.is_open {
            let palette_bg = self.theme.background_secondary;
            let palette_accent = self.theme.accent;
            let palette_fg = self.theme.foreground;
            let palette_comment = self.theme.comment_color;
            let palette_success = self.theme.success_color;

            egui::Area::new(egui::Id::new("command_palette"))
                .anchor(egui::Align2::CENTER_TOP, [0.0, 80.0])
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    egui::Frame::default()
                        .fill(palette_bg)
                        .stroke(egui::Stroke::new(2.0, palette_accent))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::same(12))
                        .shadow(egui::epaint::Shadow { spread: 8, blur: 16, color: egui::Color32::from_black_alpha(120), offset: [0, 4].into() })
                        .show(ui, |ui| {
                            ui.set_min_width(400.0);
                            ui.set_max_width(500.0);

                            // Search input display
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(egui::RichText::new("⌘").color(palette_accent).size(16.0)));
                                ui.add_space(8.0);
                                let query_display = if self.command_palette.query.is_empty() {
                                    "Type to search commands...".to_string()
                                } else {
                                    self.command_palette.query.clone()
                                };
                                ui.add(egui::Label::new(egui::RichText::new(&query_display)
                                    .color(if self.command_palette.query.is_empty() { palette_comment } else { palette_fg })
                                    .size(14.0).monospace()));
                            });

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);

                            // Results list (scrollable with auto-scroll to selection)
                            egui::ScrollArea::vertical().max_height(300.0).auto_shrink([false, false]).show(ui, |ui| {
                                for (i, entry) in self.command_palette.entries.iter().enumerate() {
                                    let is_selected = i == self.command_palette.selected;
                                    let bg = if is_selected { palette_accent.linear_multiply(0.3) } else { egui::Color32::TRANSPARENT };

                                    let response = egui::Frame::default().fill(bg).corner_radius(egui::CornerRadius::same(4)).inner_margin(egui::Margin::symmetric(8, 4)).show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            if is_selected {
                                                ui.add(egui::Label::new(egui::RichText::new("▶").color(palette_success).size(10.0)));
                                            } else {
                                                ui.add_space(14.0);
                                            }
                                            ui.add(egui::Label::new(egui::RichText::new(&entry.name).color(if is_selected { palette_accent } else { palette_fg }).size(13.0)));
                                            ui.add_space(8.0);
                                            ui.add(egui::Label::new(egui::RichText::new(&entry.description).color(palette_comment).size(11.0)));
                                            if let Some(ref shortcut) = entry.shortcut {
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    ui.add(egui::Label::new(egui::RichText::new(shortcut).color(palette_comment).size(10.0).monospace()));
                                                });
                                            }
                                        });
                                    });

                                    // Auto-scroll to keep selected item visible
                                    if is_selected {
                                        response.response.scroll_to_me(Some(egui::Align::Center));
                                    }
                                }
                                if self.command_palette.entries.is_empty() {
                                    ui.add(egui::Label::new(egui::RichText::new("No matching commands").color(palette_comment).size(12.0)));
                                }
                            });

                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(egui::RichText::new("↑↓ navigate  ↵ copy to input  ^↵ execute  esc close").color(palette_comment).size(10.0)));
                            });
                        });
                });
        }

        // Fuzzy finder overlay (bottom-anchored, like fzf)
        if let Some(pane) = self.tabs[self.active_tab].focused_pane() {
            if pane.fuzzy_finder.active {
                let fuzzy_bg = self.theme.background_secondary;
                let fuzzy_accent = self.theme.accent;
                let fuzzy_fg = self.theme.foreground;
                let fuzzy_comment = self.theme.comment_color;
                let fuzzy_success = self.theme.success_color;

                egui::Area::new(egui::Id::new("fuzzy_finder"))
                    .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -20.0])
                    .order(egui::Order::Foreground)
                    .show(ctx, |ui| {
                        egui::Frame::default()
                            .fill(fuzzy_bg)
                            .stroke(egui::Stroke::new(2.0, fuzzy_accent))
                            .corner_radius(egui::CornerRadius::same(8))
                            .inner_margin(egui::Margin::same(12))
                            .shadow(egui::epaint::Shadow { spread: 8, blur: 16, color: egui::Color32::from_black_alpha(120), offset: [0, -4].into() })
                            .show(ui, |ui| {
                                ui.set_min_width(500.0);
                                ui.set_max_width(600.0);

                                // Results list (bottom-up style - results above input)
                                egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                                    let items: Vec<_> = pane.fuzzy_finder.visible_items().collect();
                                    for (idx, item) in items.iter() {
                                        let is_selected = *idx == pane.fuzzy_finder.selected;
                                        let bg = if is_selected { fuzzy_accent.linear_multiply(0.3) } else { egui::Color32::TRANSPARENT };

                                        egui::Frame::default().fill(bg).corner_radius(egui::CornerRadius::same(4)).inner_margin(egui::Margin::symmetric(8, 4)).show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                // Selection indicator
                                                if is_selected {
                                                    ui.add(egui::Label::new(egui::RichText::new("▶").color(fuzzy_success).size(10.0)));
                                                } else {
                                                    ui.add_space(14.0);
                                                }

                                                // Icon
                                                ui.add(egui::Label::new(egui::RichText::new(item.icon).size(12.0)));

                                                // Display text with match highlighting
                                                let display_text = &item.display;
                                                if item.match_positions.is_empty() {
                                                    ui.add(egui::Label::new(egui::RichText::new(display_text).color(if is_selected { fuzzy_accent } else { fuzzy_fg }).size(13.0).monospace()));
                                                } else {
                                                    // Build highlighted text
                                                    let mut job = egui::text::LayoutJob::default();
                                                    let chars: Vec<char> = display_text.chars().collect();
                                                    for (i, ch) in chars.iter().enumerate() {
                                                        let color = if item.match_positions.contains(&i) {
                                                            fuzzy_accent
                                                        } else if is_selected {
                                                            fuzzy_fg
                                                        } else {
                                                            fuzzy_comment
                                                        };
                                                        job.append(&ch.to_string(), 0.0, egui::TextFormat {
                                                            font_id: egui::FontId::monospace(13.0),
                                                            color,
                                                            ..Default::default()
                                                        });
                                                    }
                                                    ui.label(job);
                                                }

                                                // Preview text (right-aligned)
                                                if let Some(ref preview) = item.preview {
                                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        ui.add(egui::Label::new(egui::RichText::new(preview).color(fuzzy_comment).size(10.0).monospace()));
                                                    });
                                                }
                                            });
                                        });
                                    }
                                    if pane.fuzzy_finder.items.is_empty() && !pane.fuzzy_finder.query.is_empty() {
                                        ui.add(egui::Label::new(egui::RichText::new("No matches found").color(fuzzy_comment).size(12.0)));
                                    }
                                });

                                ui.add_space(8.0);
                                ui.separator();
                                ui.add_space(4.0);

                                // Query input line
                                ui.horizontal(|ui| {
                                    ui.add(egui::Label::new(egui::RichText::new(pane.fuzzy_finder.mode_icon()).size(14.0)));
                                    ui.add(egui::Label::new(egui::RichText::new(pane.fuzzy_finder.mode_name()).color(fuzzy_accent).size(12.0).strong()));
                                    ui.add(egui::Label::new(egui::RichText::new(" > ").color(fuzzy_comment).size(12.0)));

                                    // Query with cursor
                                    let query_display = if pane.fuzzy_finder.query.is_empty() {
                                        "type to search...".to_string()
                                    } else {
                                        format!("{}▏", pane.fuzzy_finder.query)
                                    };
                                    ui.add(egui::Label::new(egui::RichText::new(&query_display)
                                        .color(if pane.fuzzy_finder.query.is_empty() { fuzzy_comment } else { fuzzy_fg })
                                        .size(13.0).monospace()));

                                    // Status (match count)
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.add(egui::Label::new(egui::RichText::new(&pane.fuzzy_finder.status_text())
                                            .color(fuzzy_comment).size(11.0).monospace()));
                                    });
                                });

                                ui.add_space(4.0);
                                // Keyboard hints
                                ui.horizontal(|ui| {
                                    ui.add(egui::Label::new(egui::RichText::new("↑↓ navigate  ↵ insert  ^↵ execute  esc close").color(fuzzy_comment).size(10.0)));
                                });
                            });
                    });
            }
        }

        // Editor overlay (full screen when editing a file)
        if let Some(ref editor) = self.editor {
            let editor_bg = self.theme.background;
            let editor_fg = self.theme.foreground;
            let editor_accent = self.theme.accent;
            let editor_comment = self.theme.comment_color;
            let line_num_color = self.theme.comment_color;
            let cursor_color = self.theme.accent;
            let modified_indicator = if editor.modified { " [+]" } else { "" };

            egui::Area::new(egui::Id::new("editor_overlay"))
                .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    let screen = ui.ctx().screen_rect();
                    egui::Frame::default()
                        .fill(editor_bg)
                        .show(ui, |ui| {
                            ui.set_min_size(screen.size());

                            // Header bar
                            egui::Frame::default()
                                .fill(self.theme.background_secondary)
                                .inner_margin(egui::Margin::symmetric(12, 8))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.add(egui::Label::new(
                                            egui::RichText::new("📝 nano")
                                                .color(editor_accent)
                                                .size(14.0)
                                                .strong(),
                                        ));
                                        ui.add(egui::Label::new(
                                            egui::RichText::new(format!(" — {}{}", editor.file_path.display(), modified_indicator))
                                                .color(editor_fg)
                                                .size(13.0),
                                        ));
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new("^X Exit  ^S Save")
                                                    .color(editor_comment)
                                                    .size(11.0),
                                            ));
                                        });
                                    });
                                });

                            // Editor content
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    ui.set_min_width(screen.width() - 20.0);

                                    let lines: Vec<&str> = editor.content.lines().collect();
                                    let line_count = lines.len().max(1);
                                    let line_num_width = (line_count as f32).log10().floor() as usize + 1;

                                    for (i, line) in lines.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            // Line number
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(format!("{:>width$} ", i + 1, width = line_num_width))
                                                    .color(line_num_color)
                                                    .size(13.0)
                                                    .monospace(),
                                            ));

                                            // Line content with cursor
                                            if i == editor.cursor_line {
                                                // Show cursor on this line
                                                let before = if editor.cursor_col <= line.len() {
                                                    &line[..editor.cursor_col]
                                                } else {
                                                    *line
                                                };
                                                let cursor_char = if editor.cursor_col < line.len() {
                                                    &line[editor.cursor_col..editor.cursor_col + 1]
                                                } else {
                                                    " "
                                                };
                                                let after = if editor.cursor_col + 1 < line.len() {
                                                    &line[editor.cursor_col + 1..]
                                                } else {
                                                    ""
                                                };

                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(before)
                                                        .color(editor_fg)
                                                        .size(13.0)
                                                        .monospace(),
                                                ));
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(cursor_char)
                                                        .color(editor_bg)
                                                        .background_color(cursor_color)
                                                        .size(13.0)
                                                        .monospace(),
                                                ));
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(after)
                                                        .color(editor_fg)
                                                        .size(13.0)
                                                        .monospace(),
                                                ));
                                            } else {
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(*line)
                                                        .color(editor_fg)
                                                        .size(13.0)
                                                        .monospace(),
                                                ));
                                            }
                                        });
                                    }

                                    // Show cursor on empty file or past last line
                                    if lines.is_empty() || editor.cursor_line >= lines.len() {
                                        ui.horizontal(|ui| {
                                            let line_num = if lines.is_empty() { 1 } else { lines.len() + 1 };
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(format!("{:>width$} ", line_num, width = line_num_width))
                                                    .color(line_num_color)
                                                    .size(13.0)
                                                    .monospace(),
                                            ));
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(" ")
                                                    .color(editor_bg)
                                                    .background_color(cursor_color)
                                                    .size(13.0)
                                                    .monospace(),
                                            ));
                                        });
                                    }
                                });

                            // Keyboard hints bar
                            egui::Frame::default()
                                .fill(self.theme.background_secondary)
                                .inner_margin(egui::Margin::symmetric(12, 8))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // Left side: position info
                                        ui.add(egui::Label::new(
                                            egui::RichText::new(format!("Line {}, Col {}", editor.cursor_line + 1, editor.cursor_col + 1))
                                                .color(editor_comment)
                                                .size(11.0),
                                        ));
                                        ui.add_space(20.0);

                                        // Show status message if any
                                        if !editor.status.is_empty() {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&editor.status)
                                                    .color(self.theme.success_color)
                                                    .size(11.0),
                                            ));
                                            ui.add_space(20.0);
                                        }

                                        // Keyboard shortcuts
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            for (keys, desc) in [
                                                ("Esc/^X", "exit"),
                                                ("^S", "save"),
                                                ("PgUp/Dn", "scroll"),
                                                ("Home/End", "line"),
                                                ("^Home/End", "file"),
                                            ].iter().rev() {
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(*desc)
                                                        .color(editor_comment)
                                                        .size(10.0),
                                                ));
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(*keys)
                                                        .color(editor_accent)
                                                        .size(10.0)
                                                        .strong(),
                                                ));
                                                ui.add_space(8.0);
                                            }
                                        });
                                    });
                                });
                        });
                });
        }

        // Mascot panel (top-right) - only show on wider windows
        if show_mascot {
            egui::Area::new(egui::Id::new("mascot_area"))
                .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 40.0))
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    self.mascot.render(ui);
                });
        }

        // Clipboard feedback toast (bottom-center)
        if let Some((message, _)) = &self.clipboard_feedback {
            let feedback_accent = self.theme.accent;
            let feedback_bg = self.theme.background_secondary;
            egui::Area::new(egui::Id::new("clipboard_feedback"))
                .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -60.0))
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    egui::Frame::default()
                        .fill(feedback_bg)
                        .stroke(egui::Stroke::new(1.0, feedback_accent))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::symmetric(16, 8))
                        .show(ui, |ui| {
                            ui.add(egui::Label::new(
                                egui::RichText::new(message)
                                    .color(feedback_accent)
                                    .size(14.0),
                            ));
                        });
                });
        }

        // Top panel for tab bar
        egui::TopBottomPanel::top("tab_bar")
            .frame(egui::Frame::default()
                .fill(self.theme.background_tertiary)
                .inner_margin(egui::Margin::symmetric(4, 2)))
            .show(ctx, |ui| {
                self.render_tab_bar(ui);
            });

        // Bottom status bar with git info
        let status_bar_height = 22.0;
        let (git_branch, cwd_display, block_count, pane_count, vi_status, hints_count) = {
            let tab = &self.tabs[self.active_tab];
            if let Some(pane) = tab.focused_pane() {
                let vi_status = if pane.vi_mode.active {
                    Some(pane.vi_mode.status_text())
                } else {
                    None
                };
                let hints_count = if pane.hints_mode.active {
                    pane.hints_mode.hints.len()
                } else {
                    0
                };
                (
                    crate::git::prompt::get_git_branch(pane.state.cwd()),
                    pane.state.cwd().display().to_string(),
                    pane.buffer.blocks().len(),
                    tab.pane_count(),
                    vi_status,
                    hints_count,
                )
            } else {
                (None, String::new(), 0, 1, None, 0)
            }
        };
        let status_accent = self.theme.accent;
        let status_fg = self.theme.foreground;
        let status_comment = self.theme.comment_color;
        let status_bg = self.theme.background_tertiary;

        egui::TopBottomPanel::bottom("status_bar")
            .frame(egui::Frame::default()
                .fill(status_bg)
                .inner_margin(egui::Margin::symmetric(8, 2)))
            .exact_height(status_bar_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Git branch (if in a git repo)
                    if let Some(branch) = &git_branch {
                        ui.add(egui::Label::new(
                            egui::RichText::new(" ")
                                .color(status_accent)
                                .size(12.0),
                        ));
                        ui.add(egui::Label::new(
                            egui::RichText::new(branch)
                                .color(status_fg)
                                .size(12.0),
                        ));
                        ui.add(egui::Label::new(
                            egui::RichText::new(" │ ")
                                .color(status_comment)
                                .size(12.0),
                        ));
                    }

                    // Current directory
                    ui.add(egui::Label::new(
                        egui::RichText::new(" ")
                            .color(status_accent)
                            .size(12.0),
                    ));
                    ui.add(egui::Label::new(
                        egui::RichText::new(&cwd_display)
                            .color(status_fg)
                            .size(12.0),
                    ));

                    // Spacer
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Block count
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!(" {} blocks", block_count))
                                .color(status_comment)
                                .size(12.0),
                        ));

                        // Pane count (if more than 1)
                        if pane_count > 1 {
                            ui.add(egui::Label::new(
                                egui::RichText::new(format!(" {} panes │", pane_count))
                                    .color(status_comment)
                                    .size(12.0),
                            ));
                        }

                        // Vi mode status
                        if let Some(vi_text) = &vi_status {
                            ui.add(egui::Label::new(
                                egui::RichText::new(format!(" │ {}", vi_text))
                                    .color(status_accent)
                                    .size(12.0),
                            ));
                        }

                        // Hints mode status
                        if hints_count > 0 {
                            ui.add(egui::Label::new(
                                egui::RichText::new(format!(" │ 🔗 {} hints", hints_count))
                                    .color(status_accent)
                                    .size(12.0),
                            ));
                        }

                        // Split shortcut hint (only when not in special mode)
                        if vi_status.is_none() && hints_count == 0 {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Ctrl+Shift+H: Hints │ Ctrl+Shift+M: Vi")
                                    .color(status_comment)
                                    .size(12.0),
                            ));
                        }
                    });
                });
            });

        // Search bar (if in search mode)
        let (search_mode, search_match_count, current_match) = {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane() {
                (pane.search_mode, pane.search_matches.len(), pane.current_match)
            } else {
                (false, 0, 0)
            }
        };
        let mut search_updated = false;

        if search_mode {
            let pane_id = self.tabs[self.active_tab].splits.focused_pane_id();
            let theme_accent = self.theme.accent;
            let theme_fg = self.theme.foreground;
            let theme_err = self.theme.error_color;
            let theme_comment = self.theme.comment_color;

            egui::TopBottomPanel::bottom("search_bar")
                .frame(egui::Frame::default()
                    .fill(self.theme.background_secondary)
                    .inner_margin(egui::Margin::symmetric(8, 4)))
                .show(ctx, |ui| {
                    if let Some(pane) = self.tabs[self.active_tab].panes.get_mut(&pane_id) {
                        ui.horizontal(|ui| {
                            ui.add(egui::Label::new(
                                egui::RichText::new("🔍 ")
                                    .color(theme_accent)
                                    .size(14.0),
                            ));

                            let response = ui.add(
                                egui::TextEdit::singleline(&mut pane.search_query)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_width(200.0)
                                    .hint_text("Search..."),
                            );

                            if response.changed() {
                                search_updated = true;
                            }

                            // Focus the search input
                            response.request_focus();

                            // Match count
                            if search_match_count > 0 {
                                ui.add(egui::Label::new(
                                    egui::RichText::new(format!(" {}/{} ", current_match + 1, search_match_count))
                                        .color(theme_fg)
                                        .size(12.0),
                                ));
                            } else if !pane.search_query.is_empty() {
                                ui.add(egui::Label::new(
                                    egui::RichText::new(" No matches ")
                                        .color(theme_err)
                                        .size(12.0),
                                ));
                            }

                            // Navigation hint
                            ui.add(egui::Label::new(
                                egui::RichText::new(" Enter: next, Shift+Enter: prev, Esc: close")
                                    .color(theme_comment)
                                    .size(11.0),
                            ));
                        });
                    }
                });
        }

        // Update search if query changed
        if search_updated {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.update_search();
            }
        }

        // Track if we need to execute a command after rendering
        let mut command_to_execute: Option<String> = None;
        let mut history_up = false;
        let mut history_down = false;
        let mut apply_suggestion = false;
        let mut close_suggestions = false;
        let mut suggestion_up = false;
        let mut suggestion_down = false;
        let mut input_changed = false;

        // Handle Tab/Escape for autocomplete at ctx level (before TextEdit consumes them)
        let has_suggestions = self.tabs[self.active_tab]
            .focused_pane()
            .map(|p| !p.suggestions.is_empty())
            .unwrap_or(false);
        let showing_suggestions = self.tabs[self.active_tab]
            .focused_pane()
            .map(|p| p.show_suggestions)
            .unwrap_or(false);

        if has_suggestions {
            if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab)) {
                apply_suggestion = true;
            }
        }
        if showing_suggestions {
            if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                close_suggestions = true;
            }
        }

        // Get focused pane ID for central panel
        let focused_pane_id = self.tabs[self.active_tab].splits.focused_pane_id();

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(bg_color))
            .show(ctx, |ui| {
                let foreground = self.theme.foreground;
                let path_color = self.theme.path_color;

                // Terminal output area (scrollable)
                let available_height = ui.available_height() - 30.0;

                // Get theme colors for output
                let error_color = self.theme.error_color;
                let success_color = self.theme.success_color;
                let command_color = self.theme.command_color;

                let link_color = self.theme.link_color;

                // Track block copy request
                let mut block_to_copy: Option<(usize, String)> = None;

                if let Some(pane) = self.tabs[self.active_tab].panes.get(&focused_pane_id) {
                    // Get block content map for copy buttons
                    let block_contents: std::collections::HashMap<usize, String> = pane.buffer.blocks()
                        .iter()
                        .filter_map(|b| {
                            pane.buffer.get_block_content(b.id)
                                .map(|content| (b.id, content))
                        })
                        .collect();

                    egui::ScrollArea::vertical()
                        .max_height(available_height)
                        .stick_to_bottom(true)
                        .auto_shrink([false; 2])
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                        .show(ui, |ui| {
                            // Leave margin on the right for mascot (only when visible)
                            let margin = if show_mascot { 90.0 } else { 10.0 };
                            ui.set_max_width(ui.available_width() - margin);

                            // Track which blocks have had their header rendered
                            let mut rendered_blocks = std::collections::HashSet::new();
                            let comment_color = self.theme.comment_color;
                            let accent_color = self.theme.accent;

                            // Render output buffer with proper colors based on line type
                            for line in pane.buffer.output_lines() {
                                let base_color = match line.line_type {
                                    LineType::Normal => foreground,
                                    LineType::Error => error_color,
                                    LineType::Command => command_color,
                                    LineType::Success => success_color,
                                };

                                // Check if this is a command line with a new block
                                if line.line_type == LineType::Command {
                                    if let Some(block_id) = line.block_id {
                                        if !rendered_blocks.contains(&block_id) {
                                            rendered_blocks.insert(block_id);

                                            // Render block header with copy button
                                            ui.horizontal(|ui| {
                                                // Copy button
                                                let copy_btn = ui.add(
                                                    egui::Button::new(
                                                        egui::RichText::new(" Copy")
                                                            .color(comment_color)
                                                            .size(11.0)
                                                    )
                                                    .frame(false)
                                                );

                                                if copy_btn.clicked() {
                                                    if let Some(content) = block_contents.get(&block_id) {
                                                        block_to_copy = Some((block_id, content.clone()));
                                                    }
                                                }

                                                if copy_btn.hovered() {
                                                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                                }

                                                // Show duration if available
                                                if let Some(block) = pane.buffer.get_block(block_id) {
                                                    if let Some(duration) = block.duration {
                                                        let dur_str = crate::terminal::buffer::format_duration(duration);
                                                        ui.add(egui::Label::new(
                                                            egui::RichText::new(format!(" {}", dur_str))
                                                                .color(accent_color)
                                                                .size(11.0)
                                                        ));
                                                    }
                                                }
                                            });
                                        }
                                    }
                                }

                                // Check if line contains ANSI codes
                                let has_ansi = ansi::has_ansi(&line.text);

                                // Check if line has URLs (only check on stripped text)
                                let has_urls = !line.urls.is_empty();

                                if has_ansi {
                                    // Parse and render ANSI-styled segments
                                    ui.horizontal(|ui| {
                                        let segments = ansi::parse_ansi(&line.text);
                                        for segment in segments {
                                            // Determine color: use ANSI color if present, otherwise base
                                            let color = segment.fg_color
                                                .map(|(r, g, b)| egui::Color32::from_rgb(r, g, b))
                                                .unwrap_or(base_color);

                                            let mut rich_text = egui::RichText::new(&segment.text)
                                                .monospace()
                                                .color(color);

                                            if segment.bold {
                                                rich_text = rich_text.strong();
                                            }
                                            if segment.italic {
                                                rich_text = rich_text.italics();
                                            }
                                            if segment.underline {
                                                rich_text = rich_text.underline();
                                            }

                                            ui.add(egui::Label::new(rich_text));
                                        }
                                    });
                                } else if has_urls {
                                    // Render with clickable URLs
                                    ui.horizontal(|ui| {
                                        let text = &line.text;
                                        let mut last_end = 0;

                                        for url_span in &line.urls {
                                            // Text before URL
                                            if url_span.start > last_end {
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(&text[last_end..url_span.start])
                                                        .monospace()
                                                        .color(base_color),
                                                ));
                                            }

                                            // Clickable URL
                                            let url_response = ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(&url_span.url)
                                                        .monospace()
                                                        .color(link_color)
                                                        .underline(),
                                                ).sense(egui::Sense::click())
                                            );

                                            if url_response.clicked() {
                                                // Open URL in browser
                                                let _ = open::that(&url_span.url);
                                            }

                                            if url_response.hovered() {
                                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                            }

                                            last_end = url_span.end;
                                        }

                                        // Text after last URL
                                        if last_end < text.len() {
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&text[last_end..])
                                                    .monospace()
                                                    .color(base_color),
                                            ));
                                        }
                                    });
                                } else {
                                    // Simple rendering for plain lines (with word wrap)
                                    ui.add(egui::Label::new(
                                        egui::RichText::new(&line.text)
                                            .monospace()
                                            .color(base_color),
                                    ).wrap_mode(egui::TextWrapMode::Wrap));
                                }
                            }
                        });
                }

                // Handle block copy request
                if let Some((_, content)) = block_to_copy {
                    if let Some(ref mut clipboard) = self.clipboard {
                        if clipboard.set_text(content.clone()).is_ok() {
                            let preview = if content.len() > 30 {
                                format!("{}...", &content[..30])
                            } else {
                                content.clone()
                            };
                            self.clipboard_feedback = Some((
                                format!("📦 Block copied~ ♪(´ε` ) {}", preview.replace('\n', " ")),
                                std::time::Instant::now(),
                            ));
                        }
                    }
                }

                // Hints overlay (when hints mode is active) - floating window
                let mut hint_to_copy: Option<String> = None;
                let mut hint_to_open: Option<String> = None;
                let hints_active = self.tabs[self.active_tab]
                    .panes
                    .get(&focused_pane_id)
                    .map(|p| p.hints_mode.active)
                    .unwrap_or(false);

                if hints_active {
                    let hints_bg = self.theme.background_secondary;
                    let hints_accent = self.theme.accent;
                    let hints_fg = self.theme.foreground;
                    let hints_label_bg = self.theme.warning_color;
                    let hints_comment = self.theme.comment_color;
                    let success_color = self.theme.success_color;

                    // Get hints data before the Area closure
                    let (hints_list, filter_text, selected_idx, scroll_to_selected): (Vec<_>, String, Option<usize>, bool) = {
                        if let Some(pane) = self.tabs[self.active_tab].panes.get_mut(&focused_pane_id) {
                            let scroll = pane.hints_scroll_to_selected;
                            pane.hints_scroll_to_selected = false; // Reset after reading
                            (pane.hints_mode.hints.clone(), pane.hints_mode.filter.clone(), pane.hints_mode.selected, scroll)
                        } else {
                            (vec![], String::new(), None, false)
                        }
                    };

                    egui::Area::new(egui::Id::new("hints_overlay"))
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .order(egui::Order::Foreground)
                        .show(ui.ctx(), |ui| {
                            egui::Frame::default()
                                .fill(hints_bg)
                                .stroke(egui::Stroke::new(2.0, hints_accent))
                                .corner_radius(egui::CornerRadius::same(8))
                                .inner_margin(egui::Margin::same(16))
                                .shadow(egui::epaint::Shadow {
                                    spread: 8,
                                    blur: 16,
                                    color: egui::Color32::from_black_alpha(100),
                                    offset: [0, 4].into(),
                                })
                                .show(ui, |ui| {
                                    ui.set_min_width(400.0);
                                    ui.set_max_width(600.0);

                                    // Header
                                    ui.horizontal(|ui| {
                                        ui.add(egui::Label::new(
                                            egui::RichText::new("🔗 Hints Mode")
                                                .color(hints_accent)
                                                .size(16.0)
                                                .strong(),
                                        ));
                                        ui.add(egui::Label::new(
                                            egui::RichText::new(format!(" — {} hints found", hints_list.len()))
                                                .color(hints_comment)
                                                .size(13.0),
                                        ));
                                    });

                                    ui.add_space(4.0);
                                    ui.add(egui::Label::new(
                                        egui::RichText::new("Type label | Tab to cycle | Enter to select | Esc to exit")
                                            .color(hints_comment)
                                            .size(11.0),
                                    ));

                                    // Show current filter
                                    if !filter_text.is_empty() {
                                        ui.add_space(4.0);
                                        ui.add(egui::Label::new(
                                            egui::RichText::new(format!("Filter: {}", filter_text))
                                                .color(hints_accent)
                                                .size(12.0),
                                        ));
                                    }

                                    ui.add_space(8.0);
                                    ui.separator();
                                    ui.add_space(8.0);

                                    // Show message if no hints found
                                    if hints_list.is_empty() {
                                        ui.add(egui::Label::new(
                                            egui::RichText::new("No hints found in terminal output.")
                                                .color(hints_comment)
                                                .size(13.0),
                                        ));
                                        ui.add_space(8.0);
                                        ui.add(egui::Label::new(
                                            egui::RichText::new("Run commands that output URLs, file paths, or git hashes.")
                                                .color(hints_comment)
                                                .size(11.0),
                                        ));
                                    } else {
                                        // Scrollable hints list
                                        egui::ScrollArea::vertical()
                                            .max_height(300.0)
                                            .show(ui, |ui| {
                                                let filtered: Vec<_> = hints_list.iter()
                                                    .enumerate()
                                                    .filter(|(_, h)| {
                                                        filter_text.is_empty() ||
                                                        h.label.starts_with(&filter_text)
                                                    })
                                                    .collect();

                                                for (orig_idx, hint) in &filtered {
                                                    // Check if this hint is selected (by Tab navigation or single match)
                                                    let is_selected = selected_idx == Some(*orig_idx) || filtered.len() == 1;

                                                    let row_response = ui.horizontal(|ui| {
                                                        // Selection indicator
                                                        if is_selected {
                                                            ui.add(egui::Label::new(
                                                                egui::RichText::new("▶")
                                                                    .color(success_color)
                                                                    .size(12.0),
                                                            ));
                                                        } else {
                                                            ui.add_space(14.0);
                                                        }

                                                        // Label badge
                                                        let label_color = if is_selected {
                                                            success_color
                                                        } else {
                                                            hints_label_bg
                                                        };

                                                        egui::Frame::default()
                                                            .fill(label_color)
                                                            .corner_radius(egui::CornerRadius::same(3))
                                                            .inner_margin(egui::Margin::symmetric(6, 2))
                                                            .show(ui, |ui| {
                                                                ui.add(egui::Label::new(
                                                                    egui::RichText::new(&hint.label)
                                                                        .color(egui::Color32::BLACK)
                                                                        .size(12.0)
                                                                        .strong()
                                                                        .monospace(),
                                                                ));
                                                            });

                                                        ui.add_space(4.0);

                                                        // Type icon
                                                        ui.add(egui::Label::new(
                                                            egui::RichText::new(hint.hint_type.icon())
                                                                .size(14.0),
                                                        ));

                                                        ui.add_space(4.0);

                                                        // Hint text (truncated if too long)
                                                        let display_text = if hint.text.len() > 50 {
                                                            format!("{}...", &hint.text[..47])
                                                        } else {
                                                            hint.text.clone()
                                                        };

                                                        let hint_label = ui.add(egui::Label::new(
                                                            egui::RichText::new(&display_text)
                                                                .color(if is_selected { hints_accent } else { hints_fg })
                                                                .size(12.0)
                                                                .monospace(),
                                                        ).sense(egui::Sense::click()));

                                                        if hint_label.clicked() {
                                                            hint_to_copy = Some(hint.text.clone());
                                                        }

                                                        if hint_label.hovered() {
                                                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                                        }

                                                        // Action hint
                                                        ui.add(egui::Label::new(
                                                            egui::RichText::new(format!(" ({})", hint.hint_type.action_desc()))
                                                                .color(hints_comment)
                                                                .size(10.0),
                                                        ));
                                                    });

                                                    // Scroll to selected hint when Tab cycles
                                                    if is_selected && scroll_to_selected {
                                                        row_response.response.scroll_to_me(Some(egui::Align::Center));
                                                    }

                                                    ui.add_space(2.0);
                                                }

                                                // Auto-select if only one match
                                                if filtered.len() == 1 && !filter_text.is_empty() {
                                                    let (_, hint) = &filtered[0];
                                                    if hint.label == filter_text {
                                                        match hint.hint_type {
                                                            HintType::Url => {
                                                                hint_to_open = Some(hint.text.clone());
                                                            }
                                                            _ => {
                                                                hint_to_copy = Some(hint.text.clone());
                                                            }
                                                        }
                                                    }
                                                }
                                            });
                                    }
                                });
                        });
                }

                // Handle hint actions
                if let Some(text) = hint_to_copy {
                    if let Some(ref mut clipboard) = self.clipboard {
                        if clipboard.set_text(text.clone()).is_ok() {
                            let preview = if text.len() > 40 {
                                format!("{}...", &text[..37])
                            } else {
                                text.clone()
                            };
                            self.clipboard_feedback = Some((
                                format!("🔗 Copied~ {}", preview),
                                std::time::Instant::now(),
                            ));
                        }
                    }
                    // Deactivate hints mode after action
                    if let Some(pane) = self.tabs[self.active_tab].panes.get_mut(&focused_pane_id) {
                        pane.hints_mode.deactivate();
                    }
                }
                if let Some(url) = hint_to_open {
                    let _ = open::that(&url);
                    self.clipboard_feedback = Some((
                        format!("🌐 Opening~ {}", if url.len() > 40 { format!("{}...", &url[..37]) } else { url }),
                        std::time::Instant::now(),
                    ));
                    // Deactivate hints mode after action
                    if let Some(pane) = self.tabs[self.active_tab].panes.get_mut(&focused_pane_id) {
                        pane.hints_mode.deactivate();
                    }
                }

                // Vi mode overlay (floating help panel)
                let vi_active = self.tabs[self.active_tab]
                    .panes
                    .get(&focused_pane_id)
                    .map(|p| p.vi_mode.active)
                    .unwrap_or(false);

                if vi_active {
                    let vi_bg = self.theme.background_secondary;
                    let vi_accent = self.theme.accent;
                    let vi_fg = self.theme.foreground;
                    let vi_comment = self.theme.comment_color;
                    let vi_success = self.theme.success_color;

                    // Get vi mode state
                    let (vi_state, vi_search, vi_status) = {
                        if let Some(pane) = self.tabs[self.active_tab].panes.get(&focused_pane_id) {
                            (pane.vi_mode.state.clone(), pane.vi_mode.search_query.clone(), pane.vi_mode.status_text())
                        } else {
                            (ViState::Normal, String::new(), String::new())
                        }
                    };

                    egui::Area::new(egui::Id::new("vi_mode_help"))
                        .anchor(egui::Align2::RIGHT_BOTTOM, [-20.0, -60.0])
                        .order(egui::Order::Foreground)
                        .show(ui.ctx(), |ui| {
                            egui::Frame::default()
                                .fill(vi_bg)
                                .stroke(egui::Stroke::new(1.0, vi_accent))
                                .corner_radius(egui::CornerRadius::same(6))
                                .inner_margin(egui::Margin::same(12))
                                .shadow(egui::epaint::Shadow { spread: 4, blur: 8, color: egui::Color32::from_black_alpha(80), offset: [0, 2].into() })
                                .show(ui, |ui| {
                                    ui.set_min_width(200.0);
                                    ui.horizontal(|ui| {
                                        ui.add(egui::Label::new(egui::RichText::new("📜 Vi Mode").color(vi_accent).size(14.0).strong()));
                                    });
                                    ui.add(egui::Label::new(egui::RichText::new(&vi_status).color(vi_success).size(11.0).monospace()));
                                    ui.add_space(6.0);
                                    ui.separator();
                                    ui.add_space(6.0);
                                    ui.add(egui::Label::new(egui::RichText::new("Navigation:").color(vi_fg).size(11.0).strong()));
                                    for (keys, desc) in [("h j k l", "← ↓ ↑ →"), ("gg / G", "top / bottom"), ("0 / $", "line start / end"), ("w / b", "word fwd / back"), ("Ctrl+D/U", "half page"), ("Ctrl+F/B", "full page")] {
                                        ui.horizontal(|ui| {
                                            ui.add(egui::Label::new(egui::RichText::new(keys).color(vi_accent).size(10.0).monospace()));
                                            ui.add(egui::Label::new(egui::RichText::new(desc).color(vi_comment).size(10.0)));
                                        });
                                    }
                                    ui.add_space(4.0);
                                    ui.add(egui::Label::new(egui::RichText::new("Actions:").color(vi_fg).size(11.0).strong()));
                                    for (keys, desc) in [("v / V", "visual / line"), ("y / Y", "yank / yank line"), ("/ or ?", "search ↓/↑"), ("n / N", "next / prev"), ("q / Esc", "exit")] {
                                        ui.horizontal(|ui| {
                                            ui.add(egui::Label::new(egui::RichText::new(keys).color(vi_accent).size(10.0).monospace()));
                                            ui.add(egui::Label::new(egui::RichText::new(desc).color(vi_comment).size(10.0)));
                                        });
                                    }
                                    if vi_state == ViState::SearchForward || vi_state == ViState::SearchBackward {
                                        ui.add_space(4.0);
                                        ui.separator();
                                        let prefix = if vi_state == ViState::SearchForward { "/" } else { "?" };
                                        ui.add(egui::Label::new(egui::RichText::new(format!("{}{}", prefix, vi_search)).color(vi_accent).size(12.0).monospace()));
                                    }
                                });
                        });
                }

                ui.add_space(4.0);

                // Input line with prompt
                if let Some(pane) = self.tabs[self.active_tab].panes.get_mut(&focused_pane_id) {
                    let prompt = pane.state.format_prompt();

                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new(&prompt)
                                .monospace()
                                .color(path_color),
                        ));

                        // Input field (leave space for mascot)
                        let input_width = ui.available_width() - 100.0;
                        let move_cursor_to_end = pane.cursor_to_end;
                        if pane.cursor_to_end {
                            pane.cursor_to_end = false;
                        }

                        let text_edit_id = ui.make_persistent_id("input_field");
                        let modal_active = pane.hints_mode.active || pane.vi_mode.active || palette_was_open || editor_is_open;
                        let text_edit = egui::TextEdit::singleline(&mut pane.input)
                            .id(text_edit_id)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(input_width.max(200.0))
                            .frame(false)
                            .interactive(!modal_active);  // Disable input when modal/overlay is active

                        let response = ui.add(text_edit);

                        // Move cursor to end after autocomplete
                        if move_cursor_to_end {
                            if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                                let ccursor = egui::text::CCursor::new(pane.input.len());
                                state.cursor.set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                                state.store(ui.ctx(), text_edit_id);
                            }
                        }

                        // Track if input changed for autocomplete
                        if response.changed() {
                            input_changed = true;
                        }

                        // Handle keyboard input (only when no overlays are active)
                        // Use palette_was_open to prevent Enter from executing after palette closes
                        let overlay_active = editor_is_open || palette_was_open || pane.fuzzy_finder.active;

                        // Auto-focus the input and execute on Enter (only if no overlay)
                        if !overlay_active && response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            command_to_execute = Some(std::mem::take(&mut pane.input));
                            response.request_focus();
                        }
                        if response.has_focus() && !overlay_active {
                            let has_suggestions = !pane.suggestions.is_empty();

                            // Tab: apply suggestion (consume to prevent focus change)
                            if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab)) {
                                if has_suggestions {
                                    apply_suggestion = true;
                                }
                            }

                            // Up/Down: navigate suggestions or history
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                                if has_suggestions && pane.show_suggestions {
                                    suggestion_up = true;
                                } else {
                                    history_up = true;
                                }
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                                if has_suggestions && pane.show_suggestions {
                                    suggestion_down = true;
                                } else {
                                    history_down = true;
                                }
                            }

                            // Escape: close suggestions
                            if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                                if pane.show_suggestions {
                                    close_suggestions = true;
                                }
                            }
                        }

                        // Keep focus on input
                        response.request_focus();
                    });
                }
            });

        // Handle history navigation outside the closure
        if history_up {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                // Save current input when starting to navigate history
                if pane.saved_input.is_empty() && !pane.input.is_empty() {
                    pane.saved_input = pane.input.clone();
                }
                if let Some(cmd) = pane.history.previous() {
                    pane.input = cmd.to_string();
                }
            }
        }
        if history_down {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if let Some(cmd) = pane.history.next() {
                    pane.input = cmd.to_string();
                } else {
                    // Restore saved input when reaching the end of history
                    pane.input = std::mem::take(&mut pane.saved_input);
                }
            }
        }

        // Handle autocomplete
        if input_changed {
            self.update_suggestions();
        }

        // Navigate suggestions
        if suggestion_up {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if !pane.suggestions.is_empty() {
                    if pane.selected_suggestion == 0 {
                        pane.selected_suggestion = pane.suggestions.len() - 1;
                    } else {
                        pane.selected_suggestion -= 1;
                    }
                }
            }
        }
        if suggestion_down {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                if !pane.suggestions.is_empty() {
                    pane.selected_suggestion = (pane.selected_suggestion + 1) % pane.suggestions.len();
                }
            }
        }

        // Apply selected suggestion
        if apply_suggestion {
            self.apply_suggestion();
        }

        // Close suggestions
        if close_suggestions {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.show_suggestions = false;
            }
        }

        // Execute command outside the closure
        if let Some(command) = command_to_execute {
            // Clear suggestions before executing
            if let Some(pane) = self.tabs[self.active_tab].focused_pane_mut() {
                pane.suggestions.clear();
                pane.show_suggestions = false;
            }
            self.execute_command(&command);
        }

        // Render autocomplete popup
        let (show_suggestions, suggestions, selected) = {
            if let Some(pane) = self.tabs[self.active_tab].focused_pane() {
                (pane.show_suggestions, pane.suggestions.clone(), pane.selected_suggestion)
            } else {
                (false, Vec::new(), 0)
            }
        };

        if show_suggestions {

            egui::Area::new(egui::Id::new("autocomplete_popup"))
                .order(egui::Order::Foreground)
                .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(100.0, -35.0))
                .show(ctx, |ui| {
                    egui::Frame::new()
                        .fill(self.theme.background_secondary)
                        .stroke(egui::Stroke::new(1.0, self.theme.accent))
                        .corner_radius(egui::CornerRadius::same(4))
                        .inner_margin(egui::Margin::same(4))
                        .show(ui, |ui| {
                            ui.set_max_width(400.0);

                            for (i, suggestion) in suggestions.iter().take(8).enumerate() {
                                let is_selected = i == selected;

                                let bg = if is_selected {
                                    self.theme.selection
                                } else {
                                    egui::Color32::TRANSPARENT
                                };

                                let text_color = if is_selected {
                                    self.theme.foreground
                                } else {
                                    self.theme.comment_color
                                };

                                // Get icon and color based on suggestion kind
                                let (icon, icon_color) = match suggestion.kind {
                                    SuggestionKind::History => ("", self.theme.info_color),
                                    SuggestionKind::File => ("", self.theme.foreground),
                                    SuggestionKind::Directory => ("", self.theme.folder_color),
                                    SuggestionKind::GitBranch => ("", self.theme.branch_color),
                                    SuggestionKind::Flag => ("", self.theme.flag_color),
                                    SuggestionKind::Command => ("", self.theme.success_color),
                                    SuggestionKind::EnvVar => ("$", self.theme.number_color),
                                };

                                egui::Frame::new()
                                    .fill(bg)
                                    .corner_radius(egui::CornerRadius::same(2))
                                    .inner_margin(egui::Margin::symmetric(6, 2))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            // Icon
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(icon)
                                                    .color(icon_color)
                                                    .size(12.0),
                                            ));

                                            // Suggestion text
                                            ui.add(egui::Label::new(
                                                egui::RichText::new(&suggestion.text)
                                                    .color(text_color)
                                                    .monospace()
                                                    .size(13.0),
                                            ));

                                            // Description if available
                                            if let Some(desc) = &suggestion.description {
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(format!(" - {}", desc))
                                                        .color(self.theme.comment_color)
                                                        .size(11.0),
                                                ));
                                            }
                                        });
                                    });
                            }

                            // Show hint at bottom
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(
                                    egui::RichText::new("Tab: apply | ↑↓: navigate | Esc: close")
                                        .color(self.theme.comment_color)
                                        .size(10.0),
                                ));
                            });
                        });
                });
        }

        // Increment frame counter and autosave periodically (every ~5 seconds at 60fps = 300 frames)
        self.frame_count += 1;
        if self.frame_count % 300 == 0 {
            self.autosave();
        }

        // Request repaint for smooth mascot animations
        ctx.request_repaint();
    }
}

impl Drop for ZaxiomApp {
    fn drop(&mut self) {
        // Save session on exit
        self.autosave();
    }
}
