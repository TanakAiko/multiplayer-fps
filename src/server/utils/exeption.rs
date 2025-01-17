use thiserror::Error;
use std::io;
use std::net::SocketAddr;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Erreur d'entrée/sortie: {0}")]
    Io(#[from] io::Error),
    
    #[error("Erreur de sérialisation: {0}")]
    Serialization(#[from] bincode::Error),
    
    #[error("Erreur de connexion à l'adresse {addr}: {source}")]
    ConnectionError {
        addr: SocketAddr,
        source: io::Error,
    },
    
    #[error("Client invalide: {0}")]
    InvalidClient(String),
    
    #[error("Message invalide reçu de {0}")]
    InvalidMessage(SocketAddr),
}
