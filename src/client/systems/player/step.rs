use bevy::{
    asset::AssetServer,
    audio::{AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackSettings},
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
};

use crate::client::components::player_component::{PlayerShoot, PlayerStep};

pub fn setupsoundsprint(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/running.ogg")),
        PlaybackSettings {
            paused: true,
            mode: bevy::audio::PlaybackMode::Loop,
            ..Default::default()
        },
        PlayerStep,
    ));
}

pub fn playsoundsprint(music_controller: &Query<&AudioSink, With<PlayerStep>>, is_moving: bool) {
    if let Ok(sink) = music_controller.get_single() {
        if is_moving {
            sink.play();
        } else {
            sink.pause();
        }
    }
}

pub fn setupsoundshoot(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/shoot.ogg")),
        PlaybackSettings {
            paused: true,
            mode: bevy::audio::PlaybackMode::Loop,
            ..Default::default()
        },
        PlayerShoot,
    ));
}

pub fn playsoundshoot(music_controller: &Query<&AudioSink, With<PlayerShoot>>, is_shooting: bool) {
    if let Ok(sink) = music_controller.get_single() {
        if is_shooting {
            sink.play();
        } else {
            sink.pause();
        }
    }
}
