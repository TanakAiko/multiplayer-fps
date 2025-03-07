use std::f32::consts::PI;

use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_rapier3d::prelude::*;

use crate::client::{components::{animation_component::AnimationComponent, enemy_component::{Enemy, EnemyMovement}}, resources::enemy_resource::EnemyResource};

const GLB_ENEMY: &str = "fps_enemy.glb";
// const ENEMY_INITIAL_POSITION: Vec3 = Vec3::new(-12., -1., 13.); // C'est en faite la meme position que le player
lazy_static::lazy_static! {
    static ref ENEMY_INITIAL_ROTATION: Quat = Quat::from_rotation_y(PI);
}

#[derive(Bundle, Debug, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    enemy_movement: EnemyMovement,
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
    graphs : &mut ResMut<Assets<AnimationGraph>>,
) {
    
    let capsule_height = 1.01; // Hauteur du corps
    let capsule_radius = 0.305; // Rayon
    let collider_offset = capsule_height / 2.0;
    
    // let scene_handle: Handle<Scene> = asset_server.load("fps_enemy.gltf#Scene0");
    let scene_handle: SceneRoot =  SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLB_ENEMY)));
    
    let (idle_graph, _idle_index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(4).from_asset(GLB_ENEMY))
    );
    let (run_graph, _) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(16).from_asset(GLB_ENEMY))
    );
    let (shoot_graph, _) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(GLB_ENEMY))
    );
    let (death_graph, _) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GLB_ENEMY))
    );
    let (gun_pointing_graph, gun_pointing_index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(6).from_asset(GLB_ENEMY))
    );

    graphs.add(idle_graph);
    graphs.add(run_graph);
    graphs.add(shoot_graph);
    graphs.add(death_graph);
    let idle_graph_handle = graphs.add(gun_pointing_graph);
    
    let animation_to_play = AnimationComponent {
        graph_handle: idle_graph_handle,
        index: gun_pointing_index,
    };


    commands
        .spawn((
            // SceneRoot(scene_handle), // 🔹 Modèle 3D
            scene_handle,
            animation_to_play,
            AnimationPlayer::default(),
            Transform {
                translation: position, // 🔹 Position réelle de l'avatar (sans modification de Y)
                rotation: *ENEMY_INITIAL_ROTATION,
                scale: Vec3::ONE,
            },
            GlobalTransform::default(),
        )).observe(play_animation_when_ready)
        .with_children(|parent| {
            parent.spawn((EnemyBundle {
                enemy: Enemy {
                    name,
                    position,
                    orientation: *ENEMY_INITIAL_ROTATION,
                    current_state: Default::default(), // Idle
                },
                enemy_movement: EnemyMovement {
                    target_position: position,
                    current_position: position,
                    lerp_time: 0.0,
                },
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

fn play_animation_when_ready(
    trigger: Trigger<SceneInstanceReady>, // Déclenché quand une scène 3D est chargée
    mut commands: Commands,
    children: Query<&Children>,  // Pour accéder aux enfants des entités
    animations_to_play: Query<&AnimationComponent>, // Les animations à jouer
    mut players: Query<&mut AnimationPlayer>, // Les lecteurs d'animation
) {
    if let Ok(animation_to_play) = animations_to_play.get(trigger.entity()) {
        for child in children.iter_descendants(trigger.entity()) {
            
            if let Ok(mut player) = players.get_mut(child) {
                // T start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                player.play(animation_to_play.index);

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }

}

pub fn spawn_all_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_resource: Res<EnemyResource>,
    mut graph : ResMut<Assets<AnimationGraph>>
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
