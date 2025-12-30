//! tar command - archive utility

use anyhow::Result;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use tar::{Archive, Builder};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TarCommand;

impl Command for TarCommand {
    fn name(&self) -> &'static str {
        "tar"
    }

    fn description(&self) -> &'static str {
        "Archive utility"
    }

    fn usage(&self) -> &'static str {
        "tar [-cxtvz] [-f archive] [files...]"
    }

    fn extended_help(&self) -> String {
        r#"tar - Archive utility (tape archive)

USAGE:
  tar [OPTIONS] -f <archive> [files...]

OPTIONS:
  -c    Create a new archive
  -x    Extract files from archive
  -t    List contents of archive
  -v    Verbose mode (show files)
  -z    Use gzip compression (.tar.gz / .tgz)
  -f    Archive file name (required!)

DESCRIPTION:
  Tar bundles files into a single archive. Often combined
  with gzip compression for .tar.gz files.

================== COMMON RECIPES ==================

CREATE ARCHIVES:
  tar -cvf archive.tar folder/        Create .tar
  tar -czvf archive.tar.gz folder/    Create .tar.gz
  tar -czvf backup.tgz *.txt          Compress txt files

EXTRACT ARCHIVES:
  tar -xvf archive.tar                Extract .tar
  tar -xzvf archive.tar.gz            Extract .tar.gz
  tar -xzvf archive.tgz               Extract .tgz

LIST CONTENTS (without extracting):
  tar -tvf archive.tar                List .tar contents
  tar -tzvf archive.tar.gz            List .tar.gz contents

================== REMEMBER THE FLAGS ==================

MEMORIZE: "create/extract + verbose + zipped + file"

  -c = Create       (make archive)
  -x = eXtract      (unpack archive)
  -t = lisT         (show contents)
  -v = Verbose      (show progress)
  -z = gZip         (compress/decompress)
  -f = File         (archive filename)

COMMON COMBINATIONS:
  -cvf    Create Verbose File
  -xvf    eXtract Verbose File
  -czvf   Create gZip Verbose File
  -xzvf   eXtract gZip Verbose File

================== FILE EXTENSIONS ==================

  .tar        Uncompressed archive
  .tar.gz     Gzip compressed (most common)
  .tgz        Same as .tar.gz (shorthand)
  .tar.bz2    Bzip2 compressed

COMMON MISTAKES:
  tar archive.tar files     WRONG (missing -f)
  tar -cvf files            WRONG (no archive name)
  tar -xf folder/           WRONG (folder isn't archive)

EXAMPLES:
  # Backup a project
  tar -czvf project-backup.tar.gz project/

  # Extract to current directory
  tar -xzvf download.tar.gz

  # Check what's in an archive first
  tar -tzvf mysterious.tar.gz

RELATED COMMANDS:
  zip      ZIP format archives
  gzip     Gzip single files
  gunzip   Decompress gzip
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut create = false;
        let mut extract = false;
        let mut list = false;
        let mut verbose = false;
        let mut gzip = false;
        let mut archive_file: Option<String> = None;
        let mut files: Vec<String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];

            if arg.starts_with('-') && !arg.starts_with("--") {
                for c in arg.chars().skip(1) {
                    match c {
                        'c' => create = true,
                        'x' => extract = true,
                        't' => list = true,
                        'v' => verbose = true,
                        'z' => gzip = true,
                        'f' => {
                            if i + 1 < args.len() {
                                i += 1;
                                archive_file = Some(args[i].clone());
                            }
                        }
                        _ => {}
                    }
                }
            } else if arg == "--help" || arg == "-h" {
                return Ok("Usage: tar [OPTIONS] [-f archive] [files...]\n\
                    Options:\n  \
                    -c    Create archive\n  \
                    -x    Extract archive\n  \
                    -t    List archive contents\n  \
                    -v    Verbose mode\n  \
                    -z    Use gzip compression\n  \
                    -f    Archive file name"
                    .to_string());
            } else {
                files.push(arg.clone());
            }
            i += 1;
        }

        let archive_file =
            archive_file.ok_or_else(|| anyhow::anyhow!("tar: no archive specified"))?;
        let archive_path = state.resolve_path(&archive_file);

        if create {
            if files.is_empty() {
                return Err(anyhow::anyhow!("tar: no files to archive"));
            }

            let file = File::create(&archive_path)?;

            if gzip {
                let encoder = GzEncoder::new(file, Compression::default());
                let mut builder = Builder::new(encoder);

                for f in &files {
                    let path = state.resolve_path(f);
                    if path.is_dir() {
                        builder.append_dir_all(f, &path)?;
                    } else {
                        builder.append_path_with_name(&path, f)?;
                    }
                    if verbose {
                        // Would print but we return string
                    }
                }

                builder.finish()?;
            } else {
                let mut builder = Builder::new(file);

                for f in &files {
                    let path = state.resolve_path(f);
                    if path.is_dir() {
                        builder.append_dir_all(f, &path)?;
                    } else {
                        builder.append_path_with_name(&path, f)?;
                    }
                }

                builder.finish()?;
            }

            Ok(format!("Created archive: {}", archive_file))
        } else if extract {
            let file = File::open(&archive_path)?;

            let mut output = Vec::new();

            if gzip || archive_file.ends_with(".gz") || archive_file.ends_with(".tgz") {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);

                for entry in archive.entries()? {
                    let mut entry = entry?;
                    let path = entry.path()?;
                    if verbose {
                        output.push(format!("x {}", path.display()));
                    }
                    entry.unpack_in(state.cwd())?;
                }
            } else {
                let mut archive = Archive::new(file);

                for entry in archive.entries()? {
                    let mut entry = entry?;
                    let path = entry.path()?;
                    if verbose {
                        output.push(format!("x {}", path.display()));
                    }
                    entry.unpack_in(state.cwd())?;
                }
            }

            if output.is_empty() {
                Ok(format!("Extracted: {}", archive_file))
            } else {
                Ok(output.join("\n"))
            }
        } else if list {
            let file = File::open(&archive_path)?;

            let mut output = Vec::new();

            if gzip || archive_file.ends_with(".gz") || archive_file.ends_with(".tgz") {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);

                for entry in archive.entries()? {
                    let entry = entry?;
                    let path = entry.path()?;
                    output.push(path.display().to_string());
                }
            } else {
                let mut archive = Archive::new(file);

                for entry in archive.entries()? {
                    let entry = entry?;
                    let path = entry.path()?;
                    output.push(path.display().to_string());
                }
            }

            Ok(output.join("\n"))
        } else {
            Err(anyhow::anyhow!("tar: must specify -c, -x, or -t"))
        }
    }
}
