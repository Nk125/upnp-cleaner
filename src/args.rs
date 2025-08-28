pub use clap::Parser;
use clap::ValueEnum;
use igd_next::PortMappingProtocol;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Protocol {
    TCP,
    UDP,
}

impl From<Protocol> for PortMappingProtocol {
    fn from(proto: Protocol) -> Self {
        match proto {
            Protocol::TCP => PortMappingProtocol::TCP,
            Protocol::UDP => PortMappingProtocol::UDP,
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        PortMappingProtocol::from(*self).fmt(f)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Protocol to unmap
    #[arg(short, long)]
    pub net_protocol: Protocol,

    /// External port to unmap: from 1-65535
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..))]
    pub port: u16,
}
