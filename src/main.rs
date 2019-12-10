use smoltcp::socket::SocketSet;
use smoltcp::socket::{IcmpEndpoint, IcmpPacketMetadata, IcmpSocket, IcmpSocketBuffer};
use smoltcp::socket::{UdpPacketMetadata, UdpSocket, UdpSocketBuffer};

use smoltcp::wire::{IpAddress, Ipv4Address};

use smoltcp::wire::{IpEndpoint, IpProtocol, IpRepr, UdpRepr};

use std::{thread, time};

fn main() -> std::io::Result<()> {
    let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 256]);
    let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 256]);
    let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);

    let icmp_rx_buffer = IcmpSocketBuffer::new(vec![IcmpPacketMetadata::EMPTY], vec![0; 256]);
    let icmp_tx_buffer = IcmpSocketBuffer::new(vec![IcmpPacketMetadata::EMPTY], vec![0; 256]);
    let icmp_socket = IcmpSocket::new(icmp_rx_buffer, icmp_tx_buffer);

    const LOCAL_ENDPOINT: IpEndpoint = IpEndpoint {
        addr: IpAddress::Ipv4(Ipv4Address::UNSPECIFIED),
        port: 33333,
    };

    udp_socket
        .bind(LOCAL_ENDPOINT)
        .expect("binding to local endpoint");

    loop {
        let endpoint: IpEndpoint = IpEndpoint {
            addr: IpAddress::v4(8, 8, 8, 8),
            port: 33434,
        };

        udp_socket.set_hop_limit(Some(1));

        let buf = [0; 1];
        udp_socket
            .send_slice(&buf, endpoint)
            .expect("failed to send");

        thread::sleep(time::Duration::from_millis(1000));
    }
}
