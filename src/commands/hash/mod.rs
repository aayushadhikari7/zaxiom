//! Hash and encoding commands
//!
//! md5sum, sha1sum, sha224sum, sha256sum, sha384sum, sha512sum, blake3sum, crc32, base64, xxd

mod md5sum;
mod sha1sum;
mod sha224sum;
mod sha256sum;
mod sha384sum;
mod sha512sum;
mod blake3sum;
mod crc32;
mod base64cmd;
mod xxd;

pub use md5sum::Md5sumCommand;
pub use sha1sum::Sha1sumCommand;
pub use sha224sum::Sha224sumCommand;
pub use sha256sum::Sha256sumCommand;
pub use sha384sum::Sha384sumCommand;
pub use sha512sum::Sha512sumCommand;
pub use blake3sum::Blake3sumCommand;
pub use crc32::Crc32Command;
pub use base64cmd::Base64Command;
pub use xxd::XxdCommand;
