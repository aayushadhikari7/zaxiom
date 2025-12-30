//! Mascot system
//!
//! A cute robot companion drawn with egui primitives for maximum performance.

use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Vec2};
use std::time::{Duration, Instant};

/// Mascot mood/state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MascotMood {
    Idle,
    Thinking,
    Happy,
    Sad,
    Excited,
    Sleepy,
    Waving,
    Love,        // Heart eyes! (◕‿◕)♡
    Surprised,   // O_O
    Proud,       // Sparkly eyes for success
    Confused,    // ?_? spiral eyes
    Dancing,     // ^o^ happy dance
    Celebrating, // ★▽★ confetti time!
    Typing,      // ._. focused typing
}

/// Color palette for the mascot
struct MascotColors {
    body_white: Color32,
    body_pink: Color32,
    body_pink_dark: Color32,
    visor: Color32,
    visor_glow: Color32,
    eye_cyan: Color32,
    eye_pink: Color32,
    antenna_pink: Color32,
    highlight: Color32,
}

impl Default for MascotColors {
    fn default() -> Self {
        Self {
            body_white: Color32::from_rgb(245, 245, 250),
            body_pink: Color32::from_rgb(255, 180, 200),
            body_pink_dark: Color32::from_rgb(230, 150, 170),
            visor: Color32::from_rgb(30, 35, 45),
            visor_glow: Color32::from_rgb(40, 50, 65),
            eye_cyan: Color32::from_rgb(100, 220, 255),
            eye_pink: Color32::from_rgb(255, 120, 150),
            antenna_pink: Color32::from_rgb(255, 150, 180),
            highlight: Color32::from_rgb(255, 255, 255),
        }
    }
}

/// The mascot companion
pub struct Mascot {
    pub mood: MascotMood,
    mood_started: Instant,
    last_activity: Instant,
    frame: u64,
    is_blinking: bool,
    blink_until: Instant,
    colors: MascotColors,
}

impl Default for Mascot {
    fn default() -> Self {
        Self::new()
    }
}

impl Mascot {
    pub fn new() -> Self {
        Self {
            mood: MascotMood::Waving,
            mood_started: Instant::now(),
            last_activity: Instant::now(),
            frame: 0,
            is_blinking: false,
            blink_until: Instant::now(),
            colors: MascotColors::default(),
        }
    }

    /// Update mascot state (call every frame)
    pub fn update(&mut self) {
        self.frame = self.frame.wrapping_add(1);

        // Check for sleepy mode
        if self.last_activity.elapsed() > Duration::from_secs(30)
            && self.mood != MascotMood::Sleepy
        {
            self.set_mood(MascotMood::Sleepy);
        }

        // Random blinking
        if !self.is_blinking && self.frame.is_multiple_of(120) && self.rand_bool(0.4) {
            self.is_blinking = true;
            self.blink_until = Instant::now() + Duration::from_millis(100);
        }
        if self.is_blinking && Instant::now() > self.blink_until {
            self.is_blinking = false;
        }

        // Return to idle after mood duration
        let mood_duration = match self.mood {
            MascotMood::Happy => Duration::from_secs(2),
            MascotMood::Sad => Duration::from_secs(3),
            MascotMood::Excited => Duration::from_secs(2),
            MascotMood::Thinking => Duration::from_secs(30),
            MascotMood::Waving => Duration::from_secs(2),
            MascotMood::Love => Duration::from_secs(3),
            MascotMood::Surprised => Duration::from_secs(1),
            MascotMood::Proud => Duration::from_secs(3),
            MascotMood::Confused => Duration::from_secs(2),
            MascotMood::Dancing => Duration::from_secs(4),
            MascotMood::Celebrating => Duration::from_secs(3),
            MascotMood::Typing => Duration::from_secs(999999), // Stays until done typing
            _ => Duration::from_secs(999999),
        };

        if self.mood != MascotMood::Idle
            && self.mood != MascotMood::Sleepy
            && self.mood_started.elapsed() > mood_duration
        {
            self.mood = MascotMood::Idle;
        }
    }

