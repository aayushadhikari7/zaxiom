//! Terminal rendering utilities
//!
//! Helpers for rendering terminal output in egui.

use eframe::egui;

use crate::config::theme::Theme;
use super::buffer::{OutputLine, LineType};

/// Render an output line with appropriate styling
#[allow(dead_code)]
pub fn render_line(ui: &mut egui::Ui, line: &OutputLine, theme: &Theme) {
    let color = match line.line_type {
        LineType::Normal => theme.foreground,
        LineType::Error => theme.error_color,
        LineType::Command => theme.command_color,
        LineType::Success => theme.success_color,
    };

    ui.add(
        egui::Label::new(
            egui::RichText::new(&line.text)
                .monospace()
                .color(color)
        )
    );
}
