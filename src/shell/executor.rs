//! Command execution
//!
//! Routes commands to appropriate handlers and executes them.

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;

use anyhow::{Result, anyhow};

use crate::commands::registry::CommandRegistry;
use crate::commands::ai::handle_ai_chat_with_context;
use crate::terminal::state::TerminalState;
use super::parser::{parse_command_line, ParsedCommand, RedirectType};

/// Command executor
pub struct Executor {
    /// Registry of built-in commands
    registry: CommandRegistry,
}

impl Executor {
    /// Create a new executor
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::new(),
        }
    }

    /// Execute a command line
    pub fn execute(&self, input: &str, state: &mut TerminalState) -> Result<String> {
        self.execute_with_history(input, state, None)
    }

    /// Execute a command line with optional command history for AI context
    pub fn execute_with_history(&self, input: &str, state: &mut TerminalState, history: Option<&[String]>) -> Result<String> {
        let input = input.trim();
        if input.is_empty() {
            return Ok(String::new());
        }

        // Check for Python mode: ! code !
        if input.starts_with('!') && input.ends_with('!') && input.len() > 2 {
            let python_code = &input[1..input.len()-1].trim();
            return self.execute_python(python_code);
        }

        // Check for AI chat mode: # prompt
        if input.starts_with('#') {
            return Ok(handle_ai_chat_with_context(input, state, history));
        }

        // Easter eggs 🥚
        if let Some(easter_egg) = self.check_easter_eggs(input) {
            return Ok(easter_egg);
        }

        // Parse the command line
        let pipeline = parse_command_line(input)
            .map_err(|e| anyhow!("{}", e))?;

        // Handle pipelines
        if !pipeline.is_single() {
            return self.execute_native_pipeline(&pipeline, input, state);
        }

        let cmd = pipeline.first().ok_or_else(|| anyhow!("Empty command"))?;

        // Execute single command with redirections
        self.execute_single_command(cmd, None, state)
    }

    /// Execute a single command with redirections
    fn execute_single_command(&self, cmd: &ParsedCommand, stdin_input: Option<&str>, state: &mut TerminalState) -> Result<String> {
        // Handle input redirection
        let stdin = if let Some(input_redir) = cmd.redirections.iter().find(|r| r.redirect_type == RedirectType::Input) {
            let path = state.resolve_path(&input_redir.target);
            let mut file = File::open(&path)
                .map_err(|e| anyhow!("{}: {}", input_redir.target, e))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| anyhow!("{}: {}", input_redir.target, e))?;
            Some(content)
        } else {
            stdin_input.map(|s| s.to_string())
        };

        // Check for --help or -h flag
        let wants_help = cmd.args.iter().any(|a| a == "--help" || a == "-h");

        // Execute the command - native only, no fallbacks
        let output = if self.registry.has_command(&cmd.command) {
            if wants_help {
                // Return extended help instead of executing
                self.registry.get_help(&cmd.command)
            } else {
                // Built-in command - execute directly (instant!)
                self.registry.execute_with_stdin(&cmd.command, &cmd.args, stdin.as_deref(), state)?
            }
        } else if let Some(expanded) = self.expand_git_shortcut(&cmd.command, &cmd.args) {
            // Git shortcut - run git directly
            self.execute_git(&expanded)?
        } else {
            // Unknown command - fail instantly with style
            return Err(anyhow!("🤷 '{}' — never heard of it lol", cmd.command));
        };

        // Handle output redirection
        if let Some(output_redir) = cmd.redirections.iter().find(|r| r.redirect_type == RedirectType::Output) {
            let path = state.resolve_path(&output_redir.target);
            let mut file = File::create(&path)
                .map_err(|e| anyhow!("{}: {}", output_redir.target, e))?;
            writeln!(file, "{}", output)
                .map_err(|e| anyhow!("{}: {}", output_redir.target, e))?;
            return Ok(String::new()); // No output to terminal when redirecting
        }

        // Handle append redirection
        if let Some(append_redir) = cmd.redirections.iter().find(|r| r.redirect_type == RedirectType::Append) {
            let path = state.resolve_path(&append_redir.target);
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .map_err(|e| anyhow!("{}: {}", append_redir.target, e))?;
            writeln!(file, "{}", output)
                .map_err(|e| anyhow!("{}: {}", append_redir.target, e))?;
            return Ok(String::new()); // No output to terminal when redirecting
        }

        Ok(output)
    }

    /// Execute a pipeline of native commands
    fn execute_native_pipeline(&self, pipeline: &super::parser::Pipeline, _original_input: &str, state: &mut TerminalState) -> Result<String> {
        // Check if all commands in the pipeline are built-in
        let all_builtin = pipeline.commands.iter()
            .all(|cmd| self.registry.has_command(&cmd.command));

        if !all_builtin {
            // Find the first non-builtin command
            let unknown = pipeline.commands.iter()
                .find(|cmd| !self.registry.has_command(&cmd.command))
                .map(|cmd| cmd.command.as_str())
                .unwrap_or("unknown");
            return Err(anyhow!("🤷 '{}' — never heard of it lol", unknown));
        }

        // Execute native pipeline
        let mut output: Option<String> = None;

        for (i, cmd) in pipeline.commands.iter().enumerate() {
            let result = self.execute_single_command(cmd, output.as_deref(), state)?;

            if i < pipeline.commands.len() - 1 {
                output = Some(result);
            } else {
                return Ok(result);
            }
        }

        Ok(output.unwrap_or_default())
    }

    /// Expand git shortcuts to full git commands
    fn expand_git_shortcut(&self, command: &str, args: &[String]) -> Option<String> {
        match command {
            "gs" => Some("status".to_string()),
            "gd" => Some("diff".to_string()),
            "gl" => Some("log --oneline -20".to_string()),
            "gp" => Some("push".to_string()),
            "gpl" => Some("pull".to_string()),
            "ga" => {
                if args.is_empty() {
                    Some("add .".to_string())
                } else {
                    Some(format!("add {}", args.join(" ")))
                }
            }
            "gc" => {
                if args.is_empty() {
                    return None; // Need a message
                }
                Some(format!("commit -m \"{}\"", args.join(" ")))
            }
            "gco" => {
                if args.is_empty() {
                    return None;
                }
                Some(format!("checkout {}", args.join(" ")))
            }
            "gb" => Some("branch".to_string()),
            _ => None,
        }
    }

    /// Execute git command directly (fast!)
    fn execute_git(&self, git_args: &str) -> Result<String> {
        let args: Vec<&str> = git_args.split_whitespace().collect();

        let output = Command::new("git")
            .args(&args)
            .output()
            .map_err(|e| anyhow!("git: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && !stderr.is_empty() {
            return Err(anyhow!("{}", stderr.trim()));
        }

        Ok(stdout.trim().to_string())
    }

    /// Execute Python code directly: ! print("hello") !
    fn execute_python(&self, code: &str) -> Result<String> {
        let output = Command::new("python")
            .args(["-c", code])
            .output()
            .map_err(|e| anyhow!("🐍 Python error: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Err(anyhow!("🐍 {}", stderr.trim()));
        }

        Ok(format!("🐍 {}", stdout.trim()))
    }

    /// Check for easter eggs 🥚
    fn check_easter_eggs(&self, input: &str) -> Option<String> {
        let input_lower = input.to_lowercase();

        match input_lower.as_str() {
            "hello" | "hi" | "hey" => Some("👋 Hey there! Ready to hack? 💻".to_string()),
            "ping" => Some("🏓 pong!".to_string()),
            "pong" => Some("🏓 ping! (wait, that's my line)".to_string()),
            "42" => Some("🌌 The answer to life, the universe, and everything.".to_string()),
            "sudo" => Some("🔐 Nice try, but I don't do sudo. I trust you already 😎".to_string()),
            "please" => Some("✨ Since you asked nicely... what do you need?".to_string()),
            "thanks" | "thank you" | "thx" => Some("💜 You're welcome! Happy hacking!".to_string()),
            "vim" => Some("🚪 You can check out any time you like, but you can never leave... jk use :q!".to_string()),
            "emacs" => Some("🎹 M-x butterfly... just kidding, we don't have 8 fingers".to_string()),
            "rust" => Some("🦀 Rust is mass! Blazingly fast, fearlessly concurrent! 🚀".to_string()),
            "coffee" => Some("☕ Here's your mass coffee. Now go build something awesome!".to_string()),
            "mass" => Some("🔥 MASSSSS! You get it! 💪".to_string()),
            "lol" | "lmao" | "haha" => Some("😂 Glad you're having fun!".to_string()),
            "matrix" => Some("💊 Red pill or blue pill? ...we only have purple here 💜".to_string()),
            "hack" | "hacker" => Some("👨‍💻 *types furiously* I'm in. 😎".to_string()),
            "windows" => Some("🪟 We're making Windows bearable, one command at a time!".to_string()),
            "linux" => Some("🐧 Linux vibes on Windows! Best of both worlds 🌍".to_string()),
            "mac" | "macos" => Some("🍎 No Mac needed here! Zaxiom's got you covered 😏".to_string()),
            "help me" => Some("🦸 I'm here to help! Try 'help' for commands, or just ask!".to_string()),
            "i love you" => Some("💜 Aww! I love helping you code! 🥰".to_string()),
            "bye" | "goodbye" | "exit" | "quit" => Some("👋 See ya! (but you're still here... type 'exit' to actually leave)".to_string()),
            "fortune" => {
                let fortunes = [
                    "🔮 A bug in the code is worth two in the documentation.",
                    "🔮 Your code will compile on the first try today. Just kidding.",
                    "🔮 The mass is upon you. Embrace the flow state.",
                    "🔮 A senior dev will review your PR... eventually.",
                    "🔮 Today's mass: Refactor that function you've been avoiding.",
                    "🔮 Your rubber duck is judging your variable names.",
                    "🔮 Stack Overflow will have the exact answer you need.",
                    "🔮 git push --force is in your future. Be careful.",
                ];
                use std::time::{SystemTime, UNIX_EPOCH};
                let idx = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs() as usize % fortunes.len())
                    .unwrap_or(0);
                Some(fortunes[idx].to_string())
            },
            "party" => Some("🎉🎊🥳 PARTY MODE ACTIVATED! 🪩✨🎈".to_string()),
            "gg" => Some("🎮 GG! Well played! 🏆".to_string()),
            "bruh" => Some("😐 bruh moment detected".to_string()),
            "sus" => Some("📮 That's pretty sus ngl 👀".to_string()),
            "yeet" => Some("🚀 YEET! *throws code into production*".to_string()),
            "nice" => Some("😏 nice.".to_string()),
            "69" => Some("😏 nice.".to_string()),
            "420" => Some("🌿 blazingly fast terminal, you might say".to_string()),
            "axolotl" | "zaxiom" | "axiom" => Some("🦎 That's me! Your friendly neighborhood terminal! 💜".to_string()),
            _ => None,
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
