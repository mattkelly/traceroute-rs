use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::icmp::IcmpPacket;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let address = "8.8.8.8:0";
    socket.connect(address).expect("failed to connect");

    socket.set_ttl(1)?;

    let buf = [0; 10];
    socket.send(&buf).expect("failed to send");

    let interface = &datalink::interfaces()[0];
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    match rx.next() {
        Ok(packet) => {
            let packet = IcmpPacket::new(packet).unwrap();

            println!("packet: {:?}", packet);
        }
        Err(e) => {
            panic!("An error occurred while reading: {}", e);
        }
    }

    Ok(())
}
