use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use multiplayer_fps::client::{
    plugins::{player_plugin::PlayerPlugin, world_plugin::WorldPlugin}, resources::network_resource::{handle_network_messages, input_connexion, NetworkResource}, udp::Client
};
// use std::sync::Arc;
// use tokio::runtime::Runtime;

fn main() {
    // Créer le runtime une seule fois
    // let runtime = Runtime::new().unwrap();

    // let (name, server_address) = input_connexion();

    // Établir la connexion et obtenir le socket
    // let socket = runtime.block_on(async {
    //     let client = Client::new(name);
    // });


    // Une fois connecté, démarrer Bevy
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldPlugin,
            PlayerPlugin,
            // RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        // .add_plugins(PlayerPlugin)
        // .insert_resource(NetworkResource {
        //     socket: Arc::new(socket),
        // })
        // .add_systems(Update, handle_network_messages)
        .run();
}
