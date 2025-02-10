use std::{collections::VecDeque, io::Write, net::{IpAddr, SocketAddr}, sync::Arc, time::Instant};
use tokio::{net::UdpSocket, runtime::Runtime};
use bevy::prelude::*;

use crate::common::{types::game_state::GameMessage, utils::socket_utils::get_local_ip};

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
    let runtime = Runtime::new().unwrap();
    
    // Obtenir les entrées utilisateur et créer le client
    print!("Entrez votre nom : ");
    std::io::stdout().flush().unwrap();
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    let server_address:IpAddr  = runtime.block_on(get_local_ip()).unwrap();
    
    print!("Entrez l'adresse du serveur (defaut: {:?}:8080) : ", server_address);
    std::io::stdout().flush().unwrap();
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();

    // Si l'utilisateur n'entre pas d'adresse, on utilise l'adresse par défaut
    if  address.trim().is_empty() {
        address = server_address.to_string() + ":8080";
    }    

    // Parser l'adresse
    let server_address: SocketAddr = address.trim().parse().expect("Adresse invalide");
    (name, server_address)
}

