//! System commands
//!
//! exit, which, du, df, ps, kill, whoami, hostname, uname, uptime, free, date, cal, id, neofetch
//! printenv, lscpu, history, test, man, theme

mod cal;
mod date;
mod df;
mod du;
mod exit;
mod free;
mod history_cmd;
mod hostname;
mod id;
mod kill;
mod lscpu;
mod man;
mod neofetch;
mod printenv;
mod ps;
mod test_cmd;
mod theme;
mod uname;
mod uptime;
mod which;
mod whoami;

pub use cal::CalCommand;
pub use date::DateCommand;
pub use df::DfCommand;
pub use du::DuCommand;
pub use exit::ExitCommand;
pub use free::FreeCommand;
pub use history_cmd::HistoryCommand;
pub use hostname::HostnameCommand;
pub use id::IdCommand;
pub use kill::KillCommand;
pub use lscpu::LscpuCommand;
pub use man::ManCommand;
pub use neofetch::NeofetchCommand;
pub use printenv::PrintenvCommand;
pub use ps::PsCommand;
pub use test_cmd::TestCommand;
pub use theme::ThemeCommand;
pub use uname::UnameCommand;
pub use uptime::UptimeCommand;
pub use which::WhichCommand;
pub use whoami::WhoamiCommand;
