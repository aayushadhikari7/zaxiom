//! zip command - package and compress files

use std::fs::File;
use std::io::{Read, Write};
use anyhow::Result;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ZipCommand;

impl Command for ZipCommand {
    fn name(&self) -> &'static str {
        "zip"
    }

    fn description(&self) -> &'static str {
        "Package and compress files"
    }

    fn usage(&self) -> &'static str {
        "zip [-r] <archive.zip> <files...>"
    }

    fn extended_help(&self) -> String {
        r#"zip - Package and compress files into ZIP format

USAGE:
  zip [OPTIONS] <archive.zip> <files...>

OPTIONS:
  -r, --recursive    Include directory contents recursively

DESCRIPTION:
  Create ZIP archives, the most widely compatible format.
  Works on Windows, Mac, and Linux without extra tools.

EXAMPLES:
  zip archive.zip file1.txt file2.txt    Zip specific files
  zip -r project.zip project/            Zip entire folder
  zip backup.zip *.doc                   Zip all .doc files

WHY ZIP?
  • Universal format - opens everywhere
  • Built into Windows/Mac file explorers
  • Good compression for most files
  • Supports individual file extraction

COMMON USE CASES:
  # Zip a project folder
  zip -r myproject.zip myproject/

  # Zip all images
  zip photos.zip *.jpg *.png

  # Zip for email attachment
  zip -r send-this.zip documents/

WITHOUT -r (directories):
  Files only, directories are skipped!
  Always use -r for folders.

EXTRACTING:
  Use 'unzip' command to extract:
  unzip archive.zip

COMPRESSION LEVELS:
  ZIP uses DEFLATE compression automatically.
  Good for text, code, documents.
  Less effective for already-compressed files
  (images, videos, other archives).

RELATED COMMANDS:
  unzip    Extract ZIP archives
  tar      TAR archives (Unix standard)
  gzip     Single-file compression
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut recursive = false;
        let mut files: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-r" | "--recursive" => recursive = true,
                "-h" | "--help" => {
                    return Ok("Usage: zip [OPTIONS] <archive.zip> <files...>\n\
                        Options:\n  \
                        -r    Recurse into directories".to_string());
                }
                _ if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        if files.len() < 2 {
            return Err(anyhow::anyhow!("zip: need archive name and files"));
        }

        let archive_name = files[0];
        let source_files = &files[1..];

        let archive_path = state.resolve_path(archive_name);
        let file = File::create(&archive_path)?;
        let mut zip = ZipWriter::new(file);

        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        let mut count = 0;

        for source in source_files {
            let source_path = state.resolve_path(source);

            if source_path.is_dir() && recursive {
                for entry in WalkDir::new(&source_path) {
                    let entry = entry?;
                    let path = entry.path();

                    if path.is_file() {
                        let name = path.strip_prefix(state.cwd())
                            .unwrap_or(path)
                            .to_string_lossy();

                        zip.start_file(name.to_string(), options)?;

                        let mut f = File::open(path)?;
                        let mut buffer = Vec::new();
                        f.read_to_end(&mut buffer)?;
                        zip.write_all(&buffer)?;
                        count += 1;
                    }
                }
            } else if source_path.is_file() {
                let name = source_path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| (*source).clone());

                zip.start_file(name, options)?;

                let mut f = File::open(&source_path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
                count += 1;
            }
        }

        zip.finish()?;

        Ok(format!("Created {}: {} files", archive_name, count))
    }
}
