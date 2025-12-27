//! gunzip command - decompress files

use std::fs::File;
use std::io::{Read, Write};
use anyhow::Result;
use flate2::read::GzDecoder;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct GunzipCommand;

impl Command for GunzipCommand {
    fn name(&self) -> &'static str {
        "gunzip"
    }

    fn description(&self) -> &'static str {
        "Decompress files"
    }

    fn usage(&self) -> &'static str {
        "gunzip [-k] [-f] <file.gz> [file2.gz...]"
    }

    fn extended_help(&self) -> String {
        r#"gunzip - Decompress gzip files

USAGE:
  gunzip [OPTIONS] <file.gz> [file2.gz...]

OPTIONS:
  -k, --keep     Keep the .gz file (don't delete)
  -f, --force    Force decompression (overwrite)

DESCRIPTION:
  Decompress files that were compressed with gzip.
  Removes .gz extension and restores original file.

EXAMPLES:
  gunzip file.txt.gz         Decompress to file.txt
  gunzip -k archive.gz       Decompress, keep .gz
  gunzip *.gz                Decompress all .gz files
  gunzip -f data.gz          Overwrite existing file

IMPORTANT BEHAVIORS:
  1. The .gz file is DELETED by default!
     Use -k to keep it.

  2. File must end in .gz:
     gunzip file.txt  (Error: unknown suffix)

  3. Won't overwrite existing files:
     Use -f to force overwrite

WHAT IT DOES:
  data.log.gz  ->  data.log
  report.txt.gz -> report.txt

ALTERNATIVE COMMANDS:
  gzip -d file.gz      Same as gunzip
  zcat file.gz         View without extracting

FOR .TAR.GZ FILES:
  Don't use gunzip alone! Use tar instead:
  tar -xzvf archive.tar.gz

  gunzip only removes gzip compression,
  leaving you with a .tar file.

COMMON USE CASES:
  • Decompress downloaded files
  • Restore compressed logs
  • Unpack gzip'd data

RELATED COMMANDS:
  gzip     Compress files
  tar      Extract .tar.gz archives
  zcat     View compressed file
  unzip    Extract ZIP archives
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut keep = false;
        let mut force = false;
        let mut files: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-k" | "--keep" => keep = true,
                "-f" | "--force" => force = true,
                "-h" | "--help" => {
                    return Ok("Usage: gunzip [OPTIONS] <file.gz> [file2.gz...]\n\
                        Options:\n  \
                        -k    Keep original file\n  \
                        -f    Force decompression".to_string());
                }
                _ if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        if files.is_empty() {
            return Err(anyhow::anyhow!("gunzip: no files specified"));
        }

        let mut output = Vec::new();

        for file in files {
            let input_path = state.resolve_path(file);

            if !file.ends_with(".gz") {
                output.push(format!("gunzip: {}: unknown suffix -- ignored", file));
                continue;
            }

            let output_name = file.trim_end_matches(".gz");
            let output_path = state.resolve_path(output_name);

            if output_path.exists() && !force {
                output.push(format!("gunzip: {} already exists", output_name));
                continue;
            }

            let input_file = File::open(&input_path)
                .map_err(|e| anyhow::anyhow!("gunzip: {}: {}", file, e))?;

            let mut decoder = GzDecoder::new(input_file);
            let mut buffer = Vec::new();
            decoder.read_to_end(&mut buffer)?;

            let mut output_file = File::create(&output_path)?;
            output_file.write_all(&buffer)?;

            if !keep {
                std::fs::remove_file(&input_path)?;
            }

            output.push(format!("{} -> {}", file, output_name));
        }

        Ok(output.join("\n"))
    }
}
