//! mktemp command - create temporary file or directory

use std::fs::{create_dir, File};
use std::path::PathBuf;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct MktempCommand;

impl Command for MktempCommand {
    fn name(&self) -> &'static str {
        "mktemp"
    }

    fn description(&self) -> &'static str {
        "Create a temporary file or directory"
    }

    fn usage(&self) -> &'static str {
        "mktemp [-d] [-p dir] [template]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut make_dir = false;
        let mut base_dir: Option<&str> = None;
        let mut template: Option<&str> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-d" | "--directory" => make_dir = true,
                "-p" | "--tmpdir" => {
                    if i + 1 < args.len() {
                        base_dir = Some(&args[i + 1]);
                        i += 1;
                    }
                }
                arg if !arg.starts_with('-') => template = Some(arg),
                _ => {}
            }
            i += 1;
        }

        // Determine base directory
        let base = match base_dir {
            Some(dir) => PathBuf::from(dir),
            None => std::env::temp_dir(),
        };

        // Generate unique name
        let prefix = template.unwrap_or(if make_dir { "tmp.dir" } else { "tmp.file" });
        let unique_suffix = generate_random_suffix();
        let name = format!(
            "{}.{}",
            prefix.replace("XXXXXX", &unique_suffix),
            unique_suffix
        );

        let path = base.join(&name);

        if make_dir {
            create_dir(&path)?;
        } else {
            File::create(&path)?;
        }

        Ok(path.display().to_string())
    }
}

fn generate_random_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    // Simple random-ish suffix based on time and process id
    let seed = time.as_nanos() as u64 ^ std::process::id() as u64;

    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut result = String::with_capacity(10);
    let mut n = seed;

    for _ in 0..10 {
        result.push(CHARS[(n % 62) as usize] as char);
        n /= 62;
        n = n.wrapping_mul(1103515245).wrapping_add(12345);
    }

    result
}
