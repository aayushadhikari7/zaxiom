//! diff command - compare files line by line

use anyhow::Result;
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct DiffCommand;

impl Command for DiffCommand {
    fn name(&self) -> &'static str {
        "diff"
    }

    fn description(&self) -> &'static str {
        "Compare files line by line"
    }

    fn usage(&self) -> &'static str {
        "diff <file1> <file2>"
    }

    fn extended_help(&self) -> String {
        r#"diff - Compare files line by line

USAGE:
  diff [OPTIONS] <file1> <file2>

OPTIONS:
  -u, --unified    Output in unified diff format (like git)

DESCRIPTION:
  Compare two files line by line and show differences.
  No output means the files are identical.

EXAMPLES:
  diff old.txt new.txt           Compare two files
  diff -u old.txt new.txt        Unified format (git-style)
  diff config.bak config.txt     Check config changes

READING THE OUTPUT:

  Normal Format:
  3c3         Line 3 was Changed
  < old text  Content in file1
  ---
  > new text  Content in file2

  5d4         Line 5 was Deleted from file1
  < deleted   Content that was removed

  6a7         Line was Added after line 6
  > added     New content in file2

  Unified Format (-u):
  --- file1       First file
  +++ file2       Second file
  @@ -1,5 +1,6 @@ Location in files
  -removed        Line removed (was in file1)
  +added          Line added (in file2)
   unchanged      Lines with no prefix = same

CHANGE INDICATORS:
  c = Changed   Line was modified
  d = Deleted   Line was removed
  a = Added     Line was inserted

COMMON USE CASES:
  • Compare config file versions
  • Check what changed in code
  • Verify backup integrity
  • Review before overwriting files

INTERPRETING RESULTS:
  No output = Files are identical
  Output    = Shows what's different

QUICK PATTERNS:
  # Check if files differ (quiet)
  diff -q file1 file2

  # Create a patch file
  diff -u old.txt new.txt > changes.patch

RELATED COMMANDS:
  cmp      Compare binary files
  comm     Compare sorted files
  patch    Apply diff patches
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut files: Vec<&String> = Vec::new();
        let mut unified = false;

        for arg in args {
            match arg.as_str() {
                "-u" | "--unified" => unified = true,
                "-h" | "--help" => {
                    return Ok("Usage: diff [OPTIONS] <file1> <file2>\n\
                        Options:\n  \
                        -u    Output unified diff format"
                        .to_string());
                }
                _ if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        if files.len() < 2 {
            return Err(anyhow::anyhow!("diff: missing operand"));
        }

        let path1 = state.resolve_path(files[0]);
        let path2 = state.resolve_path(files[1]);

        let content1 =
            fs::read_to_string(&path1).map_err(|e| anyhow::anyhow!("diff: {}: {}", files[0], e))?;
        let content2 =
            fs::read_to_string(&path2).map_err(|e| anyhow::anyhow!("diff: {}: {}", files[1], e))?;

        let lines1: Vec<&str> = content1.lines().collect();
        let lines2: Vec<&str> = content2.lines().collect();

        if lines1 == lines2 {
            return Ok(String::new()); // Files are identical
        }

        let mut result: Vec<String> = Vec::new();

        if unified {
            result.push(format!("--- {}", files[0]));
            result.push(format!("+++ {}", files[1]));
            result.push("@@ -1 +1 @@".to_string());
        }

        // Simple line-by-line diff
        let max_len = lines1.len().max(lines2.len());

        for i in 0..max_len {
            let line1 = lines1.get(i);
            let line2 = lines2.get(i);

            match (line1, line2) {
                (Some(l1), Some(l2)) if l1 == l2 => {
                    if unified {
                        result.push(format!(" {}", l1));
                    }
                }
                (Some(l1), Some(l2)) => {
                    if unified {
                        result.push(format!("-{}", l1));
                        result.push(format!("+{}", l2));
                    } else {
                        result.push(format!("{}c{}", i + 1, i + 1));
                        result.push(format!("< {}", l1));
                        result.push("---".to_string());
                        result.push(format!("> {}", l2));
                    }
                }
                (Some(l1), None) => {
                    if unified {
                        result.push(format!("-{}", l1));
                    } else {
                        result.push(format!("{}d{}", i + 1, lines2.len()));
                        result.push(format!("< {}", l1));
                    }
                }
                (None, Some(l2)) => {
                    if unified {
                        result.push(format!("+{}", l2));
                    } else {
                        result.push(format!("{}a{}", lines1.len(), i + 1));
                        result.push(format!("> {}", l2));
                    }
                }
                (None, None) => {}
            }
        }

        Ok(result.join("\n"))
    }
}
