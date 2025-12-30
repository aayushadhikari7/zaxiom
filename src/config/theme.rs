//! Theme configuration
//!
//! Colors and visual styling for the terminal.
//! Default theme: Catppuccin Mocha - a trendy modern pastel dark theme.

#![allow(dead_code)]

use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

/// Nerd Font icons for terminal indicators
pub mod icons {
    // Folder icons
    pub const FOLDER: &str = ""; // nf-fa-folder
    pub const FOLDER_OPEN: &str = ""; // nf-fa-folder_open
    pub const HOME: &str = ""; // nf-fa-home
    pub const ROOT: &str = ""; // nf-md-folder_cog

    // Git icons
    pub const GIT_BRANCH: &str = ""; // nf-fa-code_fork
    pub const GIT_COMMIT: &str = ""; // nf-oct-git_commit
    pub const GIT_MODIFIED: &str = ""; // nf-fa-circle (modified)
    pub const GIT_STAGED: &str = ""; // nf-fa-check
    pub const GIT_UNTRACKED: &str = ""; // nf-fa-question
    pub const GIT_CLEAN: &str = ""; // nf-fa-check_circle
    pub const GIT_DIRTY: &str = ""; // nf-fa-exclamation_circle

    // File type icons
    pub const FILE: &str = ""; // nf-fa-file
    pub const FILE_CODE: &str = ""; // nf-fa-file_code_o
    pub const FILE_TEXT: &str = ""; // nf-fa-file_text
    pub const FILE_IMAGE: &str = ""; // nf-fa-file_image_o
    pub const FILE_ZIP: &str = ""; // nf-fa-file_archive_o
    pub const SYMLINK: &str = ""; // nf-fa-external_link

    // Status icons
    pub const SUCCESS: &str = ""; // nf-fa-check
    pub const ERROR: &str = ""; // nf-fa-times
    pub const WARNING: &str = ""; // nf-fa-exclamation_triangle
    pub const INFO: &str = ""; // nf-fa-info_circle
    pub const ARROW_RIGHT: &str = ""; // nf-fa-arrow_right
    pub const CHEVRON: &str = ""; // nf-fa-chevron_right
    pub const PROMPT: &str = "❯"; // Unicode prompt (works everywhere)

    // System icons
    pub const WINDOWS: &str = ""; // nf-fa-windows
    pub const LINUX: &str = ""; // nf-fa-linux
    pub const APPLE: &str = ""; // nf-fa-apple
    pub const TERMINAL: &str = ""; // nf-oct-terminal
}

/// Theme configuration
#[derive(Clone, Debug)]
pub struct Theme {
    // Font settings
    /// Base font size in points
    pub font_size: f32,
    /// Line height multiplier
    pub line_height: f32,

    /// Background color (base)
    pub background: Color32,
    /// Slightly darker background for panels/sidebars (mantle)
    pub background_secondary: Color32,
    /// Darkest background for borders/separators (crust)
    pub background_tertiary: Color32,
    /// Foreground (text) color
    pub foreground: Color32,
    /// Dimmed text color (subtext)
    pub foreground_dim: Color32,
    /// Cursor color
    pub cursor: Color32,
    /// Selection color (surface)
    pub selection: Color32,
    /// Path color (in prompt) - teal
    pub path_color: Color32,
    /// Git branch color - peach
    pub branch_color: Color32,
    /// Command color (built-in commands) - mauve/purple
    pub command_color: Color32,
    /// String color - green
    pub string_color: Color32,
    /// Number/constant color - peach
    pub number_color: Color32,
    /// Flag/argument color - sky
    pub flag_color: Color32,
    /// Error color - red
    pub error_color: Color32,
    /// Warning color - yellow
    pub warning_color: Color32,
    /// Success color - green
    pub success_color: Color32,
    /// Info color - blue
    pub info_color: Color32,
    /// Accent color - mauve (primary accent)
    pub accent: Color32,
    /// Secondary accent - pink
    pub accent_secondary: Color32,
    /// Link/URL color - sapphire
    pub link_color: Color32,
    /// Comment/muted text - overlay
    pub comment_color: Color32,

    // Icon/Indicator colors (iTerm/Warp style)
    /// Folder icon color - yellow/gold
    pub folder_color: Color32,
    /// Git branch icon color - peach
    pub git_icon_color: Color32,
    /// Clean git status - green
    pub git_clean_color: Color32,
    /// Dirty git status - yellow
    pub git_dirty_color: Color32,
    /// Prompt arrow/chevron color - mauve
    pub prompt_color: Color32,
    /// Root/system folder color - red
    pub root_color: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        // Catppuccin Mocha - Modern pastel dark theme
        // https://catppuccin.com/palette/
        Self {
            // Font settings - bigger for better readability
            font_size: 16.0,  // Larger than typical 12-14pt
            line_height: 1.4, // Comfortable line spacing

            // Base colors
            background: Color32::from_rgb(0x1e, 0x1e, 0x2e), // base
            background_secondary: Color32::from_rgb(0x18, 0x18, 0x25), // mantle
            background_tertiary: Color32::from_rgb(0x11, 0x11, 0x1b), // crust
            foreground: Color32::from_rgb(0xcd, 0xd6, 0xf4), // text
            foreground_dim: Color32::from_rgb(0xa6, 0xad, 0xc8), // subtext0

            // UI elements
            cursor: Color32::from_rgb(0xf5, 0xe0, 0xdc), // rosewater
            selection: Color32::from_rgb(0x45, 0x47, 0x5a), // surface1

            // Syntax colors
            path_color: Color32::from_rgb(0x94, 0xe2, 0xd5), // teal
            branch_color: Color32::from_rgb(0xfa, 0xb3, 0x87), // peach
            command_color: Color32::from_rgb(0xcb, 0xa6, 0xf7), // mauve
            string_color: Color32::from_rgb(0xa6, 0xe3, 0xa1), // green
            number_color: Color32::from_rgb(0xfa, 0xb3, 0x87), // peach
            flag_color: Color32::from_rgb(0x89, 0xdc, 0xeb), // sky

            // Status colors
            error_color: Color32::from_rgb(0xf3, 0x8b, 0xa8), // red
            warning_color: Color32::from_rgb(0xf9, 0xe2, 0xaf), // yellow
            success_color: Color32::from_rgb(0xa6, 0xe3, 0xa1), // green
            info_color: Color32::from_rgb(0x89, 0xb4, 0xfa),  // blue

            // Accents
            accent: Color32::from_rgb(0xcb, 0xa6, 0xf7), // mauve
            accent_secondary: Color32::from_rgb(0xf5, 0xc2, 0xe7), // pink
            link_color: Color32::from_rgb(0x74, 0xc7, 0xec), // sapphire
            comment_color: Color32::from_rgb(0x6c, 0x70, 0x86), // overlay0

            // Icon/Indicator colors (iTerm/Warp style)
            folder_color: Color32::from_rgb(0xf9, 0xe2, 0xaf), // yellow
            git_icon_color: Color32::from_rgb(0xfa, 0xb3, 0x87), // peach
            git_clean_color: Color32::from_rgb(0xa6, 0xe3, 0xa1), // green
            git_dirty_color: Color32::from_rgb(0xf9, 0xe2, 0xaf), // yellow
            prompt_color: Color32::from_rgb(0xcb, 0xa6, 0xf7), // mauve
            root_color: Color32::from_rgb(0xf3, 0x8b, 0xa8),   // red
        }
    }
}

/// Theme configuration from TOML
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ThemeConfig {
    pub name: Option<String>,
    pub background: Option<String>,
    pub background_secondary: Option<String>,
    pub background_tertiary: Option<String>,
    pub foreground: Option<String>,
    pub foreground_dim: Option<String>,
    pub cursor: Option<String>,
    pub selection: Option<String>,
    pub accent: Option<String>,
    pub accent_secondary: Option<String>,
    #[serde(default)]
    pub syntax: SyntaxColors,
    #[serde(default)]
    pub status: StatusColors,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SyntaxColors {
    pub command: Option<String>,
    pub path: Option<String>,
    pub string: Option<String>,
    pub number: Option<String>,
    pub flag: Option<String>,
    pub comment: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StatusColors {
    pub error: Option<String>,
    pub warning: Option<String>,
    pub success: Option<String>,
    pub info: Option<String>,
}

/// Parse a hex color string like "#ff0000"
pub fn parse_hex_color(hex: &str) -> Option<Color32> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color32::from_rgb(r, g, b))
}