    pub fn set_mood(&mut self, mood: MascotMood) {
        self.mood = mood;
        self.mood_started = Instant::now();
    }

    pub fn activity(&mut self) {
        self.last_activity = Instant::now();
        if self.mood == MascotMood::Sleepy {
            self.set_mood(MascotMood::Idle);
        }
    }

    /// React to command execution
    pub fn on_command(&mut self, command: &str, success: bool) {
        self.activity();

        let cmd = command.split_whitespace().next().unwrap_or("");
        let cmd_lower = command.to_lowercase();

        // Check for confused state on unknown command (failure)
        if !success {
            // Check if it looks like an unknown command
            if cmd_lower.contains("not found") || cmd_lower.contains("unknown") {
                self.set_mood(MascotMood::Confused);
            } else {
                self.set_mood(MascotMood::Sad);
            }
            return;
        }

        // Check for special keywords first
        if cmd_lower.contains("love") || cmd_lower.contains("cute") || cmd_lower.contains("kawaii") {
            self.set_mood(MascotMood::Love);
            return;
        }
        if cmd_lower.contains("thank") || cmd_lower.contains("nice") || cmd_lower.contains("good") {
            self.set_mood(MascotMood::Happy);
            return;
        }
        if cmd_lower.contains("party") || cmd_lower.contains("dance") || cmd_lower.contains("music") {
            self.set_mood(MascotMood::Dancing);
            return;
        }
        if cmd_lower.contains("yay") || cmd_lower.contains("success") || cmd_lower.contains("woohoo") {
            self.set_mood(MascotMood::Celebrating);
            return;
        }

        match cmd {
            // Happy reactions
            "clear" | "theme" => {
                self.set_mood(MascotMood::Happy);
            }
            // Excited for git stuff
            "gs" | "gc" | "gp" | "gpl" | "ga" | "git" => {
                self.set_mood(MascotMood::Excited);
            }
            // Thinking for network/long operations
            "curl" | "wget" | "ping" | "find" | "grep" => {
                self.set_mood(MascotMood::Thinking);
            }
            // Celebrating for successful builds!
            "cargo" | "npm" | "make" | "build" => {
                self.set_mood(MascotMood::Celebrating);
            }
            // Love for fun commands
            "fortune" | "cowsay" | "neofetch" | "coffee" | "pet" => {
                self.set_mood(MascotMood::Love);
            }
            // Dancing for party!
            "party" | "matrix" => {
                self.set_mood(MascotMood::Dancing);
            }
            // Surprised for danger zone
            "rm" | "kill" => {
                self.set_mood(MascotMood::Surprised);
            }
            // Sad for exit
            "exit" | "quit" => {
                self.set_mood(MascotMood::Sad);
            }
            _ => {}
        }
    }

    fn rand_bool(&self, probability: f64) -> bool {
        let seed = self.frame.wrapping_mul(1103515245).wrapping_add(12345);
        (seed % 1000) as f64 / 1000.0 < probability
    }

