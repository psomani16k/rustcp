use etherparse::IpNumber;

fn main() {
    let vnic = tun_tap::Iface::new("mera_tun", tun_tap::Mode::Tun).expect("Did not create tun");
    let mut buf = [0u8; 1600];
    loop {
        let nbytes = vnic.recv(&mut buf).unwrap();
        // let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        //
        // ignoring any packet that is not a IPv4 header
        if proto != 0x0800 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let protocol = p.protocol();
                if protocol != IpNumber::TCP {
                    println!(
                        "Recieved a non-TCP packet of type: {:?}",
                        protocol.keyword_str()
                    );
                    continue;
                }
                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + p.slice().len()..]) {
                    Ok(t) => {
                        let src = t.source_port();
                        let dest = t.destination_port();
                        println!("{} -> {}, Size: {}", src, dest, t.window_size(),);
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