/// Available built-in themes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ThemeName {
    /// Catppuccin Mocha (default) - Modern pastel dark
    #[default]
    CatppuccinMocha,
    /// Catppuccin Latte - Light version
    CatppuccinLatte,
    /// Dracula - Dark purple theme
    Dracula,
    /// Nord - Arctic, bluish colors
    Nord,
    /// Gruvbox Dark - Retro groove colors
    GruvboxDark,
    /// Gruvbox Light
    GruvboxLight,
    /// Tokyo Night - Dark theme inspired by Tokyo city lights
    TokyoNight,
    /// Tokyo Night Storm - Slightly lighter variant
    TokyoNightStorm,
    /// One Dark - Atom's iconic dark theme
    OneDark,
    /// Solarized Dark
    SolarizedDark,
    /// Solarized Light
    SolarizedLight,
    /// Monokai Pro
    MonokaiPro,
    /// Palenight - Material palenight
    Palenight,
    /// Ayu Dark
    AyuDark,
    /// Ayu Mirage
    AyuMirage,
    /// Kanagawa - Theme inspired by Katsushika Hokusai
    Kanagawa,
    /// Rosé Pine - All natural pine, faux fur and a bit of soho vibes
    RosePine,
    /// Rosé Pine Moon - Darker variant
    RosePineMoon,
    /// Everforest Dark - Green-based dark theme
    EverforestDark,
    /// Night Owl - Theme for night owls
    NightOwl,
}

impl ThemeName {
    /// Get all available theme names
    pub fn all() -> &'static [ThemeName] {
        &[
            ThemeName::CatppuccinMocha,
            ThemeName::CatppuccinLatte,
            ThemeName::Dracula,
            ThemeName::Nord,
            ThemeName::GruvboxDark,
            ThemeName::GruvboxLight,
            ThemeName::TokyoNight,
            ThemeName::TokyoNightStorm,
            ThemeName::OneDark,
            ThemeName::SolarizedDark,
            ThemeName::SolarizedLight,
            ThemeName::MonokaiPro,
            ThemeName::Palenight,
            ThemeName::AyuDark,
            ThemeName::AyuMirage,
            ThemeName::Kanagawa,
            ThemeName::RosePine,
            ThemeName::RosePineMoon,
            ThemeName::EverforestDark,
            ThemeName::NightOwl,
        ]
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeName::CatppuccinMocha => "Catppuccin Mocha",
            ThemeName::CatppuccinLatte => "Catppuccin Latte",
            ThemeName::Dracula => "Dracula",
            ThemeName::Nord => "Nord",
            ThemeName::GruvboxDark => "Gruvbox Dark",
            ThemeName::GruvboxLight => "Gruvbox Light",
            ThemeName::TokyoNight => "Tokyo Night",
            ThemeName::TokyoNightStorm => "Tokyo Night Storm",
            ThemeName::OneDark => "One Dark",
            ThemeName::SolarizedDark => "Solarized Dark",
            ThemeName::SolarizedLight => "Solarized Light",
            ThemeName::MonokaiPro => "Monokai Pro",
            ThemeName::Palenight => "Palenight",
            ThemeName::AyuDark => "Ayu Dark",
            ThemeName::AyuMirage => "Ayu Mirage",
            ThemeName::Kanagawa => "Kanagawa",
            ThemeName::RosePine => "Rosé Pine",
            ThemeName::RosePineMoon => "Rosé Pine Moon",
            ThemeName::EverforestDark => "Everforest Dark",
            ThemeName::NightOwl => "Night Owl",
        }
    }

    /// Check if this is a light theme
    pub fn is_light(&self) -> bool {
        matches!(
            self,
            ThemeName::CatppuccinLatte | ThemeName::GruvboxLight | ThemeName::SolarizedLight
        )
    }

    /// Get config key for this theme (used for saving)
    pub fn config_key(&self) -> &'static str {
        match self {
            ThemeName::CatppuccinMocha => "catppuccin-mocha",
            ThemeName::CatppuccinLatte => "catppuccin-latte",
            ThemeName::Dracula => "dracula",
            ThemeName::Nord => "nord",
            ThemeName::GruvboxDark => "gruvbox-dark",
            ThemeName::GruvboxLight => "gruvbox-light",
            ThemeName::TokyoNight => "tokyo-night",
            ThemeName::TokyoNightStorm => "tokyo-night-storm",
            ThemeName::OneDark => "one-dark",
            ThemeName::SolarizedDark => "solarized-dark",
            ThemeName::SolarizedLight => "solarized-light",
            ThemeName::MonokaiPro => "monokai-pro",
            ThemeName::Palenight => "palenight",
            ThemeName::AyuDark => "ayu-dark",
            ThemeName::AyuMirage => "ayu-mirage",
            ThemeName::Kanagawa => "kanagawa",
            ThemeName::RosePine => "rose-pine",
            ThemeName::RosePineMoon => "rose-pine-moon",
            ThemeName::EverforestDark => "everforest-dark",
            ThemeName::NightOwl => "night-owl",
        }
    }

    /// Parse theme name from string (for config loading)
    pub fn from_string(s: &str) -> Option<Self> {
        let s = s.trim().to_lowercase().replace(['-', '_', ' '], "");
        match s.as_str() {
            "catppuccin" | "catppuccinmocha" | "mocha" => Some(ThemeName::CatppuccinMocha),
            "catppuccinlatte" | "latte" => Some(ThemeName::CatppuccinLatte),
            "dracula" => Some(ThemeName::Dracula),
            "nord" => Some(ThemeName::Nord),
            "gruvbox" | "gruvboxdark" => Some(ThemeName::GruvboxDark),
            "gruvboxlight" => Some(ThemeName::GruvboxLight),
            "tokyonight" | "tokyo" => Some(ThemeName::TokyoNight),
            "tokyonightstorm" | "tokyostorm" | "storm" => Some(ThemeName::TokyoNightStorm),
            "onedark" | "one" | "atom" => Some(ThemeName::OneDark),
            "solarized" | "solarizeddark" | "solar" => Some(ThemeName::SolarizedDark),
            "solarizedlight" => Some(ThemeName::SolarizedLight),
            "monokai" | "monokaipro" => Some(ThemeName::MonokaiPro),
            "palenight" | "material" => Some(ThemeName::Palenight),
            "ayu" | "ayudark" => Some(ThemeName::AyuDark),
            "ayumirage" | "mirage" => Some(ThemeName::AyuMirage),
            "kanagawa" | "wave" => Some(ThemeName::Kanagawa),
            "rosepine" | "rose" | "pine" => Some(ThemeName::RosePine),
            "rosepinemoon" | "moon" => Some(ThemeName::RosePineMoon),
            "everforest" | "forest" => Some(ThemeName::EverforestDark),
            "nightowl" | "owl" => Some(ThemeName::NightOwl),
            _ => None,
        }
    }
}

impl Theme {
    /// Get theme by name
    pub fn from_name(name: ThemeName) -> Self {
        match name {
            ThemeName::CatppuccinMocha => Self::catppuccin_mocha(),
            ThemeName::CatppuccinLatte => Self::catppuccin_latte(),
            ThemeName::Dracula => Self::dracula(),
            ThemeName::Nord => Self::nord(),
            ThemeName::GruvboxDark => Self::gruvbox_dark(),
            ThemeName::GruvboxLight => Self::gruvbox_light(),
            ThemeName::TokyoNight => Self::tokyo_night(),
            ThemeName::TokyoNightStorm => Self::tokyo_night_storm(),
            ThemeName::OneDark => Self::one_dark(),
            ThemeName::SolarizedDark => Self::solarized_dark(),
            ThemeName::SolarizedLight => Self::solarized_light(),
            ThemeName::MonokaiPro => Self::monokai_pro(),
            ThemeName::Palenight => Self::palenight(),
            ThemeName::AyuDark => Self::ayu_dark(),
            ThemeName::AyuMirage => Self::ayu_mirage(),
            ThemeName::Kanagawa => Self::kanagawa(),
            ThemeName::RosePine => Self::rose_pine(),
            ThemeName::RosePineMoon => Self::rose_pine_moon(),
            ThemeName::EverforestDark => Self::everforest_dark(),
            ThemeName::NightOwl => Self::night_owl(),
        }
    }