    /// Render the mascot in the given area
    pub fn render(&mut self, ui: &mut egui::Ui) {
        let size = Vec2::new(70.0, 95.0);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;
        let center = rect.center();

        // Animation offsets
        let bounce = match self.mood {
            MascotMood::Excited => (self.frame as f32 * 0.4).sin() * 4.0,
            MascotMood::Happy => (self.frame as f32 * 0.25).sin() * 2.5,
            MascotMood::Sleepy => (self.frame as f32 * 0.08).sin() * 1.5,
            MascotMood::Dancing => (self.frame as f32 * 0.35).sin() * 3.5,
            MascotMood::Celebrating => (self.frame as f32 * 0.5).sin().abs() * 6.0, // Jump!
            MascotMood::Confused => (self.frame as f32 * 0.15).sin() * 1.0,
            MascotMood::Typing => (self.frame as f32 * 0.1).sin() * 0.5, // Subtle
            _ => (self.frame as f32 * 0.12).sin() * 1.0,
        };

        // Side-to-side sway for dancing
        let sway = if self.mood == MascotMood::Dancing {
            (self.frame as f32 * 0.2).sin() * 5.0
        } else if self.mood == MascotMood::Confused {
            (self.frame as f32 * 0.3).sin() * 2.0 // Head tilt
        } else {
            0.0
        };

        let head_center = Pos2::new(center.x + sway, rect.top() + 28.0 + bounce);

        // Draw special effects first (behind mascot)
        if self.mood == MascotMood::Celebrating {
            self.draw_confetti(&painter, center);
        }
        if self.mood == MascotMood::Confused {
            self.draw_question_mark(&painter, head_center);
        }

        // Draw antenna
        self.draw_antenna(&painter, head_center);

        // Draw ears (pink circles on sides)
        self.draw_ears(&painter, head_center);

        // Draw head (main white rounded rect)
        self.draw_head(&painter, head_center);

        // Draw visor/screen
        self.draw_visor(&painter, head_center);

        // Draw eyes on visor
        self.draw_eyes(&painter, head_center);

        // Draw body
        let body_center = Pos2::new(center.x, head_center.y + 38.0);
        self.draw_body(&painter, body_center);

        // Draw arms
        self.draw_arms(&painter, body_center);

        // Draw feet/wheels
        self.draw_feet(&painter, body_center);

        // Draw highlight/shine
        self.draw_highlights(&painter, head_center);
    }

    fn draw_antenna(&self, painter: &egui::Painter, head_center: Pos2) {
        let antenna_base = Pos2::new(head_center.x - 15.0, head_center.y - 22.0);
        let antenna_top = Pos2::new(head_center.x - 20.0, head_center.y - 35.0);

        // Antenna stem
        painter.line_segment(
            [antenna_base, antenna_top],
            Stroke::new(2.5, self.colors.body_pink_dark),
        );

        // Antenna ball (glowing based on mood)
        let glow_color = match self.mood {
            MascotMood::Excited => Color32::from_rgb(255, 100, 150),
            MascotMood::Happy => Color32::from_rgb(255, 150, 180),
            MascotMood::Thinking => Color32::from_rgb(255, 200, 100),
            _ => self.colors.antenna_pink,
        };

        // Glow effect
        painter.circle_filled(antenna_top, 7.0, glow_color.gamma_multiply(0.3));
        painter.circle_filled(antenna_top, 5.0, glow_color);
        painter.circle_filled(Pos2::new(antenna_top.x - 1.5, antenna_top.y - 1.5), 2.0, self.colors.highlight);
    }

    fn draw_ears(&self, painter: &egui::Painter, head_center: Pos2) {
        // Left ear (pink circular speaker)
        let left_ear = Pos2::new(head_center.x - 28.0, head_center.y);
        painter.circle_filled(left_ear, 10.0, self.colors.body_pink);
        painter.circle_filled(left_ear, 6.0, self.colors.body_pink_dark);
        painter.circle_stroke(left_ear, 10.0, Stroke::new(1.0, self.colors.body_pink_dark));

        // Right ear
        let right_ear = Pos2::new(head_center.x + 28.0, head_center.y);
        painter.circle_filled(right_ear, 8.0, self.colors.body_pink);
        painter.circle_filled(right_ear, 5.0, self.colors.body_pink_dark);
    }

    fn draw_head(&self, painter: &egui::Painter, head_center: Pos2) {
        let head_rect = Rect::from_center_size(head_center, Vec2::new(48.0, 40.0));

        // Main head shape
        painter.rect_filled(head_rect, CornerRadius::same(12), self.colors.body_white);
        painter.rect_stroke(head_rect, CornerRadius::same(12), Stroke::new(1.0, Color32::from_rgb(220, 220, 225)), StrokeKind::Outside);

        // Small pink accent on top
        let accent_rect = Rect::from_center_size(
            Pos2::new(head_center.x + 8.0, head_center.y - 16.0),
            Vec2::new(6.0, 6.0),
        );
        painter.circle_filled(accent_rect.center(), 3.0, self.colors.body_pink);
    }

