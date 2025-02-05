use std::{collections::VecDeque, io::Write, net::SocketAddr, sync::Arc, time::Instant};
use tokio::net::UdpSocket;
use bevy::prelude::*;

use crate::common::types::{game_state::GameMessage, protocol::Message};

#[derive(Resource)]
pub struct NetworkResource {
    pub socket: Arc<UdpSocket>,
    pub send_queue: VecDeque<GameMessage>,
    pub last_sent: Instant
}

impl NetworkResource {
    pub fn new(socket: UdpSocket) -> Self {
        NetworkResource {
            socket: Arc::new(socket),
            send_queue: VecDeque::new(),
            last_sent: Instant::now()
        }
    }

    pub fn send(&mut self, message: GameMessage) {
        self.send_queue.push_back(message);
    }
}


pub fn input_connexion() -> (String,SocketAddr) {
    // Obtenir les entrées utilisateur et créer le client
    print!("Entrez votre nom : ");
    std::io::stdout().flush().unwrap();
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();
    
    print!("Entrez l'adresse du serveur (ex: 0.0.0.0:8080) : ");
    std::io::stdout().flush().unwrap();
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();
    
    // Parser l'adresse
    let server_address: SocketAddr = address.trim().parse().expect("Adresse invalide");
    (name, server_address)
}


pub fn handle_network_messages(network: Res<NetworkResource>) {
    let mut buf = vec![0; 1024];
    match network.socket.try_recv(&mut buf) {
        Ok(len) => {
            if let Ok(message) = bincode::deserialize(&buf[..len]) {
                match message {
                    Message::Chat { content } => {
                        info!("Chat: {}", content);
                    },
                    Message::Join { name } => {
                        info!("{} a rejoint le serveur", name);
                    },
                    Message::Leave => {
                        info!("Un joueur a quitté le serveur");
                    }
                }
            }
        },
        Err(_) => {} // Ignore errors
    }
}