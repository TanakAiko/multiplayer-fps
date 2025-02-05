use std::{collections::HashMap, io, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::RwLock};
use crate::{common::types::protocol::Message, server::utils::exeption::ServerError};
pub struct Server {
    clients: Arc<RwLock<HashMap<SocketAddr, String>>>,
}

impl Server {
    fn new() -> Self {
        Server {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn run(&self, ip_addr: SocketAddr) -> Result<(), ServerError> {
        let sock = UdpSocket::bind(ip_addr)
            .await
            .map_err(|e| ServerError::ConnectionError {
                addr: ip_addr,
                source: e,
            })?;
            
        println!("Serveur écoute sur {}", ip_addr);
        let mut buf = vec![0; 1024];

        loop {
            let result = self.handle_client_message(&sock, &mut buf).await;
            match result {
                Ok(()) => continue,
                Err(e) => {
                    eprintln!("Erreur lors du traitement du message: {}", e);
                    // Selon la gravité de l'erreur, on peut décider de continuer ou d'arrêter
                    match e {
                        ServerError::InvalidMessage(_) |
                        ServerError::InvalidClient(_) => continue, // Erreurs non-fatales
                        _ => return Err(e), // Erreurs fatales
                    }
                }
            }
        }
    }

    async fn handle_client_message(
        &self,
        sock: &UdpSocket,
        buf: &mut Vec<u8>
    ) -> Result<(), ServerError> {
        let (len, addr) = sock.recv_from(buf).await?;
        
        let message = bincode::deserialize::<Message>(&buf[..len])
            .map_err(|_| ServerError::InvalidMessage(addr))?;

        match message {
            Message::Join { name } => {
                self.handle_join(sock, addr, name).await?;
            }
            Message::Chat { content } => todo!(),
            Message::Leave => todo!(),
        }

        Ok(())
    }

    // async fn han


    pub async fn handle_join(
        &self,
        sock: &UdpSocket,
        addr: SocketAddr,
        name: String,
    ) -> Result<(), ServerError> {
        println!("Nouveau client connecté: {} depuis {}", name, addr);
        
        // Vérification du nom
        if name.trim().is_empty() {
            return Err(ServerError::InvalidClient("Nom vide non autorisé".into()));
        }

        self.clients.write().await.insert(addr, name.clone());

        // Informer tous les clients
        let connected_clients = self.clients.read().await;
        let client_list: Vec<String> = connected_clients.values().cloned().collect();
        
        let response = format!("Clients connectés: {:?}", client_list);
        let encoded = bincode::serialize(&Message::Chat { content: response })?;

        for client_addr in connected_clients.keys() {
            if let Err(e) = sock.send_to(&encoded, client_addr).await {
                eprintln!("Erreur d'envoi à {}: {}", client_addr, e);
                // On continue malgré l'erreur pour les autres clients
                continue;
            }
        }

        Ok(())
    }

pub fn start_server() -> Result<(), ServerError> {
    let server_address: SocketAddr = "0.0.0.0:8080".parse()
        .map_err(|e| ServerError::ConnectionError {
            addr: "0.0.0.0:8080".parse().unwrap(),
            source: io::Error::new(io::ErrorKind::InvalidInput, e),
        })?;

    println!("Serveur démarré | Adresse : {}", server_address);

    let serv = Self::new();
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(serv.run(server_address))
}
}
