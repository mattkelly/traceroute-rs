use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::icmp::{IcmpPacket, IcmpTypes};
use std::net::UdpSocket;

use std::{thread, time};

fn main() -> std::io::Result<()> {
    let interface = &datalink::interfaces()[5];
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
                let packet = IcmpPacket::new(packet).unwrap();

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
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
