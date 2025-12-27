//! unzip command - extract files from zip archive

use std::fs::{self, File};
use std::io::Read;
use anyhow::Result;
use zip::ZipArchive;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct UnzipCommand;

impl Command for UnzipCommand {
    fn name(&self) -> &'static str {
        "unzip"
    }

    fn description(&self) -> &'static str {
        "Extract files from zip archive"
    }

    fn usage(&self) -> &'static str {
        "unzip [-l] [-o] <archive.zip>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut list_only = false;
        let mut overwrite = false;
        let mut archive_file: Option<&String> = None;

        for arg in args {
            match arg.as_str() {
                "-l" | "--list" => list_only = true,
                "-o" | "--overwrite" => overwrite = true,
                "-h" | "--help" => {
                    return Ok("Usage: unzip [OPTIONS] <archive.zip>\n\
                        Options:\n  \
                        -l    List archive contents\n  \
                        -o    Overwrite files without prompting".to_string());
                }
                _ if !arg.starts_with('-') => archive_file = Some(arg),
                _ => {}
            }
        }

        let archive_file = archive_file.ok_or_else(|| anyhow::anyhow!("unzip: no archive specified"))?;
        let archive_path = state.resolve_path(archive_file);

        let file = File::open(&archive_path)
            .map_err(|e| anyhow::anyhow!("unzip: {}: {}", archive_file, e))?;

        let mut archive = ZipArchive::new(file)?;

        if list_only {
            let mut output = Vec::new();
            output.push(format!("{:>10}  {:>19}  {}", "Length", "Date", "Name"));
            output.push("-".repeat(50));

            let mut total_size = 0u64;

            for i in 0..archive.len() {
                let file = archive.by_index(i)?;
                let size = file.size();
                total_size += size;

                let date_str = if let Some(datetime) = file.last_modified() {
                    format!("{:04}-{:02}-{:02} {:02}:{:02}",
                        datetime.year(), datetime.month(), datetime.day(),
                        datetime.hour(), datetime.minute())
                } else {
                    "----.--.-- --:--".to_string()
                };

                output.push(format!("{:>10}  {}  {}", size, date_str, file.name()));
            }

            output.push("-".repeat(50));
            output.push(format!("{:>10}  {} files", total_size, archive.len()));

            Ok(output.join("\n"))
        } else {
            let mut output = Vec::new();
            let dest_dir = state.cwd();

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = dest_dir.join(file.name());

                if file.name().ends_with('/') {
                    fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(parent) = outpath.parent() {
                        if !parent.exists() {
                            fs::create_dir_all(parent)?;
                        }
                    }

                    if outpath.exists() && !overwrite {
                        output.push(format!("  skipping: {} (already exists)", file.name()));
                        continue;
                    }

                    let mut outfile = File::create(&outpath)?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    std::io::Write::write_all(&mut outfile, &buffer)?;

                    output.push(format!("  inflating: {}", file.name()));
                }
            }

            Ok(output.join("\n"))
        }
    }
}
