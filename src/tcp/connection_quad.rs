use std::net::Ipv4Addr;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct ConnectionQuad {
    source_addr: Ipv4Addr,
    source_port: u16,
    dest_addr: Ipv4Addr,
    dest_port: u16,
}
