//! Inline image display
//!
//! Supports displaying images in terminal output.

#![allow(dead_code)]

use eframe::egui;
use std::path::Path;

/// Supported image formats
const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "webp", "ico"];

/// Check if a path points to an image file
pub fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Image display state
pub struct ImageDisplay {
    /// Cached texture handle
    texture: Option<egui::TextureHandle>,
    /// Original image size
    size: (u32, u32),
    /// Path to the image
    path: std::path::PathBuf,
}

impl ImageDisplay {
    /// Load an image from a file path
    pub fn load(ctx: &egui::Context, path: &Path) -> Option<Self> {
        // Try to load the image
        let image_data = std::fs::read(path).ok()?;

        // Decode the image
        let image = image::load_from_memory(&image_data).ok()?;
        let rgba = image.to_rgba8();
        let size = (rgba.width(), rgba.height());

        // Create egui color image
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [size.0 as usize, size.1 as usize],
            rgba.as_raw(),
        );

        // Create texture
        let texture = ctx.load_texture(
            path.to_string_lossy(),
            color_image,
            egui::TextureOptions::LINEAR,
        );

        Some(Self {
            texture: Some(texture),
            size,
            path: path.to_path_buf(),
        })
    }

    /// Render the image with a maximum size
    pub fn render(&self, ui: &mut egui::Ui, max_width: f32, max_height: f32) {
        if let Some(texture) = &self.texture {
            // Calculate scaled size maintaining aspect ratio
            let aspect = self.size.0 as f32 / self.size.1 as f32;

            let (width, height) =
                if self.size.0 as f32 > max_width || self.size.1 as f32 > max_height {
                    if max_width / aspect <= max_height {
                        (max_width, max_width / aspect)
                    } else {
                        (max_height * aspect, max_height)
                    }
                } else {
                    (self.size.0 as f32, self.size.1 as f32)
                };

            ui.image((texture.id(), egui::vec2(width, height)));
        }
    }

    /// Get the image dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        self.size
    }

    /// Get the file path
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// ASCII art representation of an image (fallback)
pub fn image_to_ascii(path: &Path, width: u32) -> Option<String> {
    let image_data = std::fs::read(path).ok()?;
    let image = image::load_from_memory(&image_data).ok()?;

    // Calculate height maintaining aspect ratio (characters are ~2x taller than wide)
    let aspect = image.width() as f32 / image.height() as f32;
    let height = (width as f32 / aspect / 2.0) as u32;

    // Resize the image
    let resized = image.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let gray = resized.to_luma8();

    // ASCII characters from dark to light
    let chars = " .:-=+*#%@";

    let mut result = String::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = gray.get_pixel(x, y).0[0];
            let idx = (pixel as usize * (chars.len() - 1)) / 255;
            result.push(chars.chars().nth(idx).unwrap_or(' '));
        }
        result.push('\n');
    }

    Some(result)
}

/// Format image info for display
pub fn format_image_info(path: &Path) -> Option<String> {
    let image_data = std::fs::read(path).ok()?;
    let image = image::load_from_memory(&image_data).ok()?;

    let (width, height) = (image.width(), image.height());
    let color_type = match image.color() {
        image::ColorType::L8 => "Grayscale",
        image::ColorType::La8 => "Grayscale+Alpha",
        image::ColorType::Rgb8 => "RGB",
        image::ColorType::Rgba8 => "RGBA",
        image::ColorType::L16 => "Grayscale 16-bit",
        image::ColorType::La16 => "Grayscale+Alpha 16-bit",
        image::ColorType::Rgb16 => "RGB 16-bit",
        image::ColorType::Rgba16 => "RGBA 16-bit",
        image::ColorType::Rgb32F => "RGB 32-bit float",
        image::ColorType::Rgba32F => "RGBA 32-bit float",
        _ => "Unknown",
    };

    let file_size = std::fs::metadata(path).ok()?.len();
    let size_str = if file_size > 1024 * 1024 {
        format!("{:.1} MB", file_size as f64 / (1024.0 * 1024.0))
    } else if file_size > 1024 {
        format!("{:.1} KB", file_size as f64 / 1024.0)
    } else {
        format!("{} bytes", file_size)
    };

    Some(format!(
        "{}x{} {} ({})",
        width, height, color_type, size_str
    ))
}
