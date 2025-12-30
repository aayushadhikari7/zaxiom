//! base64 command - encode or decode base64

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Base64Command;

impl Command for Base64Command {
    fn name(&self) -> &'static str {
        "base64"
    }

    fn description(&self) -> &'static str {
        "Encode or decode base64"
    }

    fn usage(&self) -> &'static str {
        "base64 [-d] <file>"
    }

    fn extended_help(&self) -> String {
        r#"base64 - Encode or decode base64 data

USAGE:
  base64 [OPTIONS] <file>
  command | base64 [OPTIONS]

OPTIONS:
  -d, --decode    Decode base64 input to binary

DESCRIPTION:
  Base64 encode/decode data and print to output.
  Base64 encodes binary data to ASCII text for safe
  transmission through text-based protocols.

EXAMPLES:
  base64 image.png               Encode file to base64
  base64 -d encoded.txt          Decode base64 to binary
  echo "hello" | base64          Encode string: aGVsbG8K
  echo "aGVsbG8K" | base64 -d    Decode string: hello

HOW BASE64 WORKS:
  • Converts 3 bytes to 4 ASCII characters
  • Uses A-Z, a-z, 0-9, +, / (64 chars)
  • Padding with = if input not multiple of 3
  • Output is ~33% larger than input

COMMON USE CASES:
  • Embed images in HTML/CSS
  • Encode email attachments (MIME)
  • Store binary in JSON/XML
  • Pass binary data in URLs
  • Configuration file secrets

EXAMPLES IN PRACTICE:
  # Embed image in HTML
  base64 logo.png | xargs -I{} echo "data:image/png;base64,{}"

  # Encode a password file
  base64 secrets.json > secrets.b64

NOTE:
  Base64 is ENCODING, not ENCRYPTION!
  Anyone can decode base64. Do not use for secrets.

RELATED COMMANDS:
  xxd        Hex encoding
  uuencode   Unix-to-Unix encoding
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut decode = false;
        let mut file: Option<&String> = None;

        for arg in args {
            match arg.as_str() {
                "-d" | "--decode" => decode = true,
                "-h" | "--help" => {
                    return Ok("Usage: base64 [OPTIONS] <file>\n\
                        Options:\n  \
                        -d    Decode base64 input"
                        .to_string());
                }
                _ if !arg.starts_with('-') => file = Some(arg),
                _ => {}
            }
        }

        let file = file.ok_or_else(|| anyhow::anyhow!("base64: missing file operand"))?;
        let path = state.resolve_path(file);

        if decode {
            let content = fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("base64: {}: {}", file, e))?;

            let decoded = general_purpose::STANDARD
                .decode(content.trim())
                .map_err(|e| anyhow::anyhow!("base64: invalid input: {}", e))?;

            Ok(String::from_utf8_lossy(&decoded).to_string())
        } else {
            let content =
                fs::read(&path).map_err(|e| anyhow::anyhow!("base64: {}: {}", file, e))?;

            Ok(general_purpose::STANDARD.encode(&content))
        }
    }
}
