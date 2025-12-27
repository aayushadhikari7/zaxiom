//! wget command - non-interactive network downloader

use std::fs::File;
use std::io::Write;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct WgetCommand;

impl Command for WgetCommand {
    fn name(&self) -> &'static str {
        "wget"
    }

    fn description(&self) -> &'static str {
        "Download files from the web"
    }

    fn usage(&self) -> &'static str {
        "wget [-O output] [-q] <url>"
    }

    fn extended_help(&self) -> String {
        r#"wget - Non-interactive network downloader

USAGE:
  wget [OPTIONS] <url>

OPTIONS:
  -O <file>    Save to specified filename
  -q           Quiet mode (no output)

DESCRIPTION:
  Download files from the web. Simple, non-interactive
  file retrieval over HTTP/HTTPS.

EXAMPLES:
  wget https://example.com/file.zip         Download file
  wget -O myfile.zip https://example.com/x  Save as myfile.zip
  wget -q https://example.com/data.json     Silent download

COMMON USE CASES:
  # Download a file
  wget https://github.com/user/repo/archive/main.zip

  # Download and rename
  wget -O latest.tar.gz https://example.com/v1.2.3.tar.gz

  # Download in script (quiet)
  wget -q https://api.example.com/data.json

OUTPUT FILENAME:
  By default, uses the last part of the URL:
  https://example.com/files/document.pdf -> document.pdf

  Use -O to specify a different name.

COMPARISON WITH CURL:
  wget                 Best for: Simple downloads
  curl                 Best for: API calls, headers, methods

  wget -O file url     Same as: curl -o file url

ERROR HANDLING:
  Returns error if:
  - URL not found (404)
  - Connection failed
  - Server error (5xx)

RELATED COMMANDS:
  curl     Transfer data (more options)
  scp      Secure copy over SSH
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut output_file: Option<String> = None;
        let mut quiet = false;
        let mut url: Option<&String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-O" | "--output-document" => {
                    if i + 1 < args.len() {
                        output_file = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "-q" | "--quiet" => quiet = true,
                "-h" | "--help" => {
                    return Ok("Usage: wget [OPTIONS] <url>\n\
                        Options:\n  \
                        -O <file>    Save to specified file\n  \
                        -q           Quiet mode".to_string());
                }
                _ if !args[i].starts_with('-') => url = Some(&args[i]),
                _ => {}
            }
            i += 1;
        }

        let url = url.ok_or_else(|| anyhow::anyhow!("wget: missing URL"))?;

        // Use reqwest blocking client
        let response = reqwest::blocking::get(url.as_str())
            .map_err(|e| anyhow::anyhow!("wget: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            return Err(anyhow::anyhow!("wget: HTTP {}", status));
        }

        let bytes = response.bytes()
            .map_err(|e| anyhow::anyhow!("wget: {}", e))?;

        // Determine output filename
        let filename = output_file.unwrap_or_else(|| {
            url.split('/').last()
                .filter(|s| !s.is_empty())
                .unwrap_or("index.html")
                .to_string()
        });

        let output_path = state.resolve_path(&filename);
        let mut file = File::create(&output_path)?;
        file.write_all(&bytes)?;

        if quiet {
            Ok(String::new())
        } else {
            Ok(format!("Downloaded: {} ({} bytes)", filename, bytes.len()))
        }
    }
}
