//! sha384sum command - compute SHA384 message digest

use anyhow::Result;
use sha2::{Digest, Sha384};
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Sha384sumCommand;

impl Command for Sha384sumCommand {
    fn name(&self) -> &'static str {
        "sha384sum"
    }

    fn description(&self) -> &'static str {
        "Compute SHA384 message digest"
    }

    fn usage(&self) -> &'static str {
        "sha384sum <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"sha384sum - Compute SHA384 message digest

USAGE:
  sha384sum <file> [file2...]
  command | sha384sum

DESCRIPTION:
  Compute and print SHA-384 (384-bit) checksums.
  SHA-384 is a truncated version of SHA-512 with different
  initial values, providing 192-bit security.

EXAMPLES:
  sha384sum file.txt              Hash a single file
  sha384sum *.iso                 Hash multiple files
  echo "data" | sha384sum         Hash from stdin

OUTPUT FORMAT:
  <96-char hex hash>  <filename>

WHY SHA-384?
  • Strong 192-bit security level
  • Good balance of security and hash length
  • Faster than SHA-512 on 64-bit systems
  • Required by some government standards

SECURITY:
  ✅ Cryptographically secure
  ✅ No known vulnerabilities
  ✅ Suitable for security applications

USE CASES:
  • TLS/SSL certificates
  • Digital signatures
  • Integrity verification
  • Government/compliance requirements

RELATED COMMANDS:
  sha256sum   SHA-256 (shorter, widely used)
  sha512sum   SHA-512 (longer hash)
  blake3sum   BLAKE3 (fastest)
"#
        .to_string()
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("sha384sum: missing file operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: sha384sum <file> [file2...]\n\
                    Compute SHA384 message digest for files."
                    .to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content =
                fs::read(&path).map_err(|e| anyhow::anyhow!("sha384sum: {}: {}", arg, e))?;

            let mut hasher = Sha384::new();
            hasher.update(&content);
            let result = hasher.finalize();

            output.push(format!("{}  {}", hex::encode(result), arg));
        }

        Ok(output.join("\n"))
    }

    fn execute_with_stdin(
        &self,
        args: &[String],
        stdin: Option<&str>,
        state: &mut TerminalState,
    ) -> Result<String> {
        if let Some(input) = stdin {
            let mut hasher = Sha384::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            Ok(format!("{}  -", hex::encode(result)))
        } else {
            self.execute(args, state)
        }
    }
}
