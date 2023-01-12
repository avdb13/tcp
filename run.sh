#!/bin/bash
cargo b --release
doas setcap cap_net_admin=eip ./target/release/tcp
./target/release/tcp &
pid=$!
doas ip addr add 192.168.0.1/24 dev tun0
doas ip link set up dev tun0
wait $pid
