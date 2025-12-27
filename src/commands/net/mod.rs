//! Network commands
//!
//! curl, wget, ping, netstat, traceroute, nslookup, host, ifconfig

mod curl;
mod wget;
mod ping;
mod netstat;
mod traceroute;
mod nslookup;
mod host;
mod ifconfig;

pub use curl::CurlCommand;
pub use wget::WgetCommand;
pub use ping::PingCommand;
pub use netstat::NetstatCommand;
pub use traceroute::TracerouteCommand;
pub use nslookup::NslookupCommand;
pub use host::HostCommand;
pub use ifconfig::IfconfigCommand;