    fn draw_visor(&self, painter: &egui::Painter, head_center: Pos2) {
        let visor_rect = Rect::from_center_size(
            Pos2::new(head_center.x, head_center.y + 2.0),
            Vec2::new(38.0, 20.0),
        );

        // Visor background (dark screen)
        painter.rect_filled(visor_rect, CornerRadius::same(6), self.colors.visor);

        // Screen glow effect
        let glow_rect = visor_rect.shrink(2.0);
        painter.rect_filled(glow_rect, CornerRadius::same(4), self.colors.visor_glow);

        // Screen border
        painter.rect_stroke(visor_rect, CornerRadius::same(6), Stroke::new(2.0, self.colors.body_pink), StrokeKind::Outside);
    }

    fn draw_eyes(&self, painter: &egui::Painter, head_center: Pos2) {
        let eye_y = head_center.y + 2.0;
        let left_eye = Pos2::new(head_center.x - 9.0, eye_y);
        let right_eye = Pos2::new(head_center.x + 9.0, eye_y);

        if self.is_blinking || self.mood == MascotMood::Sleepy {
            // Closed eyes (horizontal lines)
            painter.line_segment(
                [Pos2::new(left_eye.x - 4.0, left_eye.y), Pos2::new(left_eye.x + 4.0, left_eye.y)],
                Stroke::new(2.0, self.colors.eye_cyan),
            );
            painter.line_segment(
                [Pos2::new(right_eye.x - 4.0, right_eye.y), Pos2::new(right_eye.x + 4.0, right_eye.y)],
                Stroke::new(2.0, self.colors.eye_cyan),
            );
        } else if self.mood == MascotMood::Love {
            // Heart eyes! ♡‿♡
            let heart_color = Color32::from_rgb(255, 100, 150);
            self.draw_heart(painter, left_eye, 5.0, heart_color);
            self.draw_heart(painter, right_eye, 5.0, heart_color);
        } else if self.mood == MascotMood::Surprised {
            // Big O_O eyes
            painter.circle_filled(left_eye, 6.0, self.colors.eye_cyan);
            painter.circle_filled(left_eye, 3.0, Color32::BLACK);
            painter.circle_filled(right_eye, 6.0, self.colors.eye_pink);
            painter.circle_filled(right_eye, 3.0, Color32::BLACK);
            // Tiny highlight
            painter.circle_filled(Pos2::new(left_eye.x - 2.0, left_eye.y - 2.0), 1.5, self.colors.highlight);
            painter.circle_filled(Pos2::new(right_eye.x - 2.0, right_eye.y - 2.0), 1.5, self.colors.highlight);
        } else if self.mood == MascotMood::Proud || self.mood == MascotMood::Celebrating {
            // Sparkly star eyes ★‿★
            let star_color = Color32::from_rgb(255, 220, 100);
            self.draw_star(painter, left_eye, 5.0, star_color);
            self.draw_star(painter, right_eye, 5.0, star_color);
        } else if self.mood == MascotMood::Confused {
            // Spiral/question eyes ?_?
            let spiral_color = Color32::from_rgb(150, 150, 255);
            // Draw spiral-like pattern
            painter.circle_stroke(left_eye, 4.0, Stroke::new(2.0, spiral_color));
            painter.circle_stroke(left_eye, 2.0, Stroke::new(1.5, spiral_color));
            painter.circle_stroke(right_eye, 4.0, Stroke::new(2.0, spiral_color));
            painter.circle_stroke(right_eye, 2.0, Stroke::new(1.5, spiral_color));
        } else if self.mood == MascotMood::Dancing {
            // Happy closed eyes ^o^
            let happy_color = Color32::from_rgb(255, 180, 200);
            // Draw ^ shapes
            painter.line_segment(
                [Pos2::new(left_eye.x - 3.0, left_eye.y + 2.0), Pos2::new(left_eye.x, left_eye.y - 2.0)],
                Stroke::new(2.0, happy_color),
            );
            painter.line_segment(
                [Pos2::new(left_eye.x, left_eye.y - 2.0), Pos2::new(left_eye.x + 3.0, left_eye.y + 2.0)],
                Stroke::new(2.0, happy_color),
            );
            painter.line_segment(
                [Pos2::new(right_eye.x - 3.0, right_eye.y + 2.0), Pos2::new(right_eye.x, right_eye.y - 2.0)],
                Stroke::new(2.0, happy_color),
            );
            painter.line_segment(
                [Pos2::new(right_eye.x, right_eye.y - 2.0), Pos2::new(right_eye.x + 3.0, right_eye.y + 2.0)],
                Stroke::new(2.0, happy_color),
            );
        } else if self.mood == MascotMood::Typing {
            // Focused dot eyes ._.
            let focus_color = self.colors.eye_cyan;
            painter.circle_filled(left_eye, 2.5, focus_color);
            painter.circle_filled(right_eye, 2.5, focus_color);
            // Small concentration line between eyes
            painter.line_segment(
                [Pos2::new(left_eye.x + 4.0, eye_y + 3.0), Pos2::new(right_eye.x - 4.0, eye_y + 3.0)],
                Stroke::new(1.0, Color32::from_rgb(100, 100, 120)),
            );
        } else {
            // Normal eye colors based on mood
            let (left_color, right_color) = match self.mood {
                MascotMood::Sad => (self.colors.eye_cyan.gamma_multiply(0.6), self.colors.eye_pink.gamma_multiply(0.6)),
                MascotMood::Excited => (Color32::from_rgb(255, 200, 100), Color32::from_rgb(255, 150, 200)),
                _ => (self.colors.eye_cyan, self.colors.eye_pink),
            };

            // Left eye (cyan LED dots)
            painter.circle_filled(left_eye, 5.0, left_color.gamma_multiply(0.3));
            painter.circle_filled(left_eye, 4.0, left_color);

            // Right eye (pink LED dots)
            painter.circle_filled(right_eye, 5.0, right_color.gamma_multiply(0.3));
            painter.circle_filled(right_eye, 4.0, right_color);

            // Eye shine
            painter.circle_filled(Pos2::new(left_eye.x - 1.5, left_eye.y - 1.5), 1.5, self.colors.highlight);
            painter.circle_filled(Pos2::new(right_eye.x - 1.5, right_eye.y - 1.5), 1.5, self.colors.highlight);

            // Mood-specific eye details
            if self.mood == MascotMood::Sad {
                // Sad eyes - add tear drop
                let tear = Pos2::new(left_eye.x + 5.0, left_eye.y + 6.0);
                painter.circle_filled(tear, 2.0, Color32::from_rgb(100, 180, 255));
            }
        }
    }

