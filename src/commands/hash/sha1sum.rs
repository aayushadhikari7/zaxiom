//! sha1sum command - compute SHA1 message digest

use std::fs;
use anyhow::Result;
use sha1::{Sha1, Digest};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Sha1sumCommand;

impl Command for Sha1sumCommand {
    fn name(&self) -> &'static str {
        "sha1sum"
    }

    fn description(&self) -> &'static str {
        "Compute SHA1 message digest"
    }

    fn usage(&self) -> &'static str {
        "sha1sum <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"sha1sum - Compute SHA1 message digest

USAGE:
  sha1sum <file> [file2...]
  command | sha1sum

DESCRIPTION:
  Compute and print SHA-1 (160-bit) checksums.

  ⚠️  SECURITY WARNING: SHA-1 is cryptographically broken!
  Do NOT use for security purposes. Use SHA-256 or BLAKE3.

EXAMPLES:
  sha1sum file.txt              Hash a single file
  sha1sum *.zip                 Hash multiple files
  echo "hello" | sha1sum        Hash from stdin

OUTPUT FORMAT:
  <40-char hex hash>  <filename>
  Example: da39a3ee5e6b4b0d3255bfef95601890afd80709  file.txt

WHEN TO USE SHA-1:
  • Git commit hashes (legacy)
  • Comparing with old checksums
  • Non-security applications
  • Legacy system compatibility

WHEN NOT TO USE:
  • Password hashing
  • Digital signatures
  • Any security application

BETTER ALTERNATIVES:
  sha256sum   SHA-256 (secure, widely supported)
  sha512sum   SHA-512 (secure, longer hash)
  blake3sum   BLAKE3 (fastest, secure)

RELATED COMMANDS:
  md5sum      MD5 (also insecure)
  sha256sum   SHA-256 (recommended)
  blake3sum   BLAKE3 (fastest)
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("sha1sum: missing file operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: sha1sum <file> [file2...]\n\
                    Compute SHA1 message digest for files.".to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content = fs::read(&path)
                .map_err(|e| anyhow::anyhow!("sha1sum: {}: {}", arg, e))?;

            let mut hasher = Sha1::new();
            hasher.update(&content);
            let result = hasher.finalize();

            output.push(format!("{}  {}", hex::encode(result), arg));
        }

        Ok(output.join("\n"))
    }
}
