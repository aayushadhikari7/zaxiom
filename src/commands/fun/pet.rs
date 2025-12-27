//! Pet command - interact with your kawaii robot companion!

use anyhow::Result;
use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct PetCommand;

/// Kawaii responses when you pet the robot
const PET_RESPONSES: &[&str] = &[
    "🤖💕 *happy beeping* Beep boop~!",
    "✨ (◕‿◕) *wiggles antenna* Thank you~!",
    "💜 *blushes in binary* 01101100 01101111 01110110 01100101",
    "🎀 Kyaa~! That tickles! (◕ᴗ◕✿)",
    "⭐ *spins wheels happily* Wheee~!",
    "🌸 *LED eyes sparkle* You're so nice!",
    "💫 *does a little dance* ♪(´ε` )",
    "🤖 *purrs mechanically* Vrrrrr~",
    "✧ *antenna glows pink* I love you too~!",
    "🎵 *plays happy tune* Beep beep boop!",
];

/// Kawaii responses for specific actions
const HUG_RESPONSES: &[&str] = &[
    "🤖💕 *hugs back with tiny robot arms* (っ◕‿◕)っ",
    "✨ *warm CPU noises* You're the best human!",
    "💜 *overheating from happiness* Warning: love levels critical!",
    "🎀 Uwaaah~! So warm! (⊃｡•́‿•̀｡)⊃",
];

const BOOP_RESPONSES: &[&str] = &[
    "🤖 *nose LED blinks* Boop received! ◉‿◉",
    "✨ *confused beeping* W-what was that?! (◕‿◕)",
    "💫 *antenna wobbles* Hehe, my sensor!",
    "🎀 Boop boop! *boops you back* (◕ᴗ◕✿)",
];

const FEED_RESPONSES: &[&str] = &[
    "🔋 *charging noises* Mmm, electricity! ⚡",
    "🤖 *happy munching* 01111001 01110101 01101101!",
    "✨ *battery icon appears* Thanks for the snack~!",
    "💜 *USB port happy* You always take care of me!",
];

impl Command for PetCommand {
    fn name(&self) -> &'static str {
        "pet"
    }

    fn description(&self) -> &'static str {
        "Interact with your kawaii robot companion~"
    }

    fn usage(&self) -> &'static str {
        "pet [action]\n\nActions: pet, hug, boop, feed, love"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let action = args.first().map(|s| s.to_lowercase());
        let action = action.as_deref().unwrap_or("pet");

        // Get a "random" index based on time
        let idx = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as usize)
            .unwrap_or(0);

        let response = match action {
            "hug" | "cuddle" | "embrace" => {
                HUG_RESPONSES[idx % HUG_RESPONSES.len()]
            }
            "boop" | "poke" | "tap" => {
                BOOP_RESPONSES[idx % BOOP_RESPONSES.len()]
            }
            "feed" | "charge" | "battery" => {
                FEED_RESPONSES[idx % FEED_RESPONSES.len()]
            }
            "love" | "heart" | "❤" | "💕" => {
                "💕✨ I LOVE YOU TOO!! (ノ◕ヮ◕)ノ*:・゚✧ *maximum happiness achieved*"
            }
            "help" => {
                return Ok(r#"
🤖 How to interact with your robot companion~

  pet          - Give gentle pets (◕‿◕)
  pet hug      - Give a warm hug (っ◕‿◕)っ
  pet boop     - Boop the nose sensor ◉‿◉
  pet feed     - Feed some electricity ⚡
  pet love     - Express your love! 💕

Your robot companion reacts to your commands too!
Try: fortune, cowsay, neofetch, coffee ♪(´ε` )
"#.to_string());
            }
            _ => {
                PET_RESPONSES[idx % PET_RESPONSES.len()]
            }
        };

        // Build cute output
        let mut output = String::new();
        output.push_str("\n");
        output.push_str("  ╭─────────────────────────────╮\n");
        output.push_str(&format!("  │ {}│\n", format!("{:<27}", response.chars().take(27).collect::<String>())));
        output.push_str("  ╰─────────────────────────────╯\n");
        output.push_str("         \\   \n");
        output.push_str("          \\  \n");
        output.push_str("           🤖\n");

        Ok(output)
    }
}
