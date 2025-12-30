//! column command - columnate output

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ColumnCommand;

impl Command for ColumnCommand {
    fn name(&self) -> &'static str {
        "column"
    }

    fn description(&self) -> &'static str {
        "Columnate lists"
    }

    fn usage(&self) -> &'static str {
        "column [-t] [-s separator] [file...]"
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(
        &self,
        args: &[String],
        stdin: Option<&str>,
        state: &mut TerminalState,
    ) -> Result<String> {
        let mut table_mode = false;
        let mut separator = "\t";
        let mut files: Vec<&str> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-t" => table_mode = true,
                "-s" => {
                    if i + 1 < args.len() {
                        separator = &args[i + 1];
                        i += 1;
                    }
                }
                arg if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
            i += 1;
        }

        // Get input
        let input = if let Some(s) = stdin {
            s.to_string()
        } else if !files.is_empty() {
            let mut content = String::new();
            for file in files {
                let path = state.resolve_path(file);
                content.push_str(&std::fs::read_to_string(&path)?);
            }
            content
        } else {
            return Ok(String::new());
        };

        if table_mode {
            // Parse into rows and columns
            let rows: Vec<Vec<&str>> = input
                .lines()
                .map(|line| line.split(separator).collect())
                .collect();

            if rows.is_empty() {
                return Ok(String::new());
            }

            // Find max width for each column
            let max_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
            let mut widths = vec![0usize; max_cols];

            for row in &rows {
                for (i, cell) in row.iter().enumerate() {
                    widths[i] = widths[i].max(cell.len());
                }
            }

            // Format output
            let mut output = String::new();
            for row in rows {
                let formatted: Vec<String> = row
                    .iter()
                    .enumerate()
                    .map(|(i, cell)| {
                        format!(
                            "{:width$}",
                            cell,
                            width = widths.get(i).copied().unwrap_or(0)
                        )
                    })
                    .collect();
                output.push_str(&formatted.join("  "));
                output.push('\n');
            }

            Ok(output.trim_end().to_string())
        } else {
            // Simple column mode - just pass through
            Ok(input)
        }
    }
}
