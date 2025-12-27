//! Text utility commands
//!
//! echo, head, tail, wc, sort, uniq, tac, cut, paste, diff, tr, sed, awk, rev, nl, printf
//! xargs, column, strings, split, join, comm

mod echo;
mod head;
mod tail;
mod wc;
mod sort;
mod uniq;
mod tac;
mod cut;
mod paste;
mod diff;
mod tr;
mod sed;
mod awk;
mod rev;
mod nl;
mod printf_cmd;
mod xargs;
mod column;
mod strings;
mod split;
mod join;
mod comm;

pub use echo::EchoCommand;
pub use head::HeadCommand;
pub use tail::TailCommand;
pub use wc::WcCommand;
pub use sort::SortCommand;
pub use uniq::UniqCommand;
pub use tac::TacCommand;
pub use cut::CutCommand;
pub use paste::PasteCommand;
pub use diff::DiffCommand;
pub use tr::TrCommand;
pub use sed::SedCommand;
pub use awk::AwkCommand;
pub use rev::RevCommand;
pub use nl::NlCommand;
pub use printf_cmd::PrintfCommand;
pub use xargs::XargsCommand;
pub use column::ColumnCommand;
pub use strings::StringsCommand;
pub use split::SplitCommand;
pub use join::JoinCommand;
pub use comm::CommCommand;
