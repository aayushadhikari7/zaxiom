//! gzip command - compress files

use std::fs::File;
use std::io::{Read, Write};
use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct GzipCommand;

impl Command for GzipCommand {
    fn name(&self) -> &'static str {
        "gzip"
    }

    fn description(&self) -> &'static str {
        "Compress files"
    }

    fn usage(&self) -> &'static str {
        "gzip [-k] [-f] <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"gzip - Compress files using Lempel-Ziv coding

USAGE:
  gzip [OPTIONS] <file> [file2...]

OPTIONS:
  -k, --keep     Keep original files (don't delete)
  -f, --force    Force compression (overwrite .gz)

DESCRIPTION:
  Compress files individually. Each file becomes file.gz
  and the original is DELETED (unless you use -k).

EXAMPLES:
  gzip file.txt              Compress to file.txt.gz
  gzip -k file.txt           Compress, keep original
  gzip *.log                 Compress all log files
  gzip -f old.txt            Overwrite existing .gz

IMPORTANT BEHAVIORS:
  1. Original file is DELETED by default!
     Use -k to keep it.

  2. Each file compressed separately:
     gzip a.txt b.txt -> a.txt.gz b.txt.gz
     NOT a single archive!

  3. For directories, use tar:
     tar -czvf folder.tar.gz folder/

DECOMPRESSING:
  gunzip file.txt.gz         Decompress
  gzip -d file.txt.gz        Also decompresses

COMMON USE CASES:
  • Compress log files for archiving
  • Reduce file size for transfer
  • Save disk space

COMPRESSION EFFECTIVENESS:
  Great for:  Text, logs, code, documents
  Poor for:   Images, videos, already-compressed files

RELATED COMMANDS:
  gunzip   Decompress gzip files
  zcat     View compressed file contents
  tar      Bundle + compress directories
  zip      ZIP format (keeps files together)
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
                    return Ok("Usage: gzip [OPTIONS] <file> [file2...]\n\
                        Options:\n  \
                        -k    Keep original file\n  \
                        -f    Force compression".to_string());
                }
                _ if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        if files.is_empty() {
            return Err(anyhow::anyhow!("gzip: no files specified"));
        }

        let mut output = Vec::new();

        for file in files {
            let input_path = state.resolve_path(file);
            let output_path = input_path.with_extension(
                format!("{}.gz", input_path.extension().map(|e| e.to_string_lossy()).unwrap_or_default())
            );

            if output_path.exists() && !force {
                output.push(format!("gzip: {}.gz already exists", file));
                continue;
            }

            let mut input_file = File::open(&input_path)
                .map_err(|e| anyhow::anyhow!("gzip: {}: {}", file, e))?;

            let mut buffer = Vec::new();
            input_file.read_to_end(&mut buffer)?;

            let output_file = File::create(&output_path)?;
            let mut encoder = GzEncoder::new(output_file, Compression::default());
            encoder.write_all(&buffer)?;
            encoder.finish()?;

            if !keep {
                std::fs::remove_file(&input_path)?;
            }

            output.push(format!("{} -> {}.gz", file, file));
        }

        Ok(output.join("\n"))
    }
}
