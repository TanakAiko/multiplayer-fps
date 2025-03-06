use crate::common::types::protocol::CommonPlayer;
use crate::{common::types::protocol::Message, server::utils::exeption::ServerError};
use bevy::math::Vec3;
use std::io::Write;
use std::{collections::HashMap, io, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::RwLock};
pub struct Server {
    is_game_started: bool,
    nbr_of_player: u8,
    all_dead_players: Vec<String>,
    clients: Arc<RwLock<HashMap<SocketAddr, (String, Vec3)>>>,
    next_position_index: usize,
}

impl Server {
    // Complete number possible players
    const POSITIONS: [Vec3; 10] = [
        Vec3::new(-18.0, 0., 13.0),
        Vec3::new(-18.0, 0., -15.0),
        Vec3::new(-18.0, 0., 3.0),
        Vec3::new(-18.0, 0., 0.),
        Vec3::new(-0.0, 0., 13.0),
        Vec3::new(3.0, 0., 13.0),
        Vec3::new(6.0, 0., 13.0),
        Vec3::new(9.0, 0., 13.0),
        Vec3::new(12.0, 0., 13.0),
        Vec3::new(15.0, 0., 13.0),
    ];

    fn new(nbr_player: u8) -> Self {
        Server {
            is_game_started: false,
            nbr_of_player: nbr_player,
            all_dead_players: Vec::new(),
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

        println!("Server listens to {:?}", sock.local_addr().unwrap());

        let mut buf = vec![0; 1024];

        loop {
            let encoded_message = bincode::serialize(&Message::DeadPlayer {
                all_dead_players: self.all_dead_players.clone(),
            })
            .unwrap();
            self.broadcast(&sock, encoded_message).await?;

            let result = self.handle_client_message(&sock, &mut buf).await;
            match result {
                Ok(()) => continue,
                Err(e) => {
                    eprintln!("Error when processing the message: {}", e);
                    match e {
                        ServerError::InvalidMessage(_) | ServerError::InvalidClient(_) => continue, // Erreurs non-fatales
                        _ => return Err(e), // Fatal errors
                    }
                }
            }
        }
    }

    pub fn add_dead_player_if_not_exists(&mut self, new_tab: Vec<String>) {
        for dead_player in new_tab {
            if !self.all_dead_players.contains(&dead_player) {
                self.all_dead_players.push(dead_player);
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
            Message::PlayerUpdateSending {
                position,
                rotation,
                all_dead_players,
            } => {
                self.add_dead_player_if_not_exists(all_dead_players.clone());

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

    pub async fn handle_join(
        &mut self,
        sock: &UdpSocket,
        addr: SocketAddr,
        name: String,
    ) -> Result<(), ServerError> {
        if self.is_game_started {
            return Ok(());
        }

        // Name verification
        if name.trim().is_empty() {
            return Err(ServerError::InvalidClient("Unauthorized empty name".into()));
        }
        // Verification of the length of the name
        // Scope creation to limit customer visibility
        {
            let clients = self.clients.read().await;
            if clients
                .values()
                .any(|(existing_name, _)| existing_name == &name)
            {
                return Err(ServerError::InvalidClient("Name already used".into()));
            }
        }

        println!("New connected customer: {} from {}", name, addr);
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
                    position: Vec3::new(player_pos.x, player_pos.y + 1.5, player_pos.z), // 🔹 Augmente Y de 2
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

    pub async fn broadcast(
        &self,
        sock: &UdpSocket,
        encoded_message: Vec<u8>,
    ) -> Result<(), ServerError> {
        let connected_clients = self.clients.read().await;

        for client_addr in connected_clients.keys() {
            if let Err(e) = sock.send_to(&encoded_message, client_addr).await {
                eprintln!("Sending error to {}: {}", client_addr, e);
                // We continue despite the error for other customers
                continue;
            }
        }

        Ok(())
    }

    pub fn start_server() -> Result<(), ServerError> {
        print!("Enter the number of player: (Default 2)");
        std::io::stdout().flush().unwrap();

        let mut player_count = String::new();

        // Use String to read input
        std::io::stdin().read_line(&mut player_count)?;

        // Stop server if player count is more than 10
        if player_count.trim().parse::<usize>().unwrap_or(2) > 10 {
            return Err(ServerError::InvalidClient(
                "Number of invalid players".into(),
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
