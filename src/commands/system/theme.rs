//! theme command - list and switch terminal themes

use anyhow::Result;

use crate::commands::traits::Command;
use crate::config::theme::ThemeName;
use crate::terminal::state::TerminalState;

pub struct ThemeCommand;

impl Command for ThemeCommand {
    fn name(&self) -> &'static str {
        "theme"
    }

    fn description(&self) -> &'static str {
        "List or switch terminal themes"
    }

    fn usage(&self) -> &'static str {
        "theme [name] [--kawaii|--normal]\n\n\
         Examples:\n  \
         theme           - List all available themes\n  \
         theme dracula   - Switch to Dracula theme\n  \
         theme nord      - Switch to Nord theme\n  \
         theme list      - List all themes with descriptions\n  \
         theme --kawaii  - Enable kawaii mode (cuter UI)\n  \
         theme --normal  - Disable kawaii mode"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        // Handle kawaii mode flags
        if let Some(first_arg) = args.first() {
            match first_arg.as_str() {
                "--kawaii" | "-k" => {
                    state.set_kawaii_mode(true);
                    return Ok("Kawaii mode enabled! ♡(◕‿◕)♡ Everything is cuter now~".to_string());
                }
                "--normal" | "-n" => {
                    state.set_kawaii_mode(false);
                    return Ok("Kawaii mode disabled. Back to normal mode.".to_string());
                }
                _ => {}
            }
        }

        if args.is_empty() || args.first().map(|s| s.as_str()) == Some("list") {
            // List all themes with current theme highlighted
            return Ok(self.list_themes(state.current_theme, state.kawaii_mode));
        }

        let theme_name = args.join(" ").to_lowercase();

        // Try to match theme name
        if let Some(theme) = self.parse_theme_name(&theme_name) {
            state.requested_theme = Some(theme);
            Ok(format!("Switched to {} theme", theme.display_name()))
        } else {
            // Show suggestions
            let suggestions = self.suggest_themes(&theme_name);
            if suggestions.is_empty() {
                Ok(format!(
                    "Unknown theme: '{}'\n\nRun 'theme list' to see available themes.",
                    theme_name
                ))
            } else {
                Ok(format!(
                    "Unknown theme: '{}'\n\nDid you mean?\n{}",
                    theme_name,
                    suggestions.join("\n")
                ))
            }
        }
    }
}

impl ThemeCommand {
    fn list_themes(&self, current: ThemeName, kawaii_mode: bool) -> String {
        let kawaii_status = if kawaii_mode { "on ♡" } else { "off" };
        let mut output = format!(
            "Current theme: {} ✨  (kawaii mode: {})\n\n",
            current.display_name(),
            kawaii_status
        );
        output.push_str("Available themes:\n\n");

        // Dark themes
        output.push_str("Dark themes:\n");
        for theme in ThemeName::all() {
            if !theme.is_light() {
                let marker = if *theme == current { " ◀" } else { "" };
                output.push_str(&format!("  {:20} {}{}\n",
                    self.theme_to_arg(*theme),
                    theme.display_name(),
                    marker
                ));
            }
        }

        // Light themes
        output.push_str("\nLight themes:\n");
        for theme in ThemeName::all() {
            if theme.is_light() {
                let marker = if *theme == current { " ◀" } else { "" };
                output.push_str(&format!("  {:20} {}{}\n",
                    self.theme_to_arg(*theme),
                    theme.display_name(),
                    marker
                ));
            }
        }

        output.push_str("\nUsage: theme <name>");
        output
    }

    fn theme_to_arg(&self, theme: ThemeName) -> String {
        match theme {
            ThemeName::CatppuccinMocha => "catppuccin",
            ThemeName::CatppuccinLatte => "catppuccin-latte",
            ThemeName::Dracula => "dracula",
            ThemeName::Nord => "nord",
            ThemeName::GruvboxDark => "gruvbox",
            ThemeName::GruvboxLight => "gruvbox-light",
            ThemeName::TokyoNight => "tokyo-night",
            ThemeName::TokyoNightStorm => "tokyo-storm",
            ThemeName::OneDark => "one-dark",
            ThemeName::SolarizedDark => "solarized",
            ThemeName::SolarizedLight => "solarized-light",
            ThemeName::MonokaiPro => "monokai",
            ThemeName::Palenight => "palenight",
            ThemeName::AyuDark => "ayu",
            ThemeName::AyuMirage => "ayu-mirage",
            ThemeName::Kanagawa => "kanagawa",
            ThemeName::RosePine => "rose-pine",
            ThemeName::RosePineMoon => "rose-pine-moon",
            ThemeName::EverforestDark => "everforest",
            ThemeName::NightOwl => "night-owl",
        }.to_string()
    }

