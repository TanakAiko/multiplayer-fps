use std::io::{self, Write,};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use multiplayer_fps::common::network::protocol::Message;
pub struct Client {
    name: String,
}

impl Client {
    pub fn new(name: String) -> Self {
        Client { name }
    }

    pub async fn run(&self, remote_addr: SocketAddr) -> io::Result<()> {
        // Créer et connecter le socket
        let sock = UdpSocket::bind("0.0.0.0:0").await?;
        sock.connect(remote_addr).await?;
        println!("Client connecté au serveur: {}", remote_addr);

        // Créer un Arc pour partager le socket
        let sock = Arc::new(sock);

        // Envoyer le message de connexion
        let join_message = Message::Join {
            name: self.name.clone(),
        };
        let encoded = bincode::serialize(&join_message).unwrap();
        sock.send(&encoded).await?;

        // Cloner l'Arc pour la tâche d'entrée utilisateur
        let _sock_clone = sock.clone();
        let _name_clone = self.name.clone();
        
        // Spawn une tâche pour gérer les entrées utilisateur
        // tokio::spawn(async move {
        //     handle_user_input(sock_clone, name_clone).await
        // });

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
        io::stdout().flush().unwrap();
    
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();
    
        print!("Entrez l'adresse du serveur (ex: 0.0.0.0:8080) : ");
        io::stdout().flush().unwrap();
    
        let mut address = String::new();
        io::stdin().read_line(&mut address).unwrap();
        let server_address: SocketAddr = address.trim().parse::<SocketAddr>().expect("Adresse invalide.");
    
        println!(
            "Connexion au serveur {} avec le nom {}...",
            server_address, name
        );
        let serv = Self::new(name);
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(serv.run(server_address)).unwrap();
    }
}
