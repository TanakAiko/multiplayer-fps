use std::io::{self, Write};
use std::net::{AddrParseError, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::net::UdpSocket;
use tokio::time::timeout;

use crate::common::network::protocol::Message;

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
    name: String,
}

impl Client {
    fn new(name: String) -> Self {
        Client { name }
    }

    pub async fn run(&self, remote_addr: SocketAddr) -> Result<(), ClientError> {
        // Créer et connecter le socket
        let sock = UdpSocket::bind("0.0.0.0:0").await?;

        // Ajouter un timeout de 15 secondes pour la connexion
        match timeout(Duration::from_secs(15), sock.connect(remote_addr)).await {
            Ok(result) => result?,
            Err(_) => return Err(ClientError::ConnectionTimeout),
        }

        let sock = Arc::new(sock);

        // Envoyer le message de connexion avec timeout
        let join_message = Message::Join {
            name: self.name.clone(),
        };
        let encoded = bincode::serialize(&join_message)?;

        // Timeout pour l'envoi du message de connexion
        match timeout(Duration::from_secs(15), sock.send(&encoded)).await {
            Ok(result) => result?,
            Err(_) => return Err(ClientError::ServerNotResponding),
        };

        let mut buf = vec![0; 1024];
        loop {
            let len = sock.recv(&mut buf).await?;

            if let Ok(message) = bincode::deserialize::<Message>(&buf[..len]) {
                match message {
                    Message::Chat { content } => {
                        println!("{}", content);
                    }
                    Message::Join { name } => {
                        println!("{} a rejoint le serveur", name);
                    }
                    Message::Leave => {
                        println!("Un joueur a quitté le serveur");
                    }
                }
            }
        }
    }

    pub fn start_client() {
        print!("Entrez votre nom : ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        };

        let mut name = String::new();
        match io::stdin().read_line(&mut name) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        };
        let name = name.trim().to_string();

        print!("Entrez l'adresse du serveur (ex: 0.0.0.0:8080) : ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        };

        let mut address = String::new();
        match io::stdin().read_line(&mut address) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        };
        let server_address: Result<SocketAddr, AddrParseError> = address.trim().parse();
        let server_parsed: SocketAddr = match server_address {
            Ok(server) => server,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        println!(
            "Tentative de connexion au serveur {:?} avec le nom {}...",
            server_address, name
        );
        let serv = Self::new(name);
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            if let Err(e) = serv.run(server_parsed).await {
                eprintln!("Erreur de connexion: {}", e);
                return;
            }
        });
    }
}