    /// Catppuccin Mocha (default)
    pub fn catppuccin_mocha() -> Self {
        Self::default()
    }

    /// Catppuccin Latte (light theme)
    pub fn catppuccin_latte() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0xef, 0xf1, 0xf5), // base
            background_secondary: Color32::from_rgb(0xe6, 0xe9, 0xef), // mantle
            background_tertiary: Color32::from_rgb(0xdc, 0xe0, 0xe8), // crust
            foreground: Color32::from_rgb(0x4c, 0x4f, 0x69), // text
            foreground_dim: Color32::from_rgb(0x6c, 0x6f, 0x85), // subtext0
            cursor: Color32::from_rgb(0xdc, 0x8a, 0x78),     // rosewater
            selection: Color32::from_rgb(0xcc, 0xd0, 0xda),  // surface1
            path_color: Color32::from_rgb(0x17, 0x93, 0x99), // teal
            branch_color: Color32::from_rgb(0xfe, 0x64, 0x0b), // peach
            command_color: Color32::from_rgb(0x88, 0x39, 0xef), // mauve
            string_color: Color32::from_rgb(0x40, 0xa0, 0x2b), // green
            number_color: Color32::from_rgb(0xfe, 0x64, 0x0b), // peach
            flag_color: Color32::from_rgb(0x04, 0xa5, 0xe5), // sky
            error_color: Color32::from_rgb(0xd2, 0x00, 0x65), // red
            warning_color: Color32::from_rgb(0xdf, 0x8e, 0x1d), // yellow
            success_color: Color32::from_rgb(0x40, 0xa0, 0x2b), // green
            info_color: Color32::from_rgb(0x1e, 0x66, 0xf5), // blue
            accent: Color32::from_rgb(0x88, 0x39, 0xef),     // mauve
            accent_secondary: Color32::from_rgb(0xea, 0x76, 0xcb), // pink
            link_color: Color32::from_rgb(0x20, 0x9f, 0xb5), // sapphire
            comment_color: Color32::from_rgb(0x8c, 0x8f, 0xa1), // overlay0
            folder_color: Color32::from_rgb(0xdf, 0x8e, 0x1d), // yellow
            git_icon_color: Color32::from_rgb(0xfe, 0x64, 0x0b), // peach
            git_clean_color: Color32::from_rgb(0x40, 0xa0, 0x2b), // green
            git_dirty_color: Color32::from_rgb(0xdf, 0x8e, 0x1d), // yellow
            prompt_color: Color32::from_rgb(0x88, 0x39, 0xef), // mauve
            root_color: Color32::from_rgb(0xd2, 0x00, 0x65), // red
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x28, 0x2a, 0x36),
            background_secondary: Color32::from_rgb(0x21, 0x22, 0x2c),
            background_tertiary: Color32::from_rgb(0x19, 0x1a, 0x21),
            foreground: Color32::from_rgb(0xf8, 0xf8, 0xf2),
            foreground_dim: Color32::from_rgb(0x62, 0x72, 0x84),
            cursor: Color32::from_rgb(0xf8, 0xf8, 0xf2),
            selection: Color32::from_rgb(0x44, 0x47, 0x5a),
            path_color: Color32::from_rgb(0x8b, 0xe9, 0xfd), // cyan
            branch_color: Color32::from_rgb(0xff, 0xb8, 0x6c), // orange
            command_color: Color32::from_rgb(0xbd, 0x93, 0xf9), // purple
            string_color: Color32::from_rgb(0xf1, 0xfa, 0x8c), // yellow
            number_color: Color32::from_rgb(0xbd, 0x93, 0xf9), // purple
            flag_color: Color32::from_rgb(0x8b, 0xe9, 0xfd), // cyan
            error_color: Color32::from_rgb(0xff, 0x55, 0x55), // red
            warning_color: Color32::from_rgb(0xff, 0xb8, 0x6c), // orange
            success_color: Color32::from_rgb(0x50, 0xfa, 0x7b), // green
            info_color: Color32::from_rgb(0x8b, 0xe9, 0xfd), // cyan
            accent: Color32::from_rgb(0xff, 0x79, 0xc6),     // pink
            accent_secondary: Color32::from_rgb(0xbd, 0x93, 0xf9), // purple
            link_color: Color32::from_rgb(0x8b, 0xe9, 0xfd), // cyan
            comment_color: Color32::from_rgb(0x62, 0x72, 0x84),
            folder_color: Color32::from_rgb(0xf1, 0xfa, 0x8c), // yellow
            git_icon_color: Color32::from_rgb(0xff, 0xb8, 0x6c), // orange
            git_clean_color: Color32::from_rgb(0x50, 0xfa, 0x7b), // green
            git_dirty_color: Color32::from_rgb(0xf1, 0xfa, 0x8c), // yellow
            prompt_color: Color32::from_rgb(0xff, 0x79, 0xc6), // pink
            root_color: Color32::from_rgb(0xff, 0x55, 0x55),   // red
        }
    }

    /// Nord theme
    pub fn nord() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x2e, 0x34, 0x40), // nord0
            background_secondary: Color32::from_rgb(0x3b, 0x42, 0x52), // nord1
            background_tertiary: Color32::from_rgb(0x43, 0x4c, 0x5e), // nord2
            foreground: Color32::from_rgb(0xec, 0xef, 0xf4), // nord6
            foreground_dim: Color32::from_rgb(0xd8, 0xde, 0xe9), // nord4
            cursor: Color32::from_rgb(0xd8, 0xde, 0xe9),
            selection: Color32::from_rgb(0x43, 0x4c, 0x5e), // nord3
            path_color: Color32::from_rgb(0x8f, 0xbc, 0xbb), // nord7 (frost)
            branch_color: Color32::from_rgb(0xd0, 0x87, 0x70), // nord12 (aurora orange)
            command_color: Color32::from_rgb(0x81, 0xa1, 0xc1), // nord9 (frost)
            string_color: Color32::from_rgb(0xa3, 0xbe, 0x8c), // nord14 (aurora green)
            number_color: Color32::from_rgb(0xb4, 0x8e, 0xad), // nord15 (aurora purple)
            flag_color: Color32::from_rgb(0x88, 0xc0, 0xd0), // nord8 (frost)
            error_color: Color32::from_rgb(0xbf, 0x61, 0x6a), // nord11 (aurora red)
            warning_color: Color32::from_rgb(0xeb, 0xcb, 0x8b), // nord13 (aurora yellow)
            success_color: Color32::from_rgb(0xa3, 0xbe, 0x8c), // nord14
            info_color: Color32::from_rgb(0x5e, 0x81, 0xac), // nord10
            accent: Color32::from_rgb(0x88, 0xc0, 0xd0),    // nord8
            accent_secondary: Color32::from_rgb(0x81, 0xa1, 0xc1), // nord9
            link_color: Color32::from_rgb(0x5e, 0x81, 0xac), // nord10
            comment_color: Color32::from_rgb(0x4c, 0x56, 0x6a), // nord3
            folder_color: Color32::from_rgb(0xeb, 0xcb, 0x8b), // nord13
            git_icon_color: Color32::from_rgb(0xd0, 0x87, 0x70), // nord12
            git_clean_color: Color32::from_rgb(0xa3, 0xbe, 0x8c), // nord14
            git_dirty_color: Color32::from_rgb(0xeb, 0xcb, 0x8b), // nord13
            prompt_color: Color32::from_rgb(0x88, 0xc0, 0xd0), // nord8
            root_color: Color32::from_rgb(0xbf, 0x61, 0x6a), // nord11
        }
    }

    /// Gruvbox Dark theme
    pub fn gruvbox_dark() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x28, 0x28, 0x28), // bg
            background_secondary: Color32::from_rgb(0x1d, 0x20, 0x21), // bg0_h
            background_tertiary: Color32::from_rgb(0x32, 0x30, 0x2f), // bg1
            foreground: Color32::from_rgb(0xeb, 0xdb, 0xb2), // fg
            foreground_dim: Color32::from_rgb(0xbd, 0xae, 0x93), // fg3
            cursor: Color32::from_rgb(0xeb, 0xdb, 0xb2),
            selection: Color32::from_rgb(0x50, 0x49, 0x45), // bg2
            path_color: Color32::from_rgb(0x83, 0xa5, 0x98), // aqua
            branch_color: Color32::from_rgb(0xfe, 0x80, 0x19), // orange
            command_color: Color32::from_rgb(0xd3, 0x86, 0x9b), // purple
            string_color: Color32::from_rgb(0xb8, 0xbb, 0x26), // green
            number_color: Color32::from_rgb(0xd3, 0x86, 0x9b), // purple
            flag_color: Color32::from_rgb(0x83, 0xa5, 0x98), // aqua
            error_color: Color32::from_rgb(0xfb, 0x49, 0x34), // red
            warning_color: Color32::from_rgb(0xfa, 0xbd, 0x2f), // yellow
            success_color: Color32::from_rgb(0xb8, 0xbb, 0x26), // green
            info_color: Color32::from_rgb(0x83, 0xa5, 0x98), // aqua
            accent: Color32::from_rgb(0xfe, 0x80, 0x19),    // orange
            accent_secondary: Color32::from_rgb(0xd3, 0x86, 0x9b), // purple
            link_color: Color32::from_rgb(0x45, 0x85, 0x88), // blue
            comment_color: Color32::from_rgb(0x92, 0x83, 0x74), // gray
            folder_color: Color32::from_rgb(0xfa, 0xbd, 0x2f), // yellow
            git_icon_color: Color32::from_rgb(0xfe, 0x80, 0x19), // orange
            git_clean_color: Color32::from_rgb(0xb8, 0xbb, 0x26), // green
            git_dirty_color: Color32::from_rgb(0xfa, 0xbd, 0x2f), // yellow
            prompt_color: Color32::from_rgb(0xfe, 0x80, 0x19), // orange
            root_color: Color32::from_rgb(0xfb, 0x49, 0x34), // red
        }
    }

    /// Gruvbox Light theme
    pub fn gruvbox_light() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0xfb, 0xf1, 0xc7), // bg
            background_secondary: Color32::from_rgb(0xf9, 0xf5, 0xd7), // bg0_h
            background_tertiary: Color32::from_rgb(0xeb, 0xdb, 0xb2), // bg1
            foreground: Color32::from_rgb(0x3c, 0x38, 0x36), // fg
            foreground_dim: Color32::from_rgb(0x66, 0x5c, 0x54), // fg3
            cursor: Color32::from_rgb(0x3c, 0x38, 0x36),
            selection: Color32::from_rgb(0xd5, 0xc4, 0xa1), // bg2
            path_color: Color32::from_rgb(0x42, 0x7b, 0x58), // aqua
            branch_color: Color32::from_rgb(0xaf, 0x3a, 0x03), // orange
            command_color: Color32::from_rgb(0x8f, 0x3f, 0x71), // purple
            string_color: Color32::from_rgb(0x79, 0x74, 0x0e), // green
            number_color: Color32::from_rgb(0x8f, 0x3f, 0x71), // purple
            flag_color: Color32::from_rgb(0x42, 0x7b, 0x58), // aqua
            error_color: Color32::from_rgb(0xcc, 0x24, 0x1d), // red
            warning_color: Color32::from_rgb(0xb5, 0x76, 0x14), // yellow
            success_color: Color32::from_rgb(0x79, 0x74, 0x0e), // green
            info_color: Color32::from_rgb(0x42, 0x7b, 0x58), // aqua
            accent: Color32::from_rgb(0xaf, 0x3a, 0x03),    // orange
            accent_secondary: Color32::from_rgb(0x8f, 0x3f, 0x71), // purple
            link_color: Color32::from_rgb(0x07, 0x66, 0x78), // blue
            comment_color: Color32::from_rgb(0x7c, 0x6f, 0x64), // gray
            folder_color: Color32::from_rgb(0xb5, 0x76, 0x14), // yellow
            git_icon_color: Color32::from_rgb(0xaf, 0x3a, 0x03), // orange
            git_clean_color: Color32::from_rgb(0x79, 0x74, 0x0e), // green
            git_dirty_color: Color32::from_rgb(0xb5, 0x76, 0x14), // yellow
            prompt_color: Color32::from_rgb(0xaf, 0x3a, 0x03), // orange
            root_color: Color32::from_rgb(0xcc, 0x24, 0x1d), // red
        }
    }

    /// Tokyo Night theme
    pub fn tokyo_night() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x1a, 0x1b, 0x26),
            background_secondary: Color32::from_rgb(0x16, 0x16, 0x1e),
            background_tertiary: Color32::from_rgb(0x12, 0x12, 0x1a),
            foreground: Color32::from_rgb(0xa9, 0xb1, 0xd6),
            foreground_dim: Color32::from_rgb(0x56, 0x5f, 0x89),
            cursor: Color32::from_rgb(0xc0, 0xca, 0xf5),
            selection: Color32::from_rgb(0x28, 0x2d, 0x3f),
            path_color: Color32::from_rgb(0x7d, 0xcf, 0xff), // cyan
            branch_color: Color32::from_rgb(0xff, 0x9e, 0x64), // orange
            command_color: Color32::from_rgb(0xbb, 0x9a, 0xf7), // purple
            string_color: Color32::from_rgb(0x9e, 0xce, 0x6a), // green
            number_color: Color32::from_rgb(0xff, 0x9e, 0x64), // orange
            flag_color: Color32::from_rgb(0x7d, 0xcf, 0xff), // cyan
            error_color: Color32::from_rgb(0xf7, 0x76, 0x8e), // red
            warning_color: Color32::from_rgb(0xe0, 0xaf, 0x68), // yellow
            success_color: Color32::from_rgb(0x9e, 0xce, 0x6a), // green
            info_color: Color32::from_rgb(0x7a, 0xa2, 0xf7), // blue
            accent: Color32::from_rgb(0x7a, 0xa2, 0xf7),     // blue
            accent_secondary: Color32::from_rgb(0xbb, 0x9a, 0xf7), // purple
            link_color: Color32::from_rgb(0x7d, 0xcf, 0xff), // cyan
            comment_color: Color32::from_rgb(0x56, 0x5f, 0x89),
            folder_color: Color32::from_rgb(0xe0, 0xaf, 0x68), // yellow
            git_icon_color: Color32::from_rgb(0xff, 0x9e, 0x64), // orange
            git_clean_color: Color32::from_rgb(0x9e, 0xce, 0x6a), // green
            git_dirty_color: Color32::from_rgb(0xe0, 0xaf, 0x68), // yellow
            prompt_color: Color32::from_rgb(0x7a, 0xa2, 0xf7), // blue
            root_color: Color32::from_rgb(0xf7, 0x76, 0x8e),   // red
        }
    }

    /// Tokyo Night Storm theme
    pub fn tokyo_night_storm() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x24, 0x28, 0x3b),
            background_secondary: Color32::from_rgb(0x1f, 0x23, 0x35),
            background_tertiary: Color32::from_rgb(0x1a, 0x1e, 0x30),
            foreground: Color32::from_rgb(0xc0, 0xca, 0xf5),
            foreground_dim: Color32::from_rgb(0x56, 0x5f, 0x89),
            cursor: Color32::from_rgb(0xc0, 0xca, 0xf5),
            selection: Color32::from_rgb(0x2a, 0x2f, 0x45),
            path_color: Color32::from_rgb(0x7d, 0xcf, 0xff),
            branch_color: Color32::from_rgb(0xff, 0x9e, 0x64),
            command_color: Color32::from_rgb(0xbb, 0x9a, 0xf7),
            string_color: Color32::from_rgb(0x9e, 0xce, 0x6a),
            number_color: Color32::from_rgb(0xff, 0x9e, 0x64),
            flag_color: Color32::from_rgb(0x7d, 0xcf, 0xff),
            error_color: Color32::from_rgb(0xf7, 0x76, 0x8e),
            warning_color: Color32::from_rgb(0xe0, 0xaf, 0x68),
            success_color: Color32::from_rgb(0x9e, 0xce, 0x6a),
            info_color: Color32::from_rgb(0x7a, 0xa2, 0xf7),
            accent: Color32::from_rgb(0x7a, 0xa2, 0xf7),
            accent_secondary: Color32::from_rgb(0xbb, 0x9a, 0xf7),
            link_color: Color32::from_rgb(0x7d, 0xcf, 0xff),
            comment_color: Color32::from_rgb(0x56, 0x5f, 0x89),
            folder_color: Color32::from_rgb(0xe0, 0xaf, 0x68),
            git_icon_color: Color32::from_rgb(0xff, 0x9e, 0x64),
            git_clean_color: Color32::from_rgb(0x9e, 0xce, 0x6a),
            git_dirty_color: Color32::from_rgb(0xe0, 0xaf, 0x68),
            prompt_color: Color32::from_rgb(0x7a, 0xa2, 0xf7),
            root_color: Color32::from_rgb(0xf7, 0x76, 0x8e),
        }
    }

    /// One Dark theme (Atom)
    pub fn one_dark() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x28, 0x2c, 0x34),
            background_secondary: Color32::from_rgb(0x21, 0x25, 0x2b),
            background_tertiary: Color32::from_rgb(0x1e, 0x22, 0x27),
            foreground: Color32::from_rgb(0xab, 0xb2, 0xbf),
            foreground_dim: Color32::from_rgb(0x5c, 0x63, 0x70),
            cursor: Color32::from_rgb(0x52, 0x8b, 0xff),
            selection: Color32::from_rgb(0x3e, 0x44, 0x51),
            path_color: Color32::from_rgb(0x56, 0xb6, 0xc2), // cyan
            branch_color: Color32::from_rgb(0xd1, 0x9a, 0x66), // orange
            command_color: Color32::from_rgb(0xc6, 0x78, 0xdd), // purple
            string_color: Color32::from_rgb(0x98, 0xc3, 0x79), // green
            number_color: Color32::from_rgb(0xd1, 0x9a, 0x66), // orange
            flag_color: Color32::from_rgb(0x56, 0xb6, 0xc2), // cyan
            error_color: Color32::from_rgb(0xe0, 0x6c, 0x75), // red
            warning_color: Color32::from_rgb(0xe5, 0xc0, 0x7b), // yellow
            success_color: Color32::from_rgb(0x98, 0xc3, 0x79), // green
            info_color: Color32::from_rgb(0x61, 0xaf, 0xef), // blue
            accent: Color32::from_rgb(0x61, 0xaf, 0xef),     // blue
            accent_secondary: Color32::from_rgb(0xc6, 0x78, 0xdd), // purple
            link_color: Color32::from_rgb(0x56, 0xb6, 0xc2), // cyan
            comment_color: Color32::from_rgb(0x5c, 0x63, 0x70),
            folder_color: Color32::from_rgb(0xe5, 0xc0, 0x7b), // yellow
            git_icon_color: Color32::from_rgb(0xd1, 0x9a, 0x66), // orange
            git_clean_color: Color32::from_rgb(0x98, 0xc3, 0x79), // green
            git_dirty_color: Color32::from_rgb(0xe5, 0xc0, 0x7b), // yellow
            prompt_color: Color32::from_rgb(0x61, 0xaf, 0xef), // blue
            root_color: Color32::from_rgb(0xe0, 0x6c, 0x75),   // red
        }
    }

    /// Solarized Dark theme
    pub fn solarized_dark() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x00, 0x2b, 0x36), // base03
            background_secondary: Color32::from_rgb(0x07, 0x36, 0x42), // base02
            background_tertiary: Color32::from_rgb(0x00, 0x24, 0x2e),
            foreground: Color32::from_rgb(0x83, 0x94, 0x96), // base0
            foreground_dim: Color32::from_rgb(0x58, 0x6e, 0x75), // base01
            cursor: Color32::from_rgb(0x83, 0x94, 0x96),
            selection: Color32::from_rgb(0x07, 0x36, 0x42), // base02
            path_color: Color32::from_rgb(0x2a, 0xa1, 0x98), // cyan
            branch_color: Color32::from_rgb(0xcb, 0x4b, 0x16), // orange
            command_color: Color32::from_rgb(0x6c, 0x71, 0xc4), // violet
            string_color: Color32::from_rgb(0x85, 0x99, 0x00), // green
            number_color: Color32::from_rgb(0xd3, 0x36, 0x82), // magenta
            flag_color: Color32::from_rgb(0x2a, 0xa1, 0x98), // cyan
            error_color: Color32::from_rgb(0xdc, 0x32, 0x2f), // red
            warning_color: Color32::from_rgb(0xb5, 0x89, 0x00), // yellow
            success_color: Color32::from_rgb(0x85, 0x99, 0x00), // green
            info_color: Color32::from_rgb(0x26, 0x8b, 0xd2), // blue
            accent: Color32::from_rgb(0x26, 0x8b, 0xd2),    // blue
            accent_secondary: Color32::from_rgb(0x6c, 0x71, 0xc4), // violet
            link_color: Color32::from_rgb(0x2a, 0xa1, 0x98), // cyan
            comment_color: Color32::from_rgb(0x58, 0x6e, 0x75), // base01
            folder_color: Color32::from_rgb(0xb5, 0x89, 0x00), // yellow
            git_icon_color: Color32::from_rgb(0xcb, 0x4b, 0x16), // orange
            git_clean_color: Color32::from_rgb(0x85, 0x99, 0x00), // green
            git_dirty_color: Color32::from_rgb(0xb5, 0x89, 0x00), // yellow
            prompt_color: Color32::from_rgb(0x26, 0x8b, 0xd2), // blue
            root_color: Color32::from_rgb(0xdc, 0x32, 0x2f), // red
        }
    }

    /// Solarized Light theme
    pub fn solarized_light() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0xfd, 0xf6, 0xe3), // base3
            background_secondary: Color32::from_rgb(0xee, 0xe8, 0xd5), // base2
            background_tertiary: Color32::from_rgb(0xfa, 0xf3, 0xdf),
            foreground: Color32::from_rgb(0x65, 0x7b, 0x83), // base00
            foreground_dim: Color32::from_rgb(0x93, 0xa1, 0xa1), // base1
            cursor: Color32::from_rgb(0x65, 0x7b, 0x83),
            selection: Color32::from_rgb(0xee, 0xe8, 0xd5), // base2
            path_color: Color32::from_rgb(0x2a, 0xa1, 0x98), // cyan
            branch_color: Color32::from_rgb(0xcb, 0x4b, 0x16), // orange
            command_color: Color32::from_rgb(0x6c, 0x71, 0xc4), // violet
            string_color: Color32::from_rgb(0x85, 0x99, 0x00), // green
            number_color: Color32::from_rgb(0xd3, 0x36, 0x82), // magenta
            flag_color: Color32::from_rgb(0x2a, 0xa1, 0x98), // cyan
            error_color: Color32::from_rgb(0xdc, 0x32, 0x2f), // red
            warning_color: Color32::from_rgb(0xb5, 0x89, 0x00), // yellow
            success_color: Color32::from_rgb(0x85, 0x99, 0x00), // green
            info_color: Color32::from_rgb(0x26, 0x8b, 0xd2), // blue
            accent: Color32::from_rgb(0x26, 0x8b, 0xd2),    // blue
            accent_secondary: Color32::from_rgb(0x6c, 0x71, 0xc4), // violet
            link_color: Color32::from_rgb(0x2a, 0xa1, 0x98), // cyan
            comment_color: Color32::from_rgb(0x93, 0xa1, 0xa1), // base1
            folder_color: Color32::from_rgb(0xb5, 0x89, 0x00), // yellow
            git_icon_color: Color32::from_rgb(0xcb, 0x4b, 0x16), // orange
            git_clean_color: Color32::from_rgb(0x85, 0x99, 0x00), // green
            git_dirty_color: Color32::from_rgb(0xb5, 0x89, 0x00), // yellow
            prompt_color: Color32::from_rgb(0x26, 0x8b, 0xd2), // blue
            root_color: Color32::from_rgb(0xdc, 0x32, 0x2f), // red
        }
    }

    /// Monokai Pro theme
    pub fn monokai_pro() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x2d, 0x2a, 0x2e),
            background_secondary: Color32::from_rgb(0x22, 0x1f, 0x22),
            background_tertiary: Color32::from_rgb(0x19, 0x18, 0x19),
            foreground: Color32::from_rgb(0xfc, 0xfc, 0xfa),
            foreground_dim: Color32::from_rgb(0x93, 0x93, 0x91),
            cursor: Color32::from_rgb(0xfc, 0xfc, 0xfa),
            selection: Color32::from_rgb(0x40, 0x3e, 0x41),
            path_color: Color32::from_rgb(0x78, 0xdc, 0xe8), // cyan
            branch_color: Color32::from_rgb(0xfc, 0x9e, 0x67), // orange
            command_color: Color32::from_rgb(0xab, 0x9d, 0xf2), // purple
            string_color: Color32::from_rgb(0xa9, 0xdc, 0x76), // green
            number_color: Color32::from_rgb(0xab, 0x9d, 0xf2), // purple
            flag_color: Color32::from_rgb(0x78, 0xdc, 0xe8), // cyan
            error_color: Color32::from_rgb(0xff, 0x61, 0x88), // red/pink
            warning_color: Color32::from_rgb(0xff, 0xd8, 0x66), // yellow
            success_color: Color32::from_rgb(0xa9, 0xdc, 0x76), // green
            info_color: Color32::from_rgb(0x78, 0xdc, 0xe8), // cyan
            accent: Color32::from_rgb(0xff, 0x61, 0x88),     // pink
            accent_secondary: Color32::from_rgb(0xab, 0x9d, 0xf2), // purple
            link_color: Color32::from_rgb(0x78, 0xdc, 0xe8), // cyan
            comment_color: Color32::from_rgb(0x72, 0x71, 0x72),
            folder_color: Color32::from_rgb(0xff, 0xd8, 0x66), // yellow
            git_icon_color: Color32::from_rgb(0xfc, 0x9e, 0x67), // orange
            git_clean_color: Color32::from_rgb(0xa9, 0xdc, 0x76), // green
            git_dirty_color: Color32::from_rgb(0xff, 0xd8, 0x66), // yellow
            prompt_color: Color32::from_rgb(0xff, 0x61, 0x88), // pink
            root_color: Color32::from_rgb(0xff, 0x61, 0x88),   // red
        }
    }

    /// Palenight theme (Material)
    pub fn palenight() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x29, 0x2d, 0x3e),
            background_secondary: Color32::from_rgb(0x24, 0x27, 0x38),
            background_tertiary: Color32::from_rgb(0x1e, 0x21, 0x30),
            foreground: Color32::from_rgb(0xa6, 0xac, 0xcd),
            foreground_dim: Color32::from_rgb(0x67, 0x6e, 0x95),
            cursor: Color32::from_rgb(0xff, 0xcc, 0x00),
            selection: Color32::from_rgb(0x3c, 0x43, 0x5e),
            path_color: Color32::from_rgb(0x89, 0xdd, 0xff), // cyan
            branch_color: Color32::from_rgb(0xf7, 0x8c, 0x6c), // orange
            command_color: Color32::from_rgb(0xc7, 0x92, 0xea), // purple
            string_color: Color32::from_rgb(0xc3, 0xe8, 0x8d), // green
            number_color: Color32::from_rgb(0xf7, 0x8c, 0x6c), // orange
            flag_color: Color32::from_rgb(0x89, 0xdd, 0xff), // cyan
            error_color: Color32::from_rgb(0xff, 0x53, 0x70), // red
            warning_color: Color32::from_rgb(0xff, 0xcb, 0x6b), // yellow
            success_color: Color32::from_rgb(0xc3, 0xe8, 0x8d), // green
            info_color: Color32::from_rgb(0x82, 0xaa, 0xff), // blue
            accent: Color32::from_rgb(0x82, 0xaa, 0xff),     // blue
            accent_secondary: Color32::from_rgb(0xc7, 0x92, 0xea), // purple
            link_color: Color32::from_rgb(0x89, 0xdd, 0xff), // cyan
            comment_color: Color32::from_rgb(0x67, 0x6e, 0x95),
            folder_color: Color32::from_rgb(0xff, 0xcb, 0x6b), // yellow
            git_icon_color: Color32::from_rgb(0xf7, 0x8c, 0x6c), // orange
            git_clean_color: Color32::from_rgb(0xc3, 0xe8, 0x8d), // green
            git_dirty_color: Color32::from_rgb(0xff, 0xcb, 0x6b), // yellow
            prompt_color: Color32::from_rgb(0x82, 0xaa, 0xff), // blue
            root_color: Color32::from_rgb(0xff, 0x53, 0x70),   // red
        }
    }

    /// Ayu Dark theme
    pub fn ayu_dark() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x0a, 0x0e, 0x14),
            background_secondary: Color32::from_rgb(0x07, 0x0b, 0x10),
            background_tertiary: Color32::from_rgb(0x05, 0x08, 0x0c),
            foreground: Color32::from_rgb(0xb3, 0xb1, 0xad),
            foreground_dim: Color32::from_rgb(0x5c, 0x67, 0x73),
            cursor: Color32::from_rgb(0xe6, 0xb4, 0x50),
            selection: Color32::from_rgb(0x27, 0x3d, 0x51),
            path_color: Color32::from_rgb(0x95, 0xe6, 0xcb), // cyan/mint
            branch_color: Color32::from_rgb(0xff, 0xb4, 0x54), // orange
            command_color: Color32::from_rgb(0xd2, 0xa6, 0xff), // purple
            string_color: Color32::from_rgb(0xaa, 0xd9, 0x4c), // green
            number_color: Color32::from_rgb(0xe6, 0xb4, 0x50), // yellow/gold
            flag_color: Color32::from_rgb(0x73, 0xd0, 0xff), // blue
            error_color: Color32::from_rgb(0xf0, 0x71, 0x78), // red
            warning_color: Color32::from_rgb(0xff, 0xb4, 0x54), // orange
            success_color: Color32::from_rgb(0xaa, 0xd9, 0x4c), // green
            info_color: Color32::from_rgb(0x59, 0xc2, 0xff), // blue
            accent: Color32::from_rgb(0xe6, 0xb4, 0x50),     // accent
            accent_secondary: Color32::from_rgb(0xd2, 0xa6, 0xff), // purple
            link_color: Color32::from_rgb(0x59, 0xc2, 0xff), // blue
            comment_color: Color32::from_rgb(0x5c, 0x67, 0x73),
            folder_color: Color32::from_rgb(0xff, 0xb4, 0x54), // orange
            git_icon_color: Color32::from_rgb(0xff, 0xb4, 0x54), // orange
            git_clean_color: Color32::from_rgb(0xaa, 0xd9, 0x4c), // green
            git_dirty_color: Color32::from_rgb(0xff, 0xb4, 0x54), // orange
            prompt_color: Color32::from_rgb(0xe6, 0xb4, 0x50), // accent
            root_color: Color32::from_rgb(0xf0, 0x71, 0x78),   // red
        }
    }

    /// Ayu Mirage theme
    pub fn ayu_mirage() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x1f, 0x24, 0x30),
            background_secondary: Color32::from_rgb(0x1a, 0x1f, 0x2a),
            background_tertiary: Color32::from_rgb(0x15, 0x1a, 0x24),
            foreground: Color32::from_rgb(0xcb, 0xcc, 0xc6),
            foreground_dim: Color32::from_rgb(0x5c, 0x67, 0x73),
            cursor: Color32::from_rgb(0xff, 0xcc, 0x66),
            selection: Color32::from_rgb(0x33, 0x41, 0x5e),
            path_color: Color32::from_rgb(0x95, 0xe6, 0xcb),
            branch_color: Color32::from_rgb(0xff, 0xad, 0x66),
            command_color: Color32::from_rgb(0xd4, 0xbf, 0xff),
            string_color: Color32::from_rgb(0xd5, 0xff, 0x80),
            number_color: Color32::from_rgb(0xff, 0xcc, 0x66),
            flag_color: Color32::from_rgb(0x73, 0xd0, 0xff),
            error_color: Color32::from_rgb(0xff, 0x66, 0x66),
            warning_color: Color32::from_rgb(0xff, 0xad, 0x66),
            success_color: Color32::from_rgb(0xd5, 0xff, 0x80),
            info_color: Color32::from_rgb(0x73, 0xd0, 0xff),
            accent: Color32::from_rgb(0xff, 0xcc, 0x66),
            accent_secondary: Color32::from_rgb(0xd4, 0xbf, 0xff),
            link_color: Color32::from_rgb(0x73, 0xd0, 0xff),
            comment_color: Color32::from_rgb(0x5c, 0x67, 0x73),
            folder_color: Color32::from_rgb(0xff, 0xad, 0x66),
            git_icon_color: Color32::from_rgb(0xff, 0xad, 0x66),
            git_clean_color: Color32::from_rgb(0xd5, 0xff, 0x80),
            git_dirty_color: Color32::from_rgb(0xff, 0xad, 0x66),
            prompt_color: Color32::from_rgb(0xff, 0xcc, 0x66),
            root_color: Color32::from_rgb(0xff, 0x66, 0x66),
        }
    }

    /// Kanagawa theme (inspired by Hokusai's The Great Wave)
    pub fn kanagawa() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x1f, 0x1f, 0x28),
            background_secondary: Color32::from_rgb(0x16, 0x16, 0x1d),
            background_tertiary: Color32::from_rgb(0x13, 0x13, 0x1a),
            foreground: Color32::from_rgb(0xdc, 0xd7, 0xba),
            foreground_dim: Color32::from_rgb(0x72, 0x71, 0x69),
            cursor: Color32::from_rgb(0xc8, 0xc0, 0x93),
            selection: Color32::from_rgb(0x2d, 0x4f, 0x67),
            path_color: Color32::from_rgb(0x7f, 0xb4, 0xca), // wave blue
            branch_color: Color32::from_rgb(0xff, 0xa0, 0x66), // autumn orange
            command_color: Color32::from_rgb(0x95, 0x7f, 0xb8), // spring violet
            string_color: Color32::from_rgb(0x98, 0xbb, 0x6c), // spring green
            number_color: Color32::from_rgb(0xd2, 0x7e, 0x99), // sakura pink
            flag_color: Color32::from_rgb(0x7e, 0x9c, 0xd8), // crystal blue
            error_color: Color32::from_rgb(0xc3, 0x4a, 0x43), // autumn red
            warning_color: Color32::from_rgb(0xdc, 0xa5, 0x61), // carpYellow
            success_color: Color32::from_rgb(0x98, 0xbb, 0x6c), // spring green
            info_color: Color32::from_rgb(0x7e, 0x9c, 0xd8), // crystal blue
            accent: Color32::from_rgb(0x7e, 0x9c, 0xd8),     // crystal blue
            accent_secondary: Color32::from_rgb(0x95, 0x7f, 0xb8), // spring violet
            link_color: Color32::from_rgb(0x7f, 0xb4, 0xca), // wave blue
            comment_color: Color32::from_rgb(0x72, 0x71, 0x69),
            folder_color: Color32::from_rgb(0xdc, 0xa5, 0x61), // carpYellow
            git_icon_color: Color32::from_rgb(0xff, 0xa0, 0x66), // autumn orange
            git_clean_color: Color32::from_rgb(0x98, 0xbb, 0x6c), // spring green
            git_dirty_color: Color32::from_rgb(0xdc, 0xa5, 0x61), // carpYellow
            prompt_color: Color32::from_rgb(0x7e, 0x9c, 0xd8), // crystal blue
            root_color: Color32::from_rgb(0xc3, 0x4a, 0x43),   // autumn red
        }
    }

    /// Rosé Pine theme
    pub fn rose_pine() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x19, 0x17, 0x24), // base
            background_secondary: Color32::from_rgb(0x1f, 0x1d, 0x2e), // surface
            background_tertiary: Color32::from_rgb(0x26, 0x23, 0x3a), // overlay
            foreground: Color32::from_rgb(0xe0, 0xde, 0xf4), // text
            foreground_dim: Color32::from_rgb(0x90, 0x8c, 0xaa), // subtle
            cursor: Color32::from_rgb(0xe0, 0xde, 0xf4),
            selection: Color32::from_rgb(0x26, 0x23, 0x3a),
            path_color: Color32::from_rgb(0x9c, 0xcf, 0xd8), // foam
            branch_color: Color32::from_rgb(0xea, 0x9a, 0x97), // rose
            command_color: Color32::from_rgb(0xc4, 0xa7, 0xe7), // iris
            string_color: Color32::from_rgb(0x31, 0x74, 0x8f), // pine
            number_color: Color32::from_rgb(0xf6, 0xc1, 0x77), // gold
            flag_color: Color32::from_rgb(0x9c, 0xcf, 0xd8), // foam
            error_color: Color32::from_rgb(0xeb, 0x6f, 0x92), // love
            warning_color: Color32::from_rgb(0xf6, 0xc1, 0x77), // gold
            success_color: Color32::from_rgb(0x31, 0x74, 0x8f), // pine
            info_color: Color32::from_rgb(0x9c, 0xcf, 0xd8), // foam
            accent: Color32::from_rgb(0xeb, 0x6f, 0x92),     // love
            accent_secondary: Color32::from_rgb(0xc4, 0xa7, 0xe7), // iris
            link_color: Color32::from_rgb(0x9c, 0xcf, 0xd8), // foam
            comment_color: Color32::from_rgb(0x6e, 0x6a, 0x86), // muted
            folder_color: Color32::from_rgb(0xf6, 0xc1, 0x77), // gold
            git_icon_color: Color32::from_rgb(0xea, 0x9a, 0x97), // rose
            git_clean_color: Color32::from_rgb(0x31, 0x74, 0x8f), // pine
            git_dirty_color: Color32::from_rgb(0xf6, 0xc1, 0x77), // gold
            prompt_color: Color32::from_rgb(0xeb, 0x6f, 0x92), // love
            root_color: Color32::from_rgb(0xeb, 0x6f, 0x92), // love
        }
    }

    /// Rosé Pine Moon theme
    pub fn rose_pine_moon() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x23, 0x21, 0x36),
            background_secondary: Color32::from_rgb(0x2a, 0x27, 0x3f),
            background_tertiary: Color32::from_rgb(0x39, 0x35, 0x52),
            foreground: Color32::from_rgb(0xe0, 0xde, 0xf4),
            foreground_dim: Color32::from_rgb(0x90, 0x8c, 0xaa),
            cursor: Color32::from_rgb(0xe0, 0xde, 0xf4),
            selection: Color32::from_rgb(0x39, 0x35, 0x52),
            path_color: Color32::from_rgb(0x9c, 0xcf, 0xd8),
            branch_color: Color32::from_rgb(0xea, 0x9a, 0x97),
            command_color: Color32::from_rgb(0xc4, 0xa7, 0xe7),
            string_color: Color32::from_rgb(0x3e, 0x8f, 0xb0),
            number_color: Color32::from_rgb(0xf6, 0xc1, 0x77),
            flag_color: Color32::from_rgb(0x9c, 0xcf, 0xd8),
            error_color: Color32::from_rgb(0xeb, 0x6f, 0x92),
            warning_color: Color32::from_rgb(0xf6, 0xc1, 0x77),
            success_color: Color32::from_rgb(0x3e, 0x8f, 0xb0),
            info_color: Color32::from_rgb(0x9c, 0xcf, 0xd8),
            accent: Color32::from_rgb(0xeb, 0x6f, 0x92),
            accent_secondary: Color32::from_rgb(0xc4, 0xa7, 0xe7),
            link_color: Color32::from_rgb(0x9c, 0xcf, 0xd8),
            comment_color: Color32::from_rgb(0x6e, 0x6a, 0x86),
            folder_color: Color32::from_rgb(0xf6, 0xc1, 0x77),
            git_icon_color: Color32::from_rgb(0xea, 0x9a, 0x97),
            git_clean_color: Color32::from_rgb(0x3e, 0x8f, 0xb0),
            git_dirty_color: Color32::from_rgb(0xf6, 0xc1, 0x77),
            prompt_color: Color32::from_rgb(0xeb, 0x6f, 0x92),
            root_color: Color32::from_rgb(0xeb, 0x6f, 0x92),
        }
    }

    /// Everforest Dark theme
    pub fn everforest_dark() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x2d, 0x35, 0x3b),
            background_secondary: Color32::from_rgb(0x27, 0x2e, 0x33),
            background_tertiary: Color32::from_rgb(0x21, 0x27, 0x2b),
            foreground: Color32::from_rgb(0xd3, 0xc6, 0xaa),
            foreground_dim: Color32::from_rgb(0x85, 0x9a, 0x89),
            cursor: Color32::from_rgb(0xd3, 0xc6, 0xaa),
            selection: Color32::from_rgb(0x47, 0x52, 0x58),
            path_color: Color32::from_rgb(0x83, 0xc0, 0x92), // aqua
            branch_color: Color32::from_rgb(0xe6, 0x9a, 0x75), // orange
            command_color: Color32::from_rgb(0xd6, 0x99, 0xb6), // purple
            string_color: Color32::from_rgb(0xa7, 0xc0, 0x80), // green
            number_color: Color32::from_rgb(0xd6, 0x99, 0xb6), // purple
            flag_color: Color32::from_rgb(0x7f, 0xbf, 0xb3), // blue
            error_color: Color32::from_rgb(0xe6, 0x72, 0x79), // red
            warning_color: Color32::from_rgb(0xdb, 0xbc, 0x7f), // yellow
            success_color: Color32::from_rgb(0xa7, 0xc0, 0x80), // green
            info_color: Color32::from_rgb(0x7f, 0xbf, 0xb3), // blue
            accent: Color32::from_rgb(0xa7, 0xc0, 0x80),     // green
            accent_secondary: Color32::from_rgb(0xd6, 0x99, 0xb6), // purple
            link_color: Color32::from_rgb(0x7f, 0xbf, 0xb3), // blue
            comment_color: Color32::from_rgb(0x85, 0x9a, 0x89),
            folder_color: Color32::from_rgb(0xdb, 0xbc, 0x7f), // yellow
            git_icon_color: Color32::from_rgb(0xe6, 0x9a, 0x75), // orange
            git_clean_color: Color32::from_rgb(0xa7, 0xc0, 0x80), // green
            git_dirty_color: Color32::from_rgb(0xdb, 0xbc, 0x7f), // yellow
            prompt_color: Color32::from_rgb(0xa7, 0xc0, 0x80), // green
            root_color: Color32::from_rgb(0xe6, 0x72, 0x79),   // red
        }
    }

    /// Night Owl theme
    pub fn night_owl() -> Self {
        Self {
            font_size: 16.0,
            line_height: 1.4,
            background: Color32::from_rgb(0x01, 0x11, 0x27),
            background_secondary: Color32::from_rgb(0x0b, 0x1b, 0x32),
            background_tertiary: Color32::from_rgb(0x00, 0x0a, 0x1a),
            foreground: Color32::from_rgb(0xd6, 0xde, 0xeb),
            foreground_dim: Color32::from_rgb(0x63, 0x78, 0x93),
            cursor: Color32::from_rgb(0x80, 0xa4, 0xc2),
            selection: Color32::from_rgb(0x1d, 0x3b, 0x53),
            path_color: Color32::from_rgb(0x7f, 0xdb, 0xca), // cyan
            branch_color: Color32::from_rgb(0xf7, 0x8c, 0x6c), // orange
            command_color: Color32::from_rgb(0xc7, 0x92, 0xea), // purple
            string_color: Color32::from_rgb(0xad, 0xdb, 0x67), // green
            number_color: Color32::from_rgb(0xf7, 0x8c, 0x6c), // orange
            flag_color: Color32::from_rgb(0x7f, 0xdb, 0xca), // cyan
            error_color: Color32::from_rgb(0xef, 0x53, 0x50), // red
            warning_color: Color32::from_rgb(0xff, 0xcb, 0x8b), // yellow
            success_color: Color32::from_rgb(0xad, 0xdb, 0x67), // green
            info_color: Color32::from_rgb(0x82, 0xaa, 0xff), // blue
            accent: Color32::from_rgb(0x82, 0xaa, 0xff),     // blue
            accent_secondary: Color32::from_rgb(0xc7, 0x92, 0xea), // purple
            link_color: Color32::from_rgb(0x7f, 0xdb, 0xca), // cyan
            comment_color: Color32::from_rgb(0x63, 0x78, 0x93),
            folder_color: Color32::from_rgb(0xff, 0xcb, 0x8b), // yellow
            git_icon_color: Color32::from_rgb(0xf7, 0x8c, 0x6c), // orange
            git_clean_color: Color32::from_rgb(0xad, 0xdb, 0x67), // green
            git_dirty_color: Color32::from_rgb(0xff, 0xcb, 0x8b), // yellow
            prompt_color: Color32::from_rgb(0x82, 0xaa, 0xff), // blue
            root_color: Color32::from_rgb(0xef, 0x53, 0x50),   // red
        }
    }

    /// Load theme from config
    pub fn from_config(config: &ThemeConfig) -> Self {
        let mut theme = Self::default();

        // Helper macro to reduce boilerplate
        macro_rules! apply_color {
            ($field:expr, $config_val:expr) => {
                if let Some(ref hex) = $config_val {
                    if let Some(color) = parse_hex_color(hex) {
                        $field = color;
                    }
                }
            };
        }

        // Base colors
        apply_color!(theme.background, config.background);
        apply_color!(theme.background_secondary, config.background_secondary);
        apply_color!(theme.background_tertiary, config.background_tertiary);
        apply_color!(theme.foreground, config.foreground);
        apply_color!(theme.foreground_dim, config.foreground_dim);

        // UI elements
        apply_color!(theme.cursor, config.cursor);
        apply_color!(theme.selection, config.selection);
        apply_color!(theme.accent, config.accent);
        apply_color!(theme.accent_secondary, config.accent_secondary);

        // Syntax colors
        apply_color!(theme.command_color, config.syntax.command);
        apply_color!(theme.path_color, config.syntax.path);
        apply_color!(theme.string_color, config.syntax.string);
        apply_color!(theme.number_color, config.syntax.number);
        apply_color!(theme.flag_color, config.syntax.flag);
        apply_color!(theme.comment_color, config.syntax.comment);
        apply_color!(theme.link_color, config.syntax.link);

        // Status colors
        apply_color!(theme.error_color, config.status.error);
        apply_color!(theme.warning_color, config.status.warning);
        apply_color!(theme.success_color, config.status.success);
        apply_color!(theme.info_color, config.status.info);

        theme
    }

    /// Apply kawaii mode transformations - lofi aesthetic, cozy vibes
    pub fn apply_kawaii(&self) -> Self {
        let mut theme = self.clone();

        // Lofi cozy backgrounds - warm purple/brown tones
        theme.background = Color32::from_rgb(0x1a, 0x14, 0x1f); // Deep purple-black
        theme.background_secondary = Color32::from_rgb(0x2a, 0x1f, 0x2f); // Warm purple
        theme.foreground = Color32::from_rgb(0xe8, 0xdc, 0xd0); // Warm cream white

        // Sunset pink/orange accents
        theme.accent = Color32::from_rgb(0xf4, 0xa2, 0x9c); // Salmon pink
        theme.accent_secondary = Color32::from_rgb(0xc9, 0x9c, 0xd3); // Muted lavender

        // Warm selection
        theme.selection = Color32::from_rgba_unmultiplied(0xf4, 0xa2, 0x9c, 50);

        // Cozy cursor - warm pink
        theme.cursor = Color32::from_rgb(0xf4, 0x8f, 0xb1);

        // Warm prompt color
        theme.prompt_color = Color32::from_rgb(0xf4, 0xa2, 0x9c);

        // Muted comment color - dusty purple
        theme.comment_color = Color32::from_rgb(0x8a, 0x7a, 0x9a);

        // Soft muted success/error
        theme.success_color = Color32::from_rgb(0xa8, 0xd8, 0xb9); // Sage green
        theme.error_color = Color32::from_rgb(0xe8, 0x8a, 0x8a); // Muted coral

        // Lofi syntax colors
        theme.command_color = Color32::from_rgb(0xf4, 0xa2, 0x9c); // Salmon commands
        theme.string_color = Color32::from_rgb(0xa8, 0xd8, 0xb9); // Sage green strings
        theme.number_color = Color32::from_rgb(0xf4, 0xc4, 0x9c); // Peach numbers
        theme.flag_color = Color32::from_rgb(0x9c, 0xc4, 0xf4); // Muted blue flags
        theme.path_color = Color32::from_rgb(0xc9, 0x9c, 0xd3); // Lavender paths
        theme.link_color = Color32::from_rgb(0x9c, 0xc4, 0xf4); // Muted blue links

        theme
    }
}

/// Kawaii mode icons - cuter alternatives
pub mod kawaii_icons {
    pub const PROMPT: &str = "♡"; // Heart prompt
    pub const PROMPT_ALT: &str = "✿"; // Flower prompt
    pub const FOLDER: &str = "📁"; // Emoji folder
    pub const HOME: &str = "🏠"; // House emoji
    pub const SUCCESS: &str = "✨"; // Sparkles
    pub const ERROR: &str = "💔"; // Broken heart
    pub const WARNING: &str = "⚠️"; // Warning (but cuter context)
    pub const GIT_BRANCH: &str = "🌸"; // Cherry blossom for git
    pub const STAR: &str = "⭐"; // Star
    pub const HEART: &str = "💕"; // Hearts
}
