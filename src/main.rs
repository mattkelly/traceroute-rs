use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::icmp::{IcmpPacket, IcmpTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;

use std::net::UdpSocket;

use std::io::{self, Write};

use std::env;
use std::process;
use std::{thread, time};

fn main() -> std::io::Result<()> {
    // TODO use a real argument parser
    let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(io::stderr(), "USAGE: traceroute-rs <interface>").unwrap();
            process::exit(1);
        }
    };

    let interface_name_match = |iface: &NetworkInterface| iface.name == iface_name;

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_name_match)
        .next()
        .unwrap();

    println!("interface: {:?}", interface);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    loop {
        let socket = UdpSocket::bind("0.0.0.0:33333")?;
        //let address = "192.168.1.1:33434";
        let address = "google.com:33434";

        socket.set_ttl(1)?;

        let buf = [0; 1];
        socket.connect(address).expect("failed to connect");
        socket.send(&buf).expect("failed to send");

        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

                match packet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        println!("Got IPv4 packet!");
                        let header = Ipv4Packet::new(packet.payload());
                        if let Some(header) = header {
                            println!("{:?}", header);
                        } else {
                            println!("Malformed IPv4 packet");
                        }
                    }
                    _ => println!(
                        "Unknown packet: {} > {}; ethertype={:?}",
                        packet.get_source(),
                        packet.get_destination(),
                        packet.get_ethertype(),
                    ),
                }

                /*
                let icmp_type = packet.get_icmp_type();
                //let checksum = packet.get_checksum();
                //println!("icmp_type: {:?}", icmp_type);
                //println!("checksum: {:?}", checksum);
                match packet.get_icmp_type() {
                    IcmpTypes::DestinationUnreachable => {
                        println!("destination unreachable");
                    }
                    IcmpTypes::TimeExceeded => {
                        println!("time exceeded");
                    }
                    _ => {
                        println!("unknown icmp type: {:?}", packet.get_icmp_type());
                    }
                }
                if icmp_type == IcmpTypes::DestinationUnreachable {
                    println!("packet: {:?}", packet);
                }
                */
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
