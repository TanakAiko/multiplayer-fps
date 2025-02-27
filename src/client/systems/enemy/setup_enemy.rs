use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::{
    components::enemy_component::Enemy,
    resources::{
        animation_resource::{AnimationResource, AnimationState},
        enemy_resource::EnemyResource,
    },
};

const GLB_ENEMY: &str = "fps_enemy.glb";
// const ENEMY_INITIAL_POSITION: Vec3 = Vec3::new(-12., -1., 13.); // C'est en faite la meme position que le player
lazy_static::lazy_static! {
    static ref ENEMY_INITIAL_ROTATION: Quat = Quat::from_rotation_y(PI);
}

#[derive(Bundle, Debug, Default)]
pub struct EnemyBundle {
    // pub enemy: Enemy,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    collider: Collider,
    rigid_body: RigidBody,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
    collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
}

pub fn spawn_enemy(
    name: String,
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
) {
    let capsule_height = 1.01; // Hauteur du corps
    let capsule_radius = 0.305; // Rayon
    let collider_offset = capsule_height / 2.0;

    // let scene_handle: Handle<Scene> = asset_server.load("fps_enemy.gltf#Scene0");
    let scene_handle: SceneRoot =
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLB_ENEMY)));

    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(4).from_asset(GLB_ENEMY)),
        asset_server.load(GltfAssetLabel::Animation(16).from_asset(GLB_ENEMY)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(GLB_ENEMY)),
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GLB_ENEMY)),
        asset_server.load(GltfAssetLabel::Animation(6).from_asset(GLB_ENEMY)),
    ]);

    let graph_handle = graphs.add(graph);

    commands.insert_resource(AnimationResource {
        animations: node_indices,
        graph_handle,
    });

    commands.insert_resource(AnimationState::default());

    commands
        .spawn((
            // SceneRoot(scene_handle), // 🔹 Modèle 3D
            scene_handle,
            AnimationPlayer::default(),
            AnimationTransitions::default(),
            Transform {
                translation: position, // 🔹 Position réelle de l'avatar (sans modification de Y)
                rotation: *ENEMY_INITIAL_ROTATION,
                scale: Vec3::ONE,
            },
            Enemy {
                name,
                position,
                orientation: *ENEMY_INITIAL_ROTATION,
                current_state: Default::default(), // Idle
                animation_timer: Timer::from_seconds(0.5, TimerMode::Once),
            },
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn((EnemyBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, collider_offset, 0.0), // 🔹 Seul le collider est déplacé
                    ..Default::default()
                },
                visibility: Visibility::default(),
                gravity_scale: GravityScale(0.0),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                collision_types: ActiveCollisionTypes::DYNAMIC_STATIC,
                active_events: ActiveEvents::COLLISION_EVENTS,
                global_transform: GlobalTransform::default(),
                rigid_body: RigidBody::Fixed,
                collider: Collider::capsule_y(capsule_height, capsule_radius),
            },));
        });
}

pub fn spawn_all_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_resource: Res<EnemyResource>,
    mut graph: ResMut<Assets<AnimationGraph>>,
) {
    for enemy in enemy_resource.enemies.iter() {
        println!("enemy {:?}", enemy);
        spawn_enemy(
            enemy.name.clone(),
            commands.reborrow(),
            &asset_server,
            enemy.position,
            &mut graph,
        );
    }
}
