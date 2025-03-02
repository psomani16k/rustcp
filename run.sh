#!/bin/bash
cargo build
stat=$?
if [[ $stat -ne 0 ]]; then
	exit $stat
fi
sudo setcap cap_net_admin=eip target/debug/rustcp
./target/debug/rustcp &
pid=$!
echo $pid
sudo ip addr add 192.168.0.1/24 dev rustcp_tun 
sudo ip link set up dev rustcp_tun
trap "kill $pid" INT TERM
wait $pid
