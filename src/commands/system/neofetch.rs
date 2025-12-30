//! neofetch command - display system info with style

use anyhow::Result;
use std::env;
use sysinfo::System;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct NeofetchCommand;

impl Command for NeofetchCommand {
    fn name(&self) -> &'static str {
        "neofetch"
    }

    fn description(&self) -> &'static str {
        "Display system info with style"
    }

    fn usage(&self) -> &'static str {
        "neofetch"
    }

    fn extended_help(&self) -> String {
        r#"neofetch - Display system information with style

USAGE:
  neofetch

DESCRIPTION:
  Shows system information in a beautiful, colorful format.
  Perfect for screenshots and showing off your setup!

INFORMATION DISPLAYED:
  • Username and hostname
  • Operating system and version
  • CPU model and cores
  • Total and used RAM
  • Uptime
  • Shell (Zaxiom!)

EXAMPLES:
  neofetch              Show system info

FUN FACT:
  The original neofetch is a popular tool in the Linux
  community for showing off terminal setups. This version
  is a built-in Zaxiom recreation!

RELATED COMMANDS:
  uname      System name info
  uptime     System uptime
  whoami     Current username
  hostname   Computer name
  free       Memory usage
  lscpu      CPU info
"#
        .to_string()
    }

    fn execute(&self, _args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let username = env::var("USERNAME")
            .or_else(|_| env::var("USER"))
            .unwrap_or_else(|_| "user".to_string());

        let hostname = System::host_name().unwrap_or_else(|| "localhost".to_string());
        let os = System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_default();
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let cpu = sys
            .cpus()
            .first()
            .map(|c: &sysinfo::Cpu| c.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let cpu_cores = sys.cpus().len();
        let total_mem = sys.total_memory() / 1024 / 1024; // MB
        let used_mem = sys.used_memory() / 1024 / 1024;
        let uptime_secs = System::uptime();
        let uptime_hours = uptime_secs / 3600;
        let uptime_mins = (uptime_secs % 3600) / 60;

        let logo = r#"
    ███████╗ █████╗ ██╗  ██╗██╗ ██████╗ ███╗   ███╗
    ╚══███╔╝██╔══██╗╚██╗██╔╝██║██╔═══██╗████╗ ████║
      ███╔╝ ███████║ ╚███╔╝ ██║██║   ██║██╔████╔██║
     ███╔╝  ██╔══██║ ██╔██╗ ██║██║   ██║██║╚██╔╝██║
    ███████╗██║  ██║██╔╝ ██╗██║╚██████╔╝██║ ╚═╝ ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═╝"#;

        let info = format!(
            r#"
{}

    {}@{}
    ─────────────────────────────
    🖥️  OS        {} {}
    🔧 Kernel    {}
    ⏱️  Uptime    {}h {}m
    💻 CPU       {} ({} cores)
    🧠 Memory    {} MB / {} MB
    🦎 Terminal  Zaxiom 0.3.1
    🦀 Built     with Rust + egui

    ██████████████████████████████
"#,
            logo,
            username,
            hostname,
            os,
            os_version,
            kernel,
            uptime_hours,
            uptime_mins,
            used_mem,
            total_mem,
            cpu,
            cpu_cores
        );

        Ok(info)
    }
}
