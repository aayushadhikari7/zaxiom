// Hide console window on Windows (always, not just release)
#![windows_subsystem = "windows"]

//! Zaxiom - A Linux-style terminal for Windows
//!
//! The terminal should adapt to the developer — not the other way around.

mod app;
mod commands;
mod config;
mod git;
mod mascot;
mod shell;
mod terminal;

use app::ZaxiomApp;
use eframe::egui;
use std::sync::Arc;

/// Load the app icon - embedded at compile time
fn load_icon() -> Arc<egui::IconData> {
    // Embed icon bytes at compile time
    let icon_bytes = include_bytes!("../assets/icon.ico");

    // Try to decode the ICO file
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        return Arc::new(egui::IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        });
    }

    // Fallback: create a simple purple icon
    let size = 32u32;
    let rgba: Vec<u8> = (0..size * size)
        .flat_map(|_| vec![168, 85, 247, 255]) // Purple color
        .collect();

    Arc::new(egui::IconData {
        rgba,
        width: size,
        height: size,
    })
}

fn main() -> eframe::Result<()> {
    let icon = load_icon();

    let viewport = egui::ViewportBuilder::default()
        .with_title("Zaxiom")
        .with_inner_size([1100.0, 700.0])
        .with_min_inner_size([600.0, 400.0])
        .with_transparent(true)
        .with_icon(icon);

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Zaxiom",
        options,
        Box::new(|cc| Ok(Box::new(ZaxiomApp::new(cc)))),
    )
}
