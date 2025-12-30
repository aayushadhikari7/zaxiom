//! xxd command - make a hexdump

use anyhow::Result;
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct XxdCommand;

impl Command for XxdCommand {
    fn name(&self) -> &'static str {
        "xxd"
    }

    fn description(&self) -> &'static str {
        "Make a hexdump or reverse"
    }

    fn usage(&self) -> &'static str {
        "xxd [-r] [-c cols] <file>"
    }

    fn extended_help(&self) -> String {
        r#"xxd - Make a hexdump or do the reverse

USAGE:
  xxd [OPTIONS] <file>

OPTIONS:
  -r, --reverse    Reverse operation: convert hex dump to binary
  -c, --cols N     Number of bytes per line (default: 16)

DESCRIPTION:
  Create a hex dump of a given file, or reverse it.
  Useful for examining binary files and editing them.

EXAMPLES:
  xxd binary.dat               Hexdump a file
  xxd -c 8 file.bin            8 bytes per line
  xxd -r dump.txt              Convert hex back to binary
  xxd image.png | head         First lines of hexdump

OUTPUT FORMAT:
  <offset>: <hex bytes>  <ASCII>
  00000000: 4865 6c6c 6f20 576f 726c 6421 0a    Hello World!.

COMMON USE CASES:
  • Inspect binary file contents
  • Debug file formats
  • Edit binary files (xxd -> edit -> xxd -r)
  • Analyze executable headers
  • Examine image metadata

BINARY EDITING WORKFLOW:
  1. xxd file.bin > file.hex
  2. Edit file.hex with text editor
  3. xxd -r file.hex > file_new.bin

OUTPUT COMPONENTS:
  Offset:  Position in file (hex)
  Hex:     Raw bytes in hexadecimal
  ASCII:   Printable character representation

RELATED COMMANDS:
  hexdump    Alternative hex viewer
  od         Octal dump
  file       Identify file type
  strings    Extract text from binary
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut reverse = false;
        let mut cols = 16usize;
        let mut file: Option<&String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-r" | "--reverse" => reverse = true,
                "-c" | "--cols" => {
                    if i + 1 < args.len() {
                        cols = args[i + 1].parse().unwrap_or(16);
                        i += 1;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: xxd [OPTIONS] <file>\n\
                        Options:\n  \
                        -r         Reverse: convert hex dump to binary\n  \
                        -c <cols>  Number of columns (default 16)"
                        .to_string());
                }
                _ if !args[i].starts_with('-') => file = Some(&args[i]),
                _ => {}
            }
            i += 1;
        }

        let file = file.ok_or_else(|| anyhow::anyhow!("xxd: missing file operand"))?;
        let path = state.resolve_path(file);

        if reverse {
            // Reverse hex dump to binary
            let content =
                fs::read_to_string(&path).map_err(|e| anyhow::anyhow!("xxd: {}: {}", file, e))?;

            let mut bytes = Vec::new();
            for line in content.lines() {
                // Skip the address and ASCII parts, just get hex
                if let Some(hex_part) = line.split(':').nth(1) {
                    let hex_str: String = hex_part
                        .chars()
                        .take_while(|c| {
                            *c != ' ' || hex_part.chars().filter(|c| *c == ' ').count() < 2
                        })
                        .filter(|c| c.is_ascii_hexdigit())
                        .collect();

                    for chunk in hex_str.as_bytes().chunks(2) {
                        if chunk.len() == 2 {
                            if let Ok(byte) =
                                u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16)
                            {
                                bytes.push(byte);
                            }
                        }
                    }
                }
            }

            Ok(String::from_utf8_lossy(&bytes).to_string())
        } else {
            // Normal hex dump
            let content = fs::read(&path).map_err(|e| anyhow::anyhow!("xxd: {}: {}", file, e))?;

            let mut output = Vec::new();
            let mut offset = 0;

            for chunk in content.chunks(cols) {
                let hex: Vec<String> = chunk.iter().map(|b| format!("{:02x}", b)).collect();
                let ascii: String = chunk
                    .iter()
                    .map(|&b| {
                        if b.is_ascii_graphic() || b == b' ' {
                            b as char
                        } else {
                            '.'
                        }
                    })
                    .collect();

                // Pad hex to fill columns
                let hex_str = hex.join(" ");
                let padded_hex = format!("{:width$}", hex_str, width = cols * 3 - 1);

                output.push(format!("{:08x}: {}  {}", offset, padded_hex, ascii));
                offset += cols;
            }

            Ok(output.join("\n"))
        }
    }
}