    /// Draw a cute heart shape
    fn draw_heart(&self, painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
        // Simple heart using two circles and a triangle
        let r = size * 0.4;
        let left_circle = Pos2::new(center.x - r * 0.6, center.y - r * 0.3);
        let right_circle = Pos2::new(center.x + r * 0.6, center.y - r * 0.3);
        let bottom = Pos2::new(center.x, center.y + size * 0.5);

        painter.circle_filled(left_circle, r, color);
        painter.circle_filled(right_circle, r, color);

        // Triangle for bottom of heart
        let points = vec![
            Pos2::new(center.x - size * 0.5, center.y - r * 0.1),
            Pos2::new(center.x + size * 0.5, center.y - r * 0.1),
            bottom,
        ];
        painter.add(egui::Shape::convex_polygon(points, color, Stroke::NONE));
    }

    /// Draw a sparkly star
    fn draw_star(&self, painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
        // 4-pointed star
        let points = vec![
            Pos2::new(center.x, center.y - size),        // top
            Pos2::new(center.x + size * 0.3, center.y - size * 0.3),
            Pos2::new(center.x + size, center.y),        // right
            Pos2::new(center.x + size * 0.3, center.y + size * 0.3),
            Pos2::new(center.x, center.y + size),        // bottom
            Pos2::new(center.x - size * 0.3, center.y + size * 0.3),
            Pos2::new(center.x - size, center.y),        // left
            Pos2::new(center.x - size * 0.3, center.y - size * 0.3),
        ];
        painter.add(egui::Shape::convex_polygon(points, color, Stroke::NONE));
        // Center glow
        painter.circle_filled(center, size * 0.3, self.colors.highlight);
    }

