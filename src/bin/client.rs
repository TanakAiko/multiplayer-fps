use bevy::prelude::*;
use multiplayer_fps::client::{
    plugins::{enemy_plugin::EnemyPlugin, player_plugin::PlayerPlugin, world_plugin::WorldPlugin},
    resources::network_resource::{input_connexion, NetworkResource},
    systems::enemy::receiving_update_enemy::handle_network_messages,
    udp::Client,
};
use tokio::runtime::Runtime;

fn main() {
    // Créer le runtime une seule fois
    let runtime = Runtime::new().unwrap();

    let (name, server_address) = input_connexion();

    // Établir la connexion et obtenir le socket
    let socket = runtime.block_on(async {
        let client = Client::new(name);
        match client.connect(server_address).await {
            Ok(ok) => ok,
            Err(e) => {
                println!("error: {:?}", e);
                panic!()
            }
        }
    });

    // Une fois connecté, démarrer Bevy
    App::new()
        .add_plugins((DefaultPlugins, WorldPlugin, PlayerPlugin, EnemyPlugin))
        // .add_plugins(PlayerPlugin)
        .insert_resource(NetworkResource::new(socket))
        .add_systems(Update, handle_network_messages)
        .run();
}
