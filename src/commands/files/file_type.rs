//! file command - determine file type

use std::fs::{self, File};
use std::io::Read;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct FileCommand;

impl Command for FileCommand {
    fn name(&self) -> &'static str {
        "file"
    }

    fn description(&self) -> &'static str {
        "Determine file type"
    }

    fn usage(&self) -> &'static str {
        "file <file...>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("file: missing file operand"));
        }

        if args[0] == "-h" || args[0] == "--help" {
            return Ok("Usage: file <file...>\n\
                Determine the type of each file.".to_string());
        }

        let mut output = Vec::new();

        for arg in args {
            let path = state.resolve_path(arg);
            let file_type = determine_file_type(&path);
            output.push(format!("{}: {}", arg, file_type));
        }

        Ok(output.join("\n"))
    }
}

fn determine_file_type(path: &std::path::Path) -> String {
    if !path.exists() {
        return "cannot open (No such file or directory)".to_string();
    }

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return "cannot stat".to_string(),
    };

    if metadata.is_dir() {
        return "directory".to_string();
    }

    if metadata.is_symlink() {
        return "symbolic link".to_string();
    }

    // Check file extension first
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
            "txt" | "log" | "md" | "rst" => return "ASCII text".to_string(),
            "rs" => return "Rust source code".to_string(),
            "py" => return "Python script".to_string(),
            "js" => return "JavaScript source".to_string(),
            "ts" => return "TypeScript source".to_string(),
            "json" => return "JSON data".to_string(),
            "toml" => return "TOML configuration".to_string(),
            "yaml" | "yml" => return "YAML data".to_string(),
            "xml" => return "XML document".to_string(),
            "html" | "htm" => return "HTML document".to_string(),
            "css" => return "CSS stylesheet".to_string(),
            "c" => return "C source code".to_string(),
            "cpp" | "cc" | "cxx" => return "C++ source code".to_string(),
            "h" | "hpp" => return "C/C++ header".to_string(),
            "java" => return "Java source code".to_string(),
            "go" => return "Go source code".to_string(),
            "sh" | "bash" => return "shell script".to_string(),
            "bat" | "cmd" => return "batch script".to_string(),
            "ps1" => return "PowerShell script".to_string(),
            "exe" => return "PE32+ executable (Windows)".to_string(),
            "dll" => return "PE32+ DLL (Windows)".to_string(),
            "zip" => return "Zip archive".to_string(),
            "tar" => return "tar archive".to_string(),
            "gz" | "gzip" => return "gzip compressed".to_string(),
            "7z" => return "7-zip archive".to_string(),
            "rar" => return "RAR archive".to_string(),
            "png" => return "PNG image".to_string(),
            "jpg" | "jpeg" => return "JPEG image".to_string(),
            "gif" => return "GIF image".to_string(),
            "svg" => return "SVG image".to_string(),
            "ico" => return "icon".to_string(),
            "pdf" => return "PDF document".to_string(),
            "doc" | "docx" => return "Microsoft Word document".to_string(),
            "xls" | "xlsx" => return "Microsoft Excel spreadsheet".to_string(),
            "mp3" => return "MP3 audio".to_string(),
            "mp4" => return "MP4 video".to_string(),
            "wav" => return "WAV audio".to_string(),
            "avi" => return "AVI video".to_string(),
            "mkv" => return "Matroska video".to_string(),
            _ => {}
        }
    }

    // Try to read magic bytes
    if let Ok(mut file) = File::open(path) {
        let mut magic = [0u8; 8];
        if file.read_exact(&mut magic).is_ok() {
            // Check magic bytes
            if &magic[0..4] == b"\x7fELF" {
                return "ELF executable".to_string();
            }
            if &magic[0..2] == b"MZ" {
                return "PE executable".to_string();
            }
            if &magic[0..4] == b"PK\x03\x04" {
                return "Zip archive".to_string();
            }
            if &magic[0..6] == b"GIF87a" || &magic[0..6] == b"GIF89a" {
                return "GIF image".to_string();
            }
            if &magic[0..8] == b"\x89PNG\r\n\x1a\n" {
                return "PNG image".to_string();
            }
            if &magic[0..2] == b"\xff\xd8" {
                return "JPEG image".to_string();
            }
            if &magic[0..4] == b"%PDF" {
                return "PDF document".to_string();
            }
            if &magic[0..2] == b"\x1f\x8b" {
                return "gzip compressed".to_string();
            }

            // Check if it looks like text
            let is_text = magic.iter().all(|&b| {
                b == 0x09 || b == 0x0a || b == 0x0d || (0x20..=0x7e).contains(&b) || b >= 0x80
            });

            if is_text {
                return "ASCII text".to_string();
            }
        }
    }

    "data".to_string()
}
