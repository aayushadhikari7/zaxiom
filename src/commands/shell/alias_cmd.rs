//! alias command - define or display aliases

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct AliasCommand;

impl Command for AliasCommand {
    fn name(&self) -> &'static str {
        "alias"
    }

    fn description(&self) -> &'static str {
        "Define or display aliases"
    }

    fn usage(&self) -> &'static str {
        "alias [name[=value] ...]"
    }

    fn extended_help(&self) -> String {
        r#"alias - Create command shortcuts

USAGE:
  alias                    List all aliases
  alias name               Show specific alias
  alias name='command'     Create new alias

DESCRIPTION:
  Create shortcuts for frequently used commands.
  Aliases expand to their values when executed.

EXAMPLES:
  alias                         List all aliases
  alias ll='ls -la'             Create 'ls -la' shortcut
  alias gs='git status'         Git status shortcut
  alias ..='cd ..'              Go up one directory
  alias ll                      Show ll alias definition

CREATING ALIASES:
  alias name='command'          Single command
  alias name='cmd1 && cmd2'     Chained commands
  alias name='cmd -flags'       Command with flags

POPULAR ALIASES:
  alias ll='ls -la'             Detailed listing
  alias la='ls -A'              Show hidden files
  alias ..='cd ..'              Parent directory
  alias ...='cd ../..'          Two directories up
  alias gs='git status'         Git status
  alias gp='git push'           Git push
  alias gc='git commit -m'      Git commit
  alias cls='clear'             Clear screen

REMOVING ALIASES:
  unalias name                  Remove an alias
  unalias -a                    Remove all aliases

TIPS:
  • Quote the value to preserve spaces
  • Use single quotes to prevent expansion
  • Aliases are session-specific (lost on close)

FOR PERMANENT ALIASES:
  Add to your shell config file:
  ~/.bashrc (Bash)
  ~/.zshrc (Zsh)

RELATED COMMANDS:
  unalias   Remove aliases
  type      Check if name is alias
  which     Find command location
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            // List all aliases
            let aliases = state.list_aliases();
            if aliases.is_empty() {
                return Ok(String::new());
            }
            let output: Vec<String> = aliases
                .iter()
                .map(|(name, value)| format!("alias {}='{}'", name, value))
                .collect();
            return Ok(output.join("\n"));
        }

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: alias [name[=value] ...]\n\
                    Without arguments, prints all defined aliases.\n\
                    With name=value, defines an alias.\n\
                    With name only, prints that alias."
                    .to_string());
            }

            if let Some(eq_pos) = arg.find('=') {
                // Define alias: name=value
                let name = &arg[..eq_pos];
                let value = &arg[eq_pos + 1..];
                // Remove surrounding quotes if present
                let value = value.trim_matches('\'').trim_matches('"');
                state.set_alias(name.to_string(), value.to_string());
            } else {
                // Print specific alias
                if let Some(value) = state.get_alias(arg) {
                    return Ok(format!("alias {}='{}'", arg, value));
                } else {
                    return Err(anyhow::anyhow!("alias: {}: not found", arg));
                }
            }
        }

        Ok(String::new())
    }
}