    fn draw_body(&self, painter: &egui::Painter, body_center: Pos2) {
        let body_rect = Rect::from_center_size(body_center, Vec2::new(36.0, 28.0));

        // Main body
        painter.rect_filled(body_rect, CornerRadius::same(8), self.colors.body_white);

        // Pink belly panel
        let belly_rect = Rect::from_center_size(
            Pos2::new(body_center.x, body_center.y - 2.0),
            Vec2::new(20.0, 16.0),
        );
        painter.rect_filled(belly_rect, CornerRadius::same(4), self.colors.body_pink);

        // Body outline
        painter.rect_stroke(body_rect, CornerRadius::same(8), Stroke::new(1.0, Color32::from_rgb(220, 220, 225)), StrokeKind::Outside);
    }

    fn draw_arms(&self, painter: &egui::Painter, body_center: Pos2) {
        let arm_y = body_center.y - 5.0;

        // Arm angles based on mood
        let (left_arm_angle, right_arm_angle) = match self.mood {
            MascotMood::Waving => (0.3, -0.8 + (self.frame as f32 * 0.3).sin() * 0.3),
            MascotMood::Typing => {
                // Both arms forward, alternating movement
                let typing_offset = (self.frame as f32 * 0.4).sin() * 0.15;
                (-0.2 + typing_offset, -0.2 - typing_offset)
            }
            MascotMood::Dancing => {
                // Arms up and swaying
                let dance_offset = (self.frame as f32 * 0.25).sin() * 0.3;
                (-0.6 + dance_offset, -0.6 - dance_offset)
            }
            MascotMood::Celebrating => {
                // Both arms up!
                (-1.0, -1.0)
            }
            _ => (0.3, 0.3),
        };

        // Left arm
        let left_arm_start = Pos2::new(body_center.x - 18.0, arm_y);
        let arm_len = 12.0;
        let left_arm_end = Pos2::new(
            left_arm_start.x - arm_len * left_arm_angle.cos(),
            left_arm_start.y + arm_len * left_arm_angle.sin(),
        );
        painter.line_segment([left_arm_start, left_arm_end], Stroke::new(4.0, self.colors.body_white));
        painter.circle_filled(left_arm_end, 4.0, self.colors.body_pink);

        // Right arm
        let right_arm_start = Pos2::new(body_center.x + 18.0, arm_y);
        let right_arm_end = Pos2::new(
            right_arm_start.x + arm_len * right_arm_angle.cos(),
            right_arm_start.y + arm_len * right_arm_angle.sin(),
        );
        painter.line_segment([right_arm_start, right_arm_end], Stroke::new(4.0, self.colors.body_white));
        painter.circle_filled(right_arm_end, 4.0, self.colors.body_pink);
    }

    fn draw_feet(&self, painter: &egui::Painter, body_center: Pos2) {
        let feet_y = body_center.y + 16.0;

        // Left wheel/foot
        let left_foot = Pos2::new(body_center.x - 10.0, feet_y);
        painter.circle_filled(left_foot, 7.0, self.colors.body_pink);
        painter.circle_filled(left_foot, 4.0, self.colors.body_pink_dark);
        painter.circle_stroke(left_foot, 7.0, Stroke::new(1.0, self.colors.body_pink_dark));

        // Right wheel/foot
        let right_foot = Pos2::new(body_center.x + 10.0, feet_y);
        painter.circle_filled(right_foot, 7.0, self.colors.body_pink);
        painter.circle_filled(right_foot, 4.0, self.colors.body_pink_dark);
        painter.circle_stroke(right_foot, 7.0, Stroke::new(1.0, self.colors.body_pink_dark));
    }

