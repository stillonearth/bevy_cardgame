//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_la_mesa::{events::RenderDeck, DeckArea, PlayArea};

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

fn spawn_board(
    _trigger: Trigger<SpawnBoard>,
    mut commands: Commands,
    mut ew_render_deck: EventWriter<RenderDeck>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 4.0),
        ..default()
    });

    // Deck Area
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: materials.add(Color::BLACK),
            transform: Transform::from_translation(Vec3::new(-7.6, 0.0, -4.1)),
            ..default()
        },
        DeckArea,
        Name::new("Deck"),
    ));

    // Resources - Production
    let face_texture = asset_server.load("tarjetas/resources-sales.png");
    let face_material = materials.add(StandardMaterial {
        base_color_texture: Some(face_texture.clone()),
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(2.5 * 1.2, 3.5 * 1.2)
                    .subdivisions(10),
            ),
            material: face_material,
            transform: Transform::from_translation(Vec3::new(1.2, 0.0, -4.6)),
            ..default()
        },
        PlayArea { marker: 0 },
        Name::new("Resources - Production"),
    ));

    // Resources - Sales
    let face_texture = asset_server.load("tarjetas/resources-production.png");
    let face_material = materials.add(StandardMaterial {
        base_color_texture: Some(face_texture.clone()),
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(2.5 * 1.2, 3.5 * 1.2)
                    .subdivisions(10),
            ),
            material: face_material,
            transform: Transform::from_translation(Vec3::new(4.5, 0.0, -4.6)),
            ..default()
        },
        PlayArea { marker: 0 },
        Name::new("Resources - Sales"),
    ));

    // Play Area 1
    let face_texture = asset_server.load("tarjetas/debug.png");
    let face_material = materials.add(StandardMaterial {
        base_color_texture: Some(face_texture.clone()),
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6, 0.0, -0.4)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea { marker: 1 },
        Name::new("Play Area 1"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05, 0.0, -0.4)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea { marker: 2 },
        Name::new("Play Area 2"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 2.0, 0.0, -0.4)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea { marker: 3 },
        Name::new("Play Area 3"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 3.0, 0.0, -0.4)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea { marker: 4 },
        Name::new("Play Area 4"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 4.0, 0.0, -0.4)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea { marker: 5 },
        Name::new("Play Area 5"),
    ));

    ew_render_deck.send(RenderDeck);
}
