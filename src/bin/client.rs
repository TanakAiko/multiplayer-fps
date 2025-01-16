use multiplayer_fps::client::udp::Client;
use bevy::prelude::*;

#[derive(Debug, Component)]
struct Velocity {
    pub value: Vec3
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_spaceship)
            .add_systems(Update, (update_position, print_position));
    }
}

fn spawn_spaceship(mut commands: Commands) {
    commands
        .spawn(
            (
                Transform::default(),
                Velocity {
                    value: Vec3::new(1., 1., 1.)
                }
            )
        );
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self , app: &mut App){
        app.add_systems(Update, update_position);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);    
    }
}


// System Query
fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.translation.x += velocity.value.x;
        position.translation.y += velocity.value.y;
        position.translation.z += velocity.value.z;
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at pos {:?}", entity, transform.translation)
    }
}

fn main() {
    Client::start_client();
    
    App::new()
    .add_systems(Startup, spawn_spaceship)
    .add_systems(Update, (update_position, print_position))
    .add_plugins(DefaultPlugins)
        .run();
}