//! help command - display available commands beautifully

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct HelpCommand;

impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Display available commands"
    }

    fn usage(&self) -> &'static str {
        "help [command]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if !args.is_empty() {
            // Help for specific command
            return Ok(format!("📖 Help for '{}' - coming soon!", args[0]));
        }

        let help_text = r#"
╭──────────────────────────────────────────────────────────────────╮
│                    🦎 ZAXIOM COMMAND REFERENCE                    │
╰──────────────────────────────────────────────────────────────────╯

  📂 NAVIGATION
  ─────────────────────────────────────────────────────────────────
  ls        List directory contents         cd        Change directory
  pwd       Print working directory         tree      Show directory tree
  clear     Clear the screen

  📄 FILE OPERATIONS
  ─────────────────────────────────────────────────────────────────
  cat       Display file contents           touch     Create empty file
  cp        Copy files/directories          mv        Move/rename files
  rm        Remove files/directories        mkdir     Create directories
  ln        Create links                    stat      File information
  file      Determine file type             basename  Strip directory
  dirname   Get directory path              realpath  Get absolute path

  📝 TEXT PROCESSING
  ─────────────────────────────────────────────────────────────────
  echo      Print text                      head      Show first lines
  tail      Show last lines                 wc        Word/line count
  grep      Search patterns                 sort      Sort lines
  uniq      Remove duplicates               cut       Extract columns
  tr        Translate characters            sed       Stream editor
  awk       Pattern processing              rev       Reverse lines
  nl        Number lines                    printf    Format output
  diff      Compare files                   tac       Reverse file
  paste     Merge lines

  🔍 SEARCH
  ─────────────────────────────────────────────────────────────────
  grep      Search file contents            find      Find files

  💻 SYSTEM
  ─────────────────────────────────────────────────────────────────
  ps        List processes                  kill      Terminate process
  whoami    Current user                    hostname  Show hostname
  uname     System info                     uptime    System uptime
  free      Memory usage                    df        Disk space
  du        Directory size                  date      Show date/time
  cal       Show calendar                   id        User identity
  neofetch  System info (fancy)

  🌐 NETWORK
  ─────────────────────────────────────────────────────────────────
  curl      HTTP requests                   wget      Download files
  ping      Check connectivity              netstat   Network stats

  🔐 HASH & ENCODING
  ─────────────────────────────────────────────────────────────────
  md5sum    MD5 hash                        sha256sum SHA256 hash
  base64    Base64 encode/decode            xxd       Hex dump

  📦 COMPRESSION
  ─────────────────────────────────────────────────────────────────
  tar       Archive files                   zip       Create ZIP
  unzip     Extract ZIP                     gzip      Compress files

  🔧 SHELL UTILITIES
  ─────────────────────────────────────────────────────────────────
  alias     Create aliases                  env       Show variables
  export    Set variables                   sleep     Pause execution
  seq       Generate sequences              expr      Evaluate math
  bc        Calculator                      tee       Split output
  yes       Repeat output                   true      Return success
  false     Return failure

  ⚡ GIT SHORTCUTS
  ─────────────────────────────────────────────────────────────────
  gs → git status    gd → git diff      gl → git log
  gp → git push      gpl → git pull     ga → git add
  gc → git commit    gco → git checkout gb → git branch

  🐍 PYTHON MODE
  ─────────────────────────────────────────────────────────────────
  Wrap code in ! ... ! to execute Python:
  ! print("Hello from Python!") !

  🥚 EASTER EGGS
  ─────────────────────────────────────────────────────────────────
  Try: hello, fortune, coffee, matrix, party, 42, rust...

╭──────────────────────────────────────────────────────────────────╮
│  💜 Type any command to get started! Have fun hacking! 🚀        │
╰──────────────────────────────────────────────────────────────────╯
"#;

        Ok(help_text.to_string())
    }
}