    fn parse_theme_name(&self, name: &str) -> Option<ThemeName> {
        let name = name.trim().to_lowercase();
        let name = name.replace(['-', '_', ' '], "");

        match name.as_str() {
            // Catppuccin variants
            "catppuccin" | "catppuccinmocha" | "mocha" => Some(ThemeName::CatppuccinMocha),
            "catppuccinlatte" | "latte" => Some(ThemeName::CatppuccinLatte),

            // Popular themes
            "dracula" => Some(ThemeName::Dracula),
            "nord" => Some(ThemeName::Nord),

            // Gruvbox variants
            "gruvbox" | "gruvboxdark" => Some(ThemeName::GruvboxDark),
            "gruvboxlight" => Some(ThemeName::GruvboxLight),

            // Tokyo Night variants
            "tokyonight" | "tokyo" => Some(ThemeName::TokyoNight),
            "tokyonightstorm" | "tokyostorm" | "storm" => Some(ThemeName::TokyoNightStorm),

            // One Dark
            "onedark" | "one" | "atom" => Some(ThemeName::OneDark),

            // Solarized variants
            "solarized" | "solarizeddark" | "solar" => Some(ThemeName::SolarizedDark),
            "solarizedlight" => Some(ThemeName::SolarizedLight),

            // Monokai
            "monokai" | "monokaipro" => Some(ThemeName::MonokaiPro),

            // Palenight
            "palenight" | "material" => Some(ThemeName::Palenight),

            // Ayu variants
            "ayu" | "ayudark" => Some(ThemeName::AyuDark),
            "ayumirage" | "mirage" => Some(ThemeName::AyuMirage),

            // Japanese-inspired
            "kanagawa" | "wave" => Some(ThemeName::Kanagawa),

            // Rosé Pine variants
            "rosepine" | "rose" | "pine" => Some(ThemeName::RosePine),
            "rosepinemoon" | "moon" => Some(ThemeName::RosePineMoon),

            // Nature themes
            "everforest" | "forest" => Some(ThemeName::EverforestDark),
            "nightowl" | "owl" => Some(ThemeName::NightOwl),

            _ => None,
        }
    }

    fn suggest_themes(&self, input: &str) -> Vec<String> {
        let input = input.to_lowercase();
        let mut suggestions = Vec::new();

        let all_names = [
            ("catppuccin", "Catppuccin Mocha"),
            ("dracula", "Dracula"),
            ("nord", "Nord"),
            ("gruvbox", "Gruvbox Dark"),
            ("tokyo-night", "Tokyo Night"),
            ("one-dark", "One Dark"),
            ("solarized", "Solarized Dark"),
            ("monokai", "Monokai Pro"),
            ("palenight", "Palenight"),
            ("ayu", "Ayu Dark"),
            ("kanagawa", "Kanagawa"),
            ("rose-pine", "Rosé Pine"),
            ("everforest", "Everforest"),
            ("night-owl", "Night Owl"),
        ];

        for (arg, display) in all_names {
            if arg.contains(&input) || display.to_lowercase().contains(&input) {
                suggestions.push(format!("  {} - {}", arg, display));
            }
        }

        // Fuzzy match if no direct matches
        if suggestions.is_empty() {
            for (arg, display) in all_names {
                if self.fuzzy_match(&input, arg) {
                    suggestions.push(format!("  {} - {}", arg, display));
                }
            }
        }

        suggestions.truncate(5);
        suggestions
    }

    fn fuzzy_match(&self, needle: &str, haystack: &str) -> bool {
        let mut needle_chars = needle.chars().peekable();
        for c in haystack.chars() {
            if needle_chars.peek() == Some(&c) {
                needle_chars.next();
            }
        }
        needle_chars.peek().is_none()
    }
}
