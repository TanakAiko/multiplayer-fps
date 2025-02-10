use std::{io, net::IpAddr};

use tokio::net::UdpSocket;


pub async fn get_local_ip() -> io::Result<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    /* La connexion à 8.8.8.8 (Google DNS) est utilisée pour déterminer quelle 
    interface réseau locale serait utilisée pour atteindre Internet. */
    socket.connect("8.8.8.8:80").await?; // Connexion à Google DNS
    Ok(socket.local_addr()?.ip())
}