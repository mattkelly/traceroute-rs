use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::icmp::{IcmpPacket, IcmpTypes};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let interface = &datalink::interfaces()[1];
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
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let address = "192.168.1.1:33434";

        socket.set_ttl(1)?;

        let buf = [0; 1];
        socket.connect(address).expect("failed to connect");
        socket.send(&buf).expect("failed to send");

        match rx.next() {
            Ok(packet) => {
                let packet = IcmpPacket::new(packet).unwrap();

                let icmp_type = packet.get_icmp_type();
                let checksum = packet.get_checksum();
                println!("icmp_type: {:?}", icmp_type);
                println!("checksum: {:?}", checksum);
                if icmp_type == IcmpTypes::DestinationUnreachable {
                    println!("packet: {:?}", packet);
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }

    Ok(())
}
