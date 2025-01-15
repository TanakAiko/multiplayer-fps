use std::{io,io::Write};
mod Server;
mod Client;


 fn main()  {
    println!("Bienvenue dans le jeu Maze Wars !");
    println!("1. Créer un serveur");
    println!("2. Rejoindre un serveur");

    // Demande de choix à l'utilisateur
    let choice = loop {
        print!("Choisissez une option (1 ou 2) : ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => break 1,
            "2" => break 2,
            _ => println!("Choix invalide. Veuillez entrer 1 ou 2."),
        }
    };

    match choice {
        1 => Server::Server::start_server(),
        2 => Client::Client::start_client(),
        _ => unreachable!(),
    }
}