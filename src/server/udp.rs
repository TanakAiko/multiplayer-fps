use crate::common::types::protocol::CommonPlayer;
use crate::{common::types::protocol::Message, server::utils::exeption::ServerError};
use bevy::math::Vec3;
use std::io::Write;
use std::{collections::HashMap, io, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::RwLock};
pub struct Server {
    is_game_started: bool,
    nbr_of_player: u8,
    clients: Arc<RwLock<HashMap<SocketAddr, (String, Vec3)>>>,
    next_position_index: usize,
}

impl Server {
    const POSITIONS: [Vec3; 3] = [
        Vec3::new(-12.0, 1.2, 13.0),
        Vec3::new(-9.0, 1.2, 13.0),
        Vec3::new(-6.0, 1.2, 13.0),
    ];

    fn new(nbr_player: u8) -> Self {
        Server {
            is_game_started: false,
            nbr_of_player: nbr_player,
            clients: Arc::new(RwLock::new(HashMap::new())),
            next_position_index: 0,
        }
    }

    async fn run(&mut self) -> Result<(), ServerError> {
        let default_sock = UdpSocket::bind("0.0.0.0:0")
            .await
            .map_err(|e| ServerError::ConnectionError { source: e })?;

        default_sock.connect("8.8.8.8:80").await.ok();

        let ip = if let Ok(local_addr) = default_sock.local_addr() {
            let ip = local_addr.ip();
            ip
        } else {
            return Err(ServerError::ConnectionError {
                source: io::Error::new(io::ErrorKind::Other, "Could not get local address"),
            });
        };

        let sock = UdpSocket::bind(SocketAddr::new(ip, 8080))
            .await
            .map_err(|e| ServerError::ConnectionError { source: e })?;

        println!("Serveur écoute sur {:?}", sock.local_addr().unwrap());

        let mut buf = vec![0; 1024];

        loop {
            let result = self.handle_client_message(&sock, &mut buf).await;
            match result {
                Ok(()) => continue,
                Err(e) => {
                    eprintln!("Erreur lors du traitement du message: {}", e);
                    // Selon la gravité de l'erreur, on peut décider de continuer ou d'arrêter
                    match e {
                        ServerError::InvalidMessage(_) | ServerError::InvalidClient(_) => continue, // Erreurs non-fatales
                        _ => return Err(e), // Erreurs fatales
                    }
                }
            }
        }
    }

    async fn handle_client_message(
        &mut self,
        sock: &UdpSocket,
        buf: &mut Vec<u8>,
    ) -> Result<(), ServerError> {
        let (len, addr) = sock.recv_from(buf).await?;

        let message = bincode::deserialize::<Message>(&buf[..len])
            .map_err(|_| ServerError::InvalidMessage(addr))?;

        match message {
            Message::Join { name } => {
                self.handle_join(sock, addr, name).await?;
            }
            Message::Leave => todo!(),
            Message::PlayerUpdateSending { position, rotation } => {
                let (name, _) = self.clients.read().await.get(&addr).cloned().unwrap();
                let update = Message::PlayerUpdateReceiving {
                    name,
                    position,
                    rotation,
                };

                let encoded_message = bincode::serialize(&update).unwrap();

                self.broadcast(sock, encoded_message).await?;
            }
            _ => todo!(),
        }

        Ok(())
    }

    // async fn han

    pub async fn handle_join(
        &mut self,
        sock: &UdpSocket,
        addr: SocketAddr,
        name: String,
    ) -> Result<(), ServerError> {
        if self.is_game_started {
            return Ok(());
        }

        // Vérification du nom
        if name.trim().is_empty() {
            return Err(ServerError::InvalidClient("Nom vide non autorisé".into()));
        }

        println!("Nouveau client connecté: {} depuis {}", name, addr);
        if self.next_position_index >= Self::POSITIONS.len() {
            return Err(ServerError::InvalidClient("Server is full".into()));
        }
        let position = Self::POSITIONS[self.next_position_index];
        self.next_position_index += 1;

        self.clients
            .write()
            .await
            .insert(addr, (name.clone(), position));

        let update: Message = Message::Join { name };
        let encoded_message = bincode::serialize(&update).unwrap();
        self.broadcast(sock, encoded_message).await?;

        if self.clients.read().await.len() as u8 == self.nbr_of_player {
            self.is_game_started = true;

            let clients = self.clients.read().await;

            for (player_addr, (player_name, player_pos)) in clients.iter() {
                let player = CommonPlayer {
                    name: player_name.clone(),
                    position: *player_pos,
                };

                let mut enemies = Vec::new();
                for (other_addr, (other_name, other_pos)) in clients.iter() {
                    if other_addr != player_addr {
                        enemies.push(CommonPlayer {
                            name: other_name.clone(),
                            position: *other_pos,
                        });
                    }
                }

                let start_message = Message::StartGame { player, enemies };

                let encoded_start = bincode::serialize(&start_message).unwrap();
                sock.send_to(&encoded_start, player_addr).await?;
            }
        }

        Ok(())
    }

    // pub async fn send_all_enemies(&self, target_name: String) {
    //     // Get read access to the clients HashMap
    //     let clients = self.clients.read().await;

    //     // Collect all client names except the target
    //     let all_names: Vec<String> = clients
    //         .values()
    //         .filter(|name| **name != target_name)
    //         .cloned()
    //         .collect();

    //     // Find the socket address of the target client
    //     if let Some((&target_addr, _)) = clients.iter().find(|(_, name)| **name == target_name) {
    //         // Serialize the names vector
    //         let encoded_message = bincode::serialize(&all_names).unwrap();

    //         if let Err(e) = sock.send_to(&encoded_message, client_addr).await {
    //             eprintln!("Erreur d'envoi à {}: {}", client_addr, e);
    //             // On continue malgré l'erreur pour les autres clients
    //             continue;
    //         }
    //         // Here you would send the encoded_message to target_addr
    //         // The actual sending code depends on your network implementation
    //         // For example:
    //         // self.socket.send_to(&encoded_message, target_addr).unwrap();
    //     }
    // }

    pub async fn broadcast(
        &self,
        sock: &UdpSocket,
        encoded_message: Vec<u8>,
    ) -> Result<(), ServerError> {
        let connected_clients = self.clients.read().await;

        for client_addr in connected_clients.keys() {
            if let Err(e) = sock.send_to(&encoded_message, client_addr).await {
                eprintln!("Erreur d'envoi à {}: {}", client_addr, e);
                // On continue malgré l'erreur pour les autres clients
                continue;
            }
        }

        Ok(())
    }

    pub fn start_server() -> Result<(), ServerError> {
        print!("Entrez le nombre de player : (default 2) ");
        std::io::stdout().flush().unwrap();

        let mut player_count = String::new();

        // Use String to read input
        std::io::stdin().read_line(&mut player_count)?;

        // Stop server if player count is more than 10
        if player_count.trim().parse::<usize>().unwrap_or(2) > 10 {
            return Err(ServerError::InvalidClient(
                "Nombre de joueurs invalide".into(),
            ));
        }

        // Parse the input to an integer
        let player_count: usize = player_count.trim().parse().unwrap_or(2);

        // println!("We need at least 2 players to start the game !");
        let mut serv = Self::new(player_count as u8);
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(serv.run())
    }
}
