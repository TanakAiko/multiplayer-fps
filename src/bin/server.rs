use std::{collections::HashMap, io, net::SocketAddr, sync::Arc};
use multiplayer_fps::common::network::protocol::Message;
use tokio::{net::UdpSocket, sync::RwLock};
pub struct Server {
    clients: Arc<RwLock<HashMap<SocketAddr, String>>>,
}

impl Server {
    fn new() -> Self {
        Server {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn run(&self, ip_addr: SocketAddr) -> io::Result<()> {
        let sock = UdpSocket::bind(ip_addr).await?;
        println!("Serveur écoute sur {}", ip_addr);

        let mut buf = vec![0; 1024];

        loop {
            let (len, addr) = sock.recv_from(&mut buf).await?;

            if let Ok(message) = bincode::deserialize::<Message>(&buf[..len]) {
                match message {
                    Message::Join { name } => {
                        println!("Nouveau client connecté: {} depuis {}", name, addr);
                        self.clients.write().await.insert(addr, name.clone());

                        // Informer tous les clients de la nouvelle connexion
                        let connected_clients = self.clients.read().await;
                        let client_list: Vec<String> =
                            connected_clients.values().cloned().collect();

                        println!("Clients connectés: {:?}", client_list);

                        // Envoyer la liste mise à jour à tous les clients
                        let response = format!("Clients connectés: {:?}", client_list);
                        let encoded =
                            bincode::serialize(&Message::Chat { content: response }).unwrap();

                        for client_addr in connected_clients.keys() {
                            sock.send_to(&encoded, client_addr).await?;
                        }
                    }
                    Message::Leave => {
                        if let Some(name) = self.clients.write().await.remove(&addr) {
                            println!("Client déconnecté: {} ({})", name, addr);
                        }
                    }
                    Message::Chat { content } => {
                        if let Some(name) = self.clients.read().await.get(&addr) {
                            println!("Message de {} ({}): {}", name, addr, content);
                        }
                    }
                }
            }
        }
    }

    /// Point d'entrée pour démarrer le serveur
    pub fn start_server() {
        let server_address: SocketAddr = "0.0.0.0:8080".parse().unwrap();

        println!("Serveur démarré | Adresse : {}", server_address);

        let serv = Self::new();

        // Démarrer un runtime tokio pour exécuter la méthode async
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(serv.run(server_address)).unwrap();
    }
}


fn main() {
    Server::start_server();
}