    fn draw_highlights(&self, painter: &egui::Painter, head_center: Pos2) {
        // Glossy highlight on head
        let highlight_pos = Pos2::new(head_center.x - 12.0, head_center.y - 10.0);
        painter.circle_filled(highlight_pos, 4.0, Color32::from_rgba_unmultiplied(255, 255, 255, 100));
        painter.circle_filled(
            Pos2::new(highlight_pos.x + 2.0, highlight_pos.y + 3.0),
            2.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, 60),
        );
    }

    /// Draw confetti for celebrating mood
    fn draw_confetti(&self, painter: &egui::Painter, center: Pos2) {
        let confetti_colors = [
            Color32::from_rgb(255, 100, 150), // Pink
            Color32::from_rgb(100, 200, 255), // Cyan
            Color32::from_rgb(255, 220, 100), // Yellow
            Color32::from_rgb(150, 255, 150), // Green
            Color32::from_rgb(200, 150, 255), // Purple
        ];

        // Animated confetti particles
        for i in 0..12 {
            let seed = (self.frame as f32 * 0.05 + i as f32 * 0.8).sin();
            let x_offset = (seed * 40.0) + (i as f32 - 6.0) * 8.0;
            let y_offset = ((self.frame as f32 * 0.1 + i as f32 * 0.5).sin() * 20.0) - 30.0;
            let fall = (self.frame % 60) as f32 * 0.8;

            let pos = Pos2::new(
                center.x + x_offset,
                center.y + y_offset - 20.0 + fall,
            );

            let color = confetti_colors[i % confetti_colors.len()];
            let size = 2.0 + (seed.abs() * 2.0);

            // Rotate confetti based on frame
            if i % 2 == 0 {
                painter.circle_filled(pos, size, color);
            } else {
                let rect = Rect::from_center_size(pos, Vec2::new(size * 2.0, size));
                painter.rect_filled(rect, CornerRadius::ZERO, color);
            }
        }
    }

    /// Draw question mark for confused mood
    fn draw_question_mark(&self, painter: &egui::Painter, head_center: Pos2) {
        let qm_center = Pos2::new(head_center.x + 25.0, head_center.y - 25.0);
        let bob = (self.frame as f32 * 0.15).sin() * 2.0;
        let qm_pos = Pos2::new(qm_center.x, qm_center.y + bob);

        // Question mark background bubble
        painter.circle_filled(qm_pos, 12.0, Color32::from_rgba_unmultiplied(255, 255, 255, 200));
        painter.circle_stroke(qm_pos, 12.0, Stroke::new(1.5, Color32::from_rgb(200, 150, 200)));

        // Draw "?" using simple shapes
        let q_color = Color32::from_rgb(150, 100, 180);

        // Top curve of ?
        painter.circle_stroke(
            Pos2::new(qm_pos.x, qm_pos.y - 3.0),
            4.0,
            Stroke::new(2.5, q_color),
        );

        // Stem
        painter.line_segment(
            [Pos2::new(qm_pos.x + 4.0, qm_pos.y - 1.0), Pos2::new(qm_pos.x, qm_pos.y + 2.0)],
            Stroke::new(2.5, q_color),
        );

        // Dot
        painter.circle_filled(Pos2::new(qm_pos.x, qm_pos.y + 6.0), 1.5, q_color);
    }

    /// Set typing mood (called when user is typing)
    /// Reserved for future use: integrate with input handling to show typing animation
    #[allow(dead_code)]
    pub fn on_typing(&mut self) {
        self.activity();
        if self.mood == MascotMood::Idle || self.mood == MascotMood::Sleepy {
            self.set_mood(MascotMood::Typing);
        }
    }

    /// Stop typing mood
    /// Reserved for future use: integrate with input handling to show typing animation
    #[allow(dead_code)]
    pub fn on_typing_done(&mut self) {
        if self.mood == MascotMood::Typing {
            self.set_mood(MascotMood::Idle);
        }
    }
}
