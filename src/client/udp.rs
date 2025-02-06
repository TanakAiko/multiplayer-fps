use std::io::{self};
use std::net::SocketAddr;
use std::time::Duration;
use thiserror::Error;
use tokio::net::UdpSocket;
use tokio::time::timeout;

use crate::common::types::protocol::Message;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Erreur d'entrée/sortie: {0}")]
    Io(#[from] io::Error),

    #[error("Erreur de sérialisation: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("Délai de connexion dépassé")]
    ConnectionTimeout,

    #[error("Erreur de parsing d'adresse: {0}")]
    AddressParseError(#[from] std::net::AddrParseError),

    #[error("Le serveur n'a pas répondu")]
    ServerNotResponding,
}

pub struct Client {
    pub name: String,
}

impl Client {
    pub fn new(name: String) -> Self {
        Client { name }
    }

    pub async fn connect(&self, remote_addr: SocketAddr) -> Result<UdpSocket, ClientError> {
        // Créer le socket local
        let sock = UdpSocket::bind("0.0.0.0:0").await?;
        sock.connect(remote_addr).await?;

        // Envoyer un message de test et attendre la réponse pour vérifier que le serveur est actif
        let test_message = Message::Join {
            name: self.name.clone(),
        };
        let encoded = bincode::serialize(&test_message)?;

        // Envoyer le message avec timeout
        match timeout(Duration::from_secs(5), sock.send(&encoded)).await {
            Ok(result) => result?,
            Err(_) => return Err(ClientError::ServerNotResponding),
        };

        // Attendre une réponse du serveur
        let mut buffer = [0u8; 1024];
        match timeout(Duration::from_secs(5), sock.recv(&mut buffer)).await {
            Ok(result) => {
                match result {
                    Ok(_) => Ok(sock), // Si on reçoit une réponse, le serveur est actif
                    Err(_) => Err(ClientError::ServerNotResponding),
                }
            }
            Err(_) => Err(ClientError::ServerNotResponding),
        }
    }

    // pub fn start_client() {
    //     print!("Entrez votre nom : ");
    //     match io::stdout().flush() {
    //         Ok(_) => (),
    //         Err(e) => eprintln!("{}", e),
    //     };

    //     let mut name = String::new();
    //     match io::stdin().read_line(&mut name) {
    //         Ok(_) => (),
    //         Err(e) => eprintln!("{}", e),
    //     };
    //     let name = name.trim().to_string();

    //     print!("Entrez l'adresse du serveur (ex: 0.0.0.0:8080) : ");
    //     match io::stdout().flush() {
    //         Ok(_) => (),
    //         Err(e) => eprintln!("{}", e),
    //     };

    //     let mut address = String::new();
    //     match io::stdin().read_line(&mut address) {
    //         Ok(_) => (),
    //         Err(e) => eprintln!("{}", e),
    //     };
    //     let server_address: Result<SocketAddr, AddrParseError> = address.trim().parse();
    //     let server_parsed: SocketAddr = match server_address {
    //         Ok(server) => server,
    //         Err(e) => {
    //             eprintln!("{}", e);
    //             return;
    //         }
    //     };

    //     println!(
    //         "Tentative de connexion au serveur {:?} avec le nom {}...",
    //         server_address, name
    //     );
    //     let serv = Self::new(name);
    //     let runtime = tokio::runtime::Runtime::new().unwrap();
    //     runtime.block_on(async {
    //         if let Err(e) = serv.run(server_parsed).await {
    //             eprintln!("Erreur de connexion: {}", e);
    //             return;
    //         }
    //     });
    // }
}
