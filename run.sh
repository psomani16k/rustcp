#!/bin/sh
cargo build
sudo setcap cap_net_admin=eip /home/parth/Projects/rustcp/target/debug/rustcp
/home/parth/Projects/rustcp/target/debug/rustcp &
pid= $!
# sudo ip addr add 192.168.0.1/24 dev mera_tun
# sudo ip link set up dev mera_tun
wait pid