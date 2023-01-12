use std::{collections::HashMap, io, net::SocketAddrV4};

mod data;

use data::{Quad, TcpState};
use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, TcpState> = Default::default();

    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf)?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);

        if proto != 0x0800 {
            continue;
        }

        match Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(iph) => {
                let src = iph.source_addr();
                let dst = iph.destination_addr();
                let proto = iph.protocol();

                if proto != 0x06 {
                    continue;
                }

                match TcpHeaderSlice::from_slice(&buf[4 + iph.slice().len()..]) {
                    Ok(tcph) => {
                        let data = 4 + iph.slice().len() + tcph.slice().len();

                        connections
                            .entry(Quad {
                                src: SocketAddrV4::new(src, tcph.source_port()),
                                dst: SocketAddrV4::new(dst, tcph.destination_port()),
                            })
                            .or_default()
                            .on_packet(iph, tcph, &buf[data..]);
                        println!(
                            "{} -> {} ({} bytes, port {}):\n {:x?}",
                            src,
                            dst,
                            tcph.slice().len(),
                            tcph.destination_port(),
                            tcph,
                        );
                    }
                    Err(e) => {
                        println!("ignoring faulty packet: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("ignoring faulty packet: {}", e);
            }
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
