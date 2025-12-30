//! Text utility commands
//!
//! echo, head, tail, wc, sort, uniq, tac, cut, paste, diff, tr, sed, awk, rev, nl, printf
//! xargs, column, strings, split, join, comm

mod awk;
mod column;
mod comm;
mod cut;
mod diff;
mod echo;
mod head;
mod join;
mod nl;
mod paste;
mod printf_cmd;
mod rev;
mod sed;
mod sort;
mod split;
mod strings;
mod tac;
mod tail;
mod tr;
mod uniq;
mod wc;
mod xargs;

pub use awk::AwkCommand;
pub use column::ColumnCommand;
pub use comm::CommCommand;
pub use cut::CutCommand;
pub use diff::DiffCommand;
pub use echo::EchoCommand;
pub use head::HeadCommand;
pub use join::JoinCommand;
pub use nl::NlCommand;
pub use paste::PasteCommand;
pub use printf_cmd::PrintfCommand;
pub use rev::RevCommand;
pub use sed::SedCommand;
pub use sort::SortCommand;
pub use split::SplitCommand;
pub use strings::StringsCommand;
pub use tac::TacCommand;
pub use tail::TailCommand;
pub use tr::TrCommand;
pub use uniq::UniqCommand;
pub use wc::WcCommand;
pub use xargs::XargsCommand;
