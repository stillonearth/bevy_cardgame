//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(spawn_board);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    // commands.trigger(SpawnPlayer);
}

#[derive(Event, Debug)]
pub struct SpawnBoard;

fn spawn_board(_trigger: Trigger<SpawnBoard>, mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 4.0),
        ..default()
    });
}
