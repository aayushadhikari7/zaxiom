//! sha512sum command - compute SHA512 message digest

use std::fs;
use anyhow::Result;
use sha2::{Sha512, Digest};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Sha512sumCommand;

impl Command for Sha512sumCommand {
    fn name(&self) -> &'static str {
        "sha512sum"
    }

    fn description(&self) -> &'static str {
        "Compute SHA512 message digest"
    }

    fn usage(&self) -> &'static str {
        "sha512sum <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"sha512sum - Compute SHA512 message digest

USAGE:
  sha512sum <file> [file2...]
  command | sha512sum

DESCRIPTION:
  Compute and print SHA-512 (512-bit) checksums.
  The strongest member of the SHA-2 family with
  256-bit security level.

EXAMPLES:
  sha512sum file.txt              Hash a single file
  sha512sum *.iso                 Hash multiple files
  echo "data" | sha512sum         Hash from stdin

OUTPUT FORMAT:
  <128-char hex hash>  <filename>

WHY SHA-512?
  • Maximum security (256-bit)
  • Faster than SHA-256 on 64-bit CPUs
  • Longer hash = harder to brute force
  • Quantum resistant (larger margin)

SECURITY:
  ✅ Cryptographically secure
  ✅ No known vulnerabilities
  ✅ 256-bit security level
  ✅ Good for long-term security

USE CASES:
  • High-security applications
  • Long-term archival verification
  • Password hashing (with proper KDF)
  • Critical file integrity

COMPARISON:
  SHA-256: 256-bit hash, 128-bit security
  SHA-384: 384-bit hash, 192-bit security
  SHA-512: 512-bit hash, 256-bit security

RELATED COMMANDS:
  sha256sum   SHA-256 (shorter, widely used)
  sha384sum   SHA-384 (medium length)
  blake3sum   BLAKE3 (faster, equally secure)
"#.to_string()
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("sha512sum: missing file operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: sha512sum <file> [file2...]\n\
                    Compute SHA512 message digest for files.".to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content = fs::read(&path)
                .map_err(|e| anyhow::anyhow!("sha512sum: {}: {}", arg, e))?;

            let mut hasher = Sha512::new();
            hasher.update(&content);
            let result = hasher.finalize();

            output.push(format!("{}  {}", hex::encode(result), arg));
        }

        Ok(output.join("\n"))
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        if let Some(input) = stdin {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            Ok(format!("{}  -", hex::encode(result)))
        } else {
            self.execute(args, state)
        }
    }
}
