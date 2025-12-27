//! lscpu command - display CPU architecture information

use sysinfo::System;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct LscpuCommand;

impl Command for LscpuCommand {
    fn name(&self) -> &'static str {
        "lscpu"
    }

    fn description(&self) -> &'static str {
        "Display CPU architecture information"
    }

    fn usage(&self) -> &'static str {
        "lscpu"
    }

    fn execute(&self, _args: &[String], _state: &mut TerminalState) -> Result<String> {
        let sys = System::new_all();

        let mut output = Vec::new();

        // Architecture
        output.push(format!("{:<20} {}", "Architecture:", std::env::consts::ARCH));

        // CPU count
        let cpu_count = sys.cpus().len();
        output.push(format!("{:<20} {}", "CPU(s):", cpu_count));

        // Get CPU info from first CPU
        if let Some(cpu) = sys.cpus().first() {
            output.push(format!("{:<20} {}", "Model name:", cpu.brand()));
            output.push(format!("{:<20} {} MHz", "CPU MHz:", cpu.frequency()));
            output.push(format!("{:<20} {}", "Vendor ID:", cpu.vendor_id()));
        }

        // Physical vs logical cores
        let physical_cores = sys.physical_core_count().unwrap_or(cpu_count);
        output.push(format!("{:<20} {}", "Core(s):", physical_cores));
        output.push(format!("{:<20} {}", "Thread(s) per core:", cpu_count / physical_cores.max(1)));

        // CPU usage
        let total_usage: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / cpu_count as f32;
        output.push(format!("{:<20} {:.1}%", "CPU Usage:", total_usage));

        Ok(output.join("\n"))
    }
}
