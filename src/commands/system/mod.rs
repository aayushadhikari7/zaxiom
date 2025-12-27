//! System commands
//!
//! exit, which, du, df, ps, kill, whoami, hostname, uname, uptime, free, date, cal, id, neofetch
//! printenv, lscpu, history, test, man, theme

mod exit;
mod which;
mod du;
mod df;
mod ps;
mod kill;
mod whoami;
mod hostname;
mod uname;
mod uptime;
mod free;
mod date;
mod cal;
mod id;
mod neofetch;
mod printenv;
mod lscpu;
mod history_cmd;
mod test_cmd;
mod man;
mod theme;

pub use exit::ExitCommand;
pub use which::WhichCommand;
pub use du::DuCommand;
pub use df::DfCommand;
pub use ps::PsCommand;
pub use kill::KillCommand;
pub use whoami::WhoamiCommand;
pub use hostname::HostnameCommand;
pub use uname::UnameCommand;
pub use uptime::UptimeCommand;
pub use free::FreeCommand;
pub use date::DateCommand;
pub use cal::CalCommand;
pub use id::IdCommand;
pub use neofetch::NeofetchCommand;
pub use printenv::PrintenvCommand;
pub use lscpu::LscpuCommand;
pub use history_cmd::HistoryCommand;
pub use test_cmd::TestCommand;
pub use man::ManCommand;
pub use theme::ThemeCommand;
