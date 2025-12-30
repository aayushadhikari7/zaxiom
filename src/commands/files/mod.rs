//! File operation commands
//!
//! cat, touch, rm, mkdir, cp, mv, ln, stat, file, basename, dirname, realpath
//! chmod, readlink, mktemp, nano, vim, vi, edit

mod basename;
mod cat;
mod chmod;
mod cp;
mod dirname;
mod file_type;
mod ln;
mod mkdir;
mod mktemp;
mod mv;
pub mod nano;
mod readlink;
mod realpath;
mod rm;
mod stat;
mod touch;

pub use basename::BasenameCommand;
pub use cat::CatCommand;
pub use chmod::ChmodCommand;
pub use cp::CpCommand;
pub use dirname::DirnameCommand;
pub use file_type::FileCommand;
pub use ln::LnCommand;
pub use mkdir::MkdirCommand;
pub use mktemp::MktempCommand;
pub use mv::MvCommand;
pub use nano::{EditCommand, EditorState, NanoCommand, ViCommand, VimCommand};
pub use readlink::ReadlinkCommand;
pub use realpath::RealpathCommand;
pub use rm::RmCommand;
pub use stat::StatCommand;
pub use touch::TouchCommand;
