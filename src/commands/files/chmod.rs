//! chmod command - change file mode/permissions
//! Note: On Windows, this has limited functionality

use std::fs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ChmodCommand;

impl Command for ChmodCommand {
    fn name(&self) -> &'static str {
        "chmod"
    }

    fn description(&self) -> &'static str {
        "Change file mode bits"
    }

    fn usage(&self) -> &'static str {
        "chmod [-R] mode file..."
    }

    fn extended_help(&self) -> String {
        r#"chmod - Change file mode/permissions

USAGE:
  chmod [OPTIONS] <mode> <file...>

OPTIONS:
  -R, -r, --recursive    Change files and directories recursively

DESCRIPTION:
  Change the file mode bits of each given file.
  ⚠️  Note: Windows has limited permission support!

MODES (Numeric - Octal):
  755    rwxr-xr-x   Owner: all, Others: read+execute
  644    rw-r--r--   Owner: read+write, Others: read
  700    rwx------   Owner only, full access
  600    rw-------   Owner only, read+write
  444    r--r--r--   Read-only for everyone

MODES (Symbolic):
  +w     Add write permission
  -w     Remove write permission (read-only)
  +x     Add execute permission
  -x     Remove execute permission

EXAMPLES:
  chmod 755 script.sh           Make executable
  chmod 644 document.txt        Standard file perms
  chmod -w important.txt        Make read-only
  chmod +w locked.txt           Make writable
  chmod -R 755 project/         Recursive change

WINDOWS LIMITATIONS:
  Only read-only attribute is supported on Windows.
  Full Unix permissions are not available.

RELATED COMMANDS:
  ls -l    Show permissions
  stat     Detailed file info
  chown    Change owner (not available)
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut recursive = false;
        let mut mode: Option<&str> = None;
        let mut files: Vec<&str> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-R" | "-r" | "--recursive" => recursive = true,
                arg if mode.is_none() && !arg.starts_with('-') => mode = Some(arg),
                arg if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        let mode = mode.ok_or_else(|| anyhow::anyhow!("chmod: missing mode operand"))?;

        if files.is_empty() {
            return Err(anyhow::anyhow!("chmod: missing file operand"));
        }

        // Parse mode (basic support for numeric modes)
        let readonly = parse_mode_readonly(mode);

        let mut count = 0;
        for file in files {
            let path = state.resolve_path(file);

            if recursive && path.is_dir() {
                count += chmod_recursive(&path, readonly)?;
            } else {
                set_readonly(&path, readonly)?;
                count += 1;
            }
        }

        // On Windows, show a note about limited support
        Ok(format!("Changed permissions for {} file(s)\n⚠️  Note: Windows has limited permission support (read-only only)", count))
    }
}

fn parse_mode_readonly(mode: &str) -> bool {
    // Numeric mode
    if let Ok(num) = u32::from_str_radix(mode, 8) {
        // If write bit is not set for owner (not 0o200), make readonly
        return num & 0o200 == 0;
    }

    // Symbolic mode (simplified)
    if mode.contains("-w") {
        return true;
    }
    if mode.contains("+w") {
        return false;
    }

    false
}

fn set_readonly(path: &std::path::Path, readonly: bool) -> Result<()> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_readonly(readonly);
    fs::set_permissions(path, perms)?;
    Ok(())
}

fn chmod_recursive(path: &std::path::Path, readonly: bool) -> Result<usize> {
    let mut count = 0;

    for entry in walkdir::WalkDir::new(path) {
        let entry = entry?;
        set_readonly(entry.path(), readonly)?;
        count += 1;
    }

    Ok(count)
}
