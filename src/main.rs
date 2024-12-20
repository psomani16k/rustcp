fn main() {
    let vnic = tun_tap::Iface::new("mera_tun", tun_tap::Mode::Tun).expect("Did not create tun");
    let mut buf = [0u8; 1600];
    loop {
        let nbytes = vnic.recv(&mut buf).unwrap();
        eprintln!("recieved {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }
}
