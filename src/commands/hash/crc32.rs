//! crc32 command - compute CRC32 checksum

use std::fs;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Crc32Command;

impl Command for Crc32Command {
    fn name(&self) -> &'static str {
        "crc32"
    }

    fn description(&self) -> &'static str {
        "Compute CRC32 checksum"
    }

    fn usage(&self) -> &'static str {
        "crc32 <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"crc32 - Compute CRC32 checksum

USAGE:
  crc32 <file> [file2...]
  command | crc32

DESCRIPTION:
  Compute CRC-32 checksum using the IEEE polynomial.
  CRC32 is a fast error-detection code, NOT for security!

EXAMPLES:
  crc32 file.txt                 Checksum a single file
  crc32 *.zip                    Checksum multiple files
  echo "hello" | crc32           Checksum from stdin

OUTPUT FORMAT:
  <8-char hex checksum>  <filename>
  Example: cbf43926  file.txt

WHY CRC32?
  • Extremely fast computation
  • Used in ZIP, PNG, Ethernet
  • Good for error detection
  • Compact 32-bit output

⚠️  NOT FOR SECURITY:
  CRC32 is NOT cryptographically secure!
  • Easy to create collisions
  • Not suitable for integrity against tampering
  • Only detects accidental corruption

USE CASES:
  • File integrity (accidental corruption)
  • Network packet checking
  • Archive file verification
  • Quick file comparison

FOR SECURITY USE:
  sha256sum   Cryptographic hash
  blake3sum   Fast + secure

RELATED COMMANDS:
  md5sum      Faster than SHA, insecure
  sha256sum   Secure hash
  blake3sum   Fast + secure
"#.to_string()
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("crc32: missing file operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: crc32 <file> [file2...]\n\
                    Compute CRC32 checksum for files.".to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content = fs::read(&path)
                .map_err(|e| anyhow::anyhow!("crc32: {}: {}", arg, e))?;

            let checksum = crc32_compute(&content);
            output.push(format!("{:08x}  {}", checksum, arg));
        }

        Ok(output.join("\n"))
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        if let Some(input) = stdin {
            let checksum = crc32_compute(input.as_bytes());
            Ok(format!("{:08x}  -", checksum))
        } else {
            self.execute(args, state)
        }
    }
}

/// Compute CRC32 using the standard polynomial
fn crc32_compute(data: &[u8]) -> u32 {
    // CRC32 lookup table (IEEE polynomial)
    const CRC32_TABLE: [u32; 256] = {
        let mut table = [0u32; 256];
        let mut i = 0;
        while i < 256 {
            let mut crc = i as u32;
            let mut j = 0;
            while j < 8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
                j += 1;
            }
            table[i] = crc;
            i += 1;
        }
        table
    };

    let mut crc = 0xFFFFFFFF_u32;
    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }
    crc ^ 0xFFFFFFFF
}
