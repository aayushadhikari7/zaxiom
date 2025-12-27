//! blake3sum command - compute BLAKE3 hash
//!
//! BLAKE3 is a modern cryptographic hash function that is faster than
//! MD5, SHA-1, SHA-2, and SHA-3, while being more secure.

use std::fs;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Blake3sumCommand;

impl Command for Blake3sumCommand {
    fn name(&self) -> &'static str {
        "blake3sum"
    }

    fn description(&self) -> &'static str {
        "Compute BLAKE3 cryptographic hash"
    }

    fn usage(&self) -> &'static str {
        "blake3sum <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"blake3sum - Compute BLAKE3 cryptographic hash

USAGE:
  blake3sum <file> [file2...]
  command | blake3sum

DESCRIPTION:
  BLAKE3 is a modern cryptographic hash function that is:
  • FASTER than MD5, SHA-1, SHA-2, and SHA-3
  • More SECURE than MD5 and SHA-1
  • Highly parallelizable

  🚀 Recommended for high-performance hashing!

EXAMPLES:
  blake3sum file.txt           Hash a single file
  blake3sum *.zip              Hash all zip files
  echo "hello" | blake3sum     Hash from stdin

OUTPUT FORMAT:
  <64-char hex hash>  <filename>

SPEED COMPARISON (approximate):
  BLAKE3     ~3 GB/s  ← Fastest! 🏆
  MD5        ~500 MB/s
  SHA-256    ~300 MB/s
  SHA-512    ~400 MB/s

WHY BLAKE3?
  • Designed in 2020 (modern)
  • No known vulnerabilities
  • Extremely fast on modern CPUs
  • Great for large files

RELATED COMMANDS:
  sha256sum   SHA-256 (widely supported)
  md5sum      MD5 (legacy, insecure)
  sha512sum   SHA-512
"#.to_string()
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("blake3sum: missing file operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: blake3sum <file> [file2...]\n\
                    Compute BLAKE3 cryptographic hash for files.\n\
                    BLAKE3 is fast and secure - recommended for new applications.".to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content = fs::read(&path)
                .map_err(|e| anyhow::anyhow!("blake3sum: {}: {}", arg, e))?;

            let hash = blake3::hash(&content);
            output.push(format!("{}  {}", hash.to_hex(), arg));
        }

        Ok(output.join("\n"))
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        if let Some(input) = stdin {
            let hash = blake3::hash(input.as_bytes());
            Ok(format!("{}  -", hash.to_hex()))
        } else {
            self.execute(args, state)
        }
    }
}
