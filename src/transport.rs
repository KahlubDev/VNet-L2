use tokio::net::UdpSocket;

pub async fn start_udp_server(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(address).await?;
    println!("VNet-L2 UDP server listening on {}", address);

    let mut buffer = [0u8; 1500];

    loop {
        let (size, peer) = socket.recv_from(&mut buffer).await?;

        println!(
            "Received {} bytes from {}",
            size,
            peer
        );
    }
}