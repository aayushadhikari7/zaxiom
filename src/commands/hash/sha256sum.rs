//! sha256sum command - compute SHA256 message digest

use std::fs;
use anyhow::Result;
use sha2::{Sha256, Digest};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Sha256sumCommand;

impl Command for Sha256sumCommand {
    fn name(&self) -> &'static str {
        "sha256sum"
    }

    fn description(&self) -> &'static str {
        "Compute SHA256 message digest"
    }

    fn usage(&self) -> &'static str {
        "sha256sum <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"sha256sum - Compute SHA-256 cryptographic hash

USAGE:
  sha256sum <file> [file2...]

DESCRIPTION:
  Compute and print SHA-256 (256-bit) cryptographic checksums.
  SHA-256 is part of the SHA-2 family and is widely used for
  security applications and file integrity verification.

  ✅ Recommended for security-sensitive applications.

EXAMPLES:
  sha256sum file.txt              Hash a single file
  sha256sum *.zip                 Hash all zip files
  sha256sum a.txt b.txt c.txt     Hash multiple files

OUTPUT FORMAT:
  <64-char hex hash>  <filename>

VERIFYING DOWNLOADS:
  1. Download the file and its .sha256 checksum
  2. Run: sha256sum downloaded-file.zip
  3. Compare the output with the provided checksum

HASH COMPARISON:
  md5sum      128-bit (32 chars) - INSECURE, don't use for security
  sha1sum     160-bit (40 chars) - DEPRECATED
  sha256sum   256-bit (64 chars) - RECOMMENDED ✅
  sha512sum   512-bit (128 chars) - Stronger but slower
  blake3sum   256-bit (64 chars) - Fastest, modern

RELATED COMMANDS:
  md5sum      MD5 hash (insecure)
  sha512sum   SHA-512 hash
  blake3sum   BLAKE3 hash (fastest)
  base64      Base64 encoding
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("sha256sum: missing file operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: sha256sum <file> [file2...]\n\
                    Compute SHA256 message digest for files.".to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content = fs::read(&path)
                .map_err(|e| anyhow::anyhow!("sha256sum: {}: {}", arg, e))?;

            let mut hasher = Sha256::new();
            hasher.update(&content);
            let result = hasher.finalize();

            output.push(format!("{}  {}", hex::encode(result), arg));
        }

        Ok(output.join("\n"))
    }
}
