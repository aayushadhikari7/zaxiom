//! File operation commands
//!
//! cat, touch, rm, mkdir, cp, mv, ln, stat, file, basename, dirname, realpath
//! chmod, readlink, mktemp, nano, vim, vi, edit

mod cat;
mod touch;
mod rm;
mod mkdir;
mod cp;
mod mv;
mod ln;
mod stat;
mod file_type;
mod basename;
mod dirname;
mod realpath;
mod chmod;
mod readlink;
mod mktemp;
pub mod nano;

pub use cat::CatCommand;
pub use touch::TouchCommand;
pub use rm::RmCommand;
pub use mkdir::MkdirCommand;
pub use cp::CpCommand;
pub use mv::MvCommand;
pub use ln::LnCommand;
pub use stat::StatCommand;
pub use file_type::FileCommand;
pub use basename::BasenameCommand;
pub use dirname::DirnameCommand;
pub use realpath::RealpathCommand;
pub use chmod::ChmodCommand;
pub use readlink::ReadlinkCommand;
pub use mktemp::MktempCommand;
pub use nano::{NanoCommand, VimCommand, ViCommand, EditCommand, EditorState};
