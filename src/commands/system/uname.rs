//! uname command - print system information

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct UnameCommand;

impl Command for UnameCommand {
    fn name(&self) -> &'static str {
        "uname"
    }

    fn description(&self) -> &'static str {
        "Print system information"
    }

    fn usage(&self) -> &'static str {
        "uname [-a] [-s] [-n] [-r] [-m]"
    }

    fn extended_help(&self) -> String {
        r#"uname - Print system information

USAGE:
  uname [OPTIONS]

OPTIONS:
  -a, --all             Print all information
  -s, --kernel-name     Print the kernel name
  -n, --nodename        Print the network node hostname
  -r, --kernel-release  Print the kernel release (version)
  -m, --machine         Print the machine hardware name

DESCRIPTION:
  Print certain system information. With no options,
  same as -s (prints kernel name).

EXAMPLES:
  uname                 Kernel name only: Windows_NT
  uname -a              All info: Windows_NT DESKTOP 10.0 x86_64
  uname -r              Version: 10.0.19045
  uname -m              Architecture: x86_64

OUTPUT WITH -a:
  Windows_NT DESKTOP-ABC 10.0.19045 x86_64
  |          |           |          |
  kernel     hostname    version    arch

WHAT EACH SHOWS:
  -s   Operating system type (Windows_NT)
  -n   Computer name (hostname)
  -r   Windows version number
  -m   CPU architecture (x86_64, aarch64)

COMMON USE CASES:
  • Check system type in scripts
  • Verify architecture for builds
  • System documentation
  • Compatibility checking

ARCHITECTURE VALUES:
  x86_64     64-bit Intel/AMD
  aarch64    64-bit ARM
  x86        32-bit Intel/AMD

RELATED COMMANDS:
  hostname   Just the computer name
  neofetch   Detailed system info
  uptime     System uptime
"#
        .to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut show_all = false;
        let mut show_kernel = false;
        let mut show_nodename = false;
        let mut show_release = false;
        let mut show_machine = false;

        for arg in args {
            match arg.as_str() {
                "-a" | "--all" => show_all = true,
                "-s" | "--kernel-name" => show_kernel = true,
                "-n" | "--nodename" => show_nodename = true,
                "-r" | "--kernel-release" => show_release = true,
                "-m" | "--machine" => show_machine = true,
                "-h" | "--help" => {
                    return Ok("Usage: uname [OPTIONS]\n\
                        Options:\n  \
                        -a    Print all information\n  \
                        -s    Print kernel name\n  \
                        -n    Print network node hostname\n  \
                        -r    Print kernel release\n  \
                        -m    Print machine hardware name"
                        .to_string());
                }
                _ => {}
            }
        }

        // Default to kernel name if no options
        if !show_all && !show_kernel && !show_nodename && !show_release && !show_machine {
            show_kernel = true;
        }

        let kernel_name = "Windows_NT";
        let nodename = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string());
        let machine = std::env::consts::ARCH;

        // Get Windows version
        let release = get_windows_version();

        let mut parts = Vec::new();

        if show_all || show_kernel {
            parts.push(kernel_name.to_string());
        }
        if show_all || show_nodename {
            parts.push(nodename);
        }
        if show_all || show_release {
            parts.push(release);
        }
        if show_all || show_machine {
            parts.push(machine.to_string());
        }

        Ok(parts.join(" "))
    }
}

fn get_windows_version() -> String {
    let output = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "(Get-CimInstance Win32_OperatingSystem).Version",
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => "10.0".to_string(),
    }
}
