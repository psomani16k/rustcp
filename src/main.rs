use etherparse::IpNumber;
mod tcp;

fn main() {
    let mut vnic =
        tun_tap::Iface::new("rustcp_tun", tun_tap::Mode::Tun).expect("Did not create tun");
    let mut buf = [0u8; 1600];

    loop {
        let nbytes = vnic.recv(&mut buf).unwrap();

        let flags = u16::from_be_bytes([buf[0], buf[1]]);

        let proto = u16::from_be_bytes([buf[2], buf[3]]);

        // ignoring any packet that is not a IPv4 header
        if proto != 0x0800 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip_header) => {
                let protocol = ip_header.protocol();

                // ignoring any packet that is not a TCP packet
                if protocol != IpNumber::TCP {
                    println!(
                        "Recieved a non-TCP packet of type: {:?}",
                        protocol.keyword_str()
                    );
                    continue;
                }
                match etherparse::TcpHeaderSlice::from_slice(
                    &buf[4 + ip_header.slice().len()..nbytes],
                ) {
                    Ok(tcp_header) => {
                        // processing TCP packets
                        println!(
                            "Recieved TCP: {}:{} -> {}:{}",
                            ip_header.source_addr(),
                            tcp_header.source_port(),
                            ip_header.destination_addr(),
                            tcp_header.destination_port()
                        );
                    }
                    Err(_) => {
                        println!("Recieved malformed TCP packet");
                    }
                }
            }
            Err(_) => {
                println!("Recieved some weird packet, unable to parse");
            }
        }
    }
}
