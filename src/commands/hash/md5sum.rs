//! md5sum command - compute MD5 message digest

use std::fs;
use anyhow::Result;
use md5::{Md5, Digest};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Md5sumCommand;

impl Command for Md5sumCommand {
    fn name(&self) -> &'static str {
        "md5sum"
    }

    fn description(&self) -> &'static str {
        "Compute MD5 message digest"
    }

    fn usage(&self) -> &'static str {
        "md5sum <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"md5sum - Compute MD5 message digest

USAGE:
  md5sum <file> [file2...]

DESCRIPTION:
  Compute and print MD5 (128-bit) checksums for files.
  MD5 produces a 32-character hexadecimal hash.

  ⚠️  Note: MD5 is cryptographically broken and should NOT be used
  for security purposes. Use SHA-256 or BLAKE3 instead.

EXAMPLES:
  md5sum file.txt              Compute MD5 hash of file.txt
  md5sum *.txt                 Hash all .txt files in current directory
  md5sum file1.txt file2.txt   Hash multiple files

OUTPUT FORMAT:
  <hash>  <filename>
  Example: d41d8cd98f00b204e9800998ecf8427e  empty.txt

RELATED COMMANDS:
  sha256sum   SHA-256 hash (recommended for security)
  sha512sum   SHA-512 hash
  blake3sum   BLAKE3 hash (fastest, modern)
  base64      Base64 encoding
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("md5sum: missing file operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: md5sum <file> [file2...]\n\
                    Compute MD5 message digest for files.".to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content = fs::read(&path)
                .map_err(|e| anyhow::anyhow!("md5sum: {}: {}", arg, e))?;

            let mut hasher = Md5::new();
            hasher.update(&content);
            let result = hasher.finalize();

            output.push(format!("{}  {}", hex::encode(result), arg));
        }

        Ok(output.join("\n"))
    }
}
