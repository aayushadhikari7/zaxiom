//! cowsay command - display a message with ASCII cow art
//!
//! A configurable speaking/thinking cow (or other creature)

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct CowsayCommand;

const COW: &str = r#"        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||"#;

const ROBOT: &str = r#"        \
         \  [◉_◉]
            /|   |\
           | |___|  |
           |_______|"#;

const TUXEDO: &str = r#"        \
         \   .--.
           |o_o |
           |:_/ |
          //   \ \
         (|     | )
        /'\_   _/`\
        \___)=(___/"#;

impl Command for CowsayCommand {
    fn name(&self) -> &'static str {
        "cowsay"
    }

    fn description(&self) -> &'static str {
        "Display a message with ASCII art"
    }

    fn usage(&self) -> &'static str {
        "cowsay [-f cow|robot|tux] <message>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut creature = "cow";
        let mut message_parts = Vec::new();
        let mut skip_next = false;

        for (i, arg) in args.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            if arg == "-h" || arg == "--help" {
                return Ok("Usage: cowsay [-f cow|robot|tux] <message>\n\
                    Display a message with ASCII art creature.\n\n\
                    Options:\n  \
                      -f <creature>  Use specified creature (cow, robot, tux)\n\n\
                    Example:\n  \
                      cowsay Hello World!\n  \
                      cowsay -f robot Beep boop!".to_string());
            }

            if arg == "-f" {
                if let Some(next) = args.get(i + 1) {
                    creature = match next.as_str() {
                        "robot" => "robot",
                        "tux" | "penguin" | "linux" => "tux",
                        _ => "cow",
                    };
                    skip_next = true;
                }
                continue;
            }

            message_parts.push(arg.clone());
        }

        let message = if message_parts.is_empty() {
            "Moo!".to_string()
        } else {
            message_parts.join(" ")
        };

        // Build the speech bubble
        let msg_len = message.len();
        let top = format!(" {} ", "_".repeat(msg_len + 2));
        let bottom = format!(" {} ", "-".repeat(msg_len + 2));
        let middle = format!("< {} >", message);

        let art = match creature {
            "robot" => ROBOT,
            "tux" => TUXEDO,
            _ => COW,
        };

        Ok(format!("{}\n{}\n{}\n{}", top, middle, bottom, art))
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        if let Some(input) = stdin {
            let mut new_args = args.to_vec();
            new_args.push(input.trim().to_string());
            self.execute(&new_args, state)
        } else {
            self.execute(args, state)
        }
    }
}
