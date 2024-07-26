use std::time::Duration;

use bevy::prelude::*;
use bevy_la_mesa::{events::RenderDeck, Chip, ChipArea, DeckArea, HandArea, PlayArea};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};

use crate::game::cards::{ChipType, DiscardChip, DropChip, MoveChip};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(spawn_board);
    app.add_systems(
        Update,
        (handle_drop_chip, handle_move_chip_to_sales, discard_chip),
    )
    .add_systems(Startup, render_hand_area);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, _commands: Commands) {}

#[derive(Event, Debug)]
pub struct SpawnBoard;

#[derive(Component)]
pub struct ResourceArea {
    pub marker: usize,
    pub player: usize,
}

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
        transform: Transform::from_xyz(0.0, 7.0, 7.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 7.0, -7.0),
        ..default()
    });

    // Deck Area
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: materials.add(Color::BLACK),
            transform: Transform::from_translation(Vec3::new(-7.6, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            ..default()
        },
        DeckArea { marker: 1 },
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
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(1.2, 0.0, 3.5 * 1.2 / 2.0 + 0.1)),
            ..default()
        },
        ResourceArea {
            marker: 1,
            player: 1,
        },
        Name::new("Resources - Production - Player 1"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(2.5 * 1.2, 3.5 * 1.2)
                    .subdivisions(10),
            ),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(1.2, 0.0, -(3.5 * 1.2 / 2.0 + 0.1)))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..default()
        },
        ResourceArea {
            marker: 1,
            player: 2,
        },
        Name::new("Resources - Production - Player 2"),
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
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(4.5, 0.0, 3.5 * 1.2 / 2.0 + 0.1)),
            ..default()
        },
        ResourceArea {
            marker: 2,
            player: 1,
        },
        Name::new("Resources - Sales - Player 1"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(2.5 * 1.2, 3.5 * 1.2)
                    .subdivisions(10),
            ),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(4.5, 0.0, -(3.5 * 1.2 / 2.0 + 0.1)))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..default()
        },
        ResourceArea {
            marker: 2,
            player: 2,
        },
        Name::new("Resources - Sales - Player 2"),
    ));

    // Play Area 1
    let face_texture = asset_server.load("tarjetas/debug.png");
    let face_material = materials.add(StandardMaterial {
        base_color_texture: Some(face_texture.clone()),
        ..Default::default()
    });

    // ------------------------------

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6, 0.0, 6.2)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 1,
            player: 1,
        },
        Name::new("Play Area 1 - Player 1"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05, 0.0, 6.2)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 2,
            player: 1,
        },
        Name::new("Play Area 2 - Player 2"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 2.0, 0.0, 6.2)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 3,
            player: 1,
        },
        Name::new("Play Area 3 - Player 3"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 3.0, 0.0, 6.2)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 4,
            player: 1,
        },
        Name::new("Play Area 4 - Player 4"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 4.0, 0.0, 6.2)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 5,
            player: 1,
        },
        Name::new("Play Area 5 - Player 5"),
    ));

    // ------------------------------

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6, 0.0, -6.2))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 5,
            player: 2,
        },
        Name::new("Play Area 1 - Player 2"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05, 0.0, -6.2))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 4,
            player: 2,
        },
        Name::new("Play Area 2 - Player 2"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 2.0, 0.0, -6.2))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 3,
            player: 2,
        },
        Name::new("Play Area 3 - Player 2"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 3.0, 0.0, -6.2))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 2,
            player: 2,
        },
        Name::new("Play Area 4 - Player 2"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 4.0, 0.0, -6.2))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 1,
            player: 2,
        },
        Name::new("Play Area 5 - Player 2"),
    ));

    ew_render_deck.send(RenderDeck);
}

pub fn handle_drop_chip(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut er_drop_chip: EventReader<DropChip>,
    query: Query<(Entity, &ChipArea, &Chip<ChipType>)>,
) {
    let mut cocaine_counter = 0;
    let mut cannabis_counter = 0;
    for drop_chip in er_drop_chip.read() {
        let num_chips_of_kind = query
            .iter()
            .filter(|(_, area, chip)| {
                area.player == drop_chip.player
                    && area.marker == 1
                    && chip.data == drop_chip.chip_type
            })
            .count();

        let model = match drop_chip.chip_type {
            ChipType::Cannabis => asset_server
                .load("models/chip-cannabis/chip_for_tabletop_gam_0723233549_preview.obj"),
            ChipType::Cocaine => {
                asset_server.load("models/chip-cocaine/chip_for_tabletop_gam_0723233917_refine.obj")
            }
        };

        let mut initial_translation = match drop_chip.chip_type {
            ChipType::Cannabis => Transform::from_xyz(0.6, 12.0, 1.5).with_scale(Vec3::ONE * 1.0),
            ChipType::Cocaine => Transform::from_xyz(1.8, 12.0, 3.3).with_scale(Vec3::ONE * 1.0),
        }
        .translation;
        initial_translation.z *= if drop_chip.player == 1 { 1.0 } else { -1.0 };

        let mut final_translation = initial_translation;
        final_translation.y = 0.1
            + (match drop_chip.chip_type {
                ChipType::Cannabis => cannabis_counter,
                ChipType::Cocaine => cocaine_counter,
            } + num_chips_of_kind) as f32
                * 0.2;

        let tween: Tween<Transform> = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(350),
            TransformPositionLens {
                start: initial_translation,
                end: final_translation,
            },
        );

        commands.spawn((
            SceneBundle {
                scene: model,
                transform: match drop_chip.chip_type {
                    ChipType::Cannabis => {
                        Transform::from_xyz(0.6, 12.0, -5.2).with_scale(Vec3::ONE * 1.0)
                    }
                    ChipType::Cocaine => {
                        Transform::from_xyz(1.8, 12.0, -3.6).with_scale(Vec3::ONE * 1.0)
                    }
                },
                ..default()
            },
            Name::new("Chip"),
            Chip::<ChipType> {
                data: drop_chip.chip_type,
                turn_activation: 0,
            },
            ChipArea {
                player: drop_chip.player,
                marker: drop_chip.area,
            },
            Animator::new(tween),
        ));

        match drop_chip.chip_type {
            ChipType::Cannabis => cannabis_counter += 1,
            ChipType::Cocaine => cocaine_counter += 1,
        }
    }
}

pub fn handle_move_chip_to_sales(
    mut commands: Commands,
    mut er_move_chip: EventReader<MoveChip>,
    query: Query<(Entity, &Transform, &ChipArea, &Chip<ChipType>)>,
) {
    for move_chip in er_move_chip.read() {
        let chip = query.get(move_chip.entity).unwrap();
        let chip_type = chip.3.data;
        let initial_translation = chip.1.translation;
        let num_chips_of_kind = query
            .iter()
            .filter(|(_, _, area, chip)| area.marker == move_chip.area && chip.data == chip_type)
            .count();

        let mut final_translation = initial_translation;
        final_translation.x += 3.3; //* if move_chip.player == 1 { 1.0 } else { -1.0 };
        final_translation.y = 0.1 + num_chips_of_kind as f32 * 0.2;

        let tween: Tween<Transform> = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(350),
            TransformPositionLens {
                start: initial_translation,
                end: final_translation,
            },
        );

        commands
            .entity(move_chip.entity)
            .insert(Animator::new(tween))
            .insert(ChipArea {
                marker: move_chip.area,
                player: move_chip.player,
            });
    }
}

pub fn discard_chip(
    mut commands: Commands,
    mut er_discard_chip: EventReader<DiscardChip>,
    query: Query<(Entity, &Transform, &Chip<ChipType>)>,
) {
    for discard_chip in er_discard_chip.read() {
        let chip = query.get(discard_chip.entity).unwrap();
        let initial_translation = chip.1.translation;

        let mut final_translation = initial_translation;
        final_translation.y = 120.0;

        let tween: Tween<Transform> = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(350),
            TransformPositionLens {
                start: initial_translation,
                end: final_translation,
            },
        );

        commands
            .entity(discard_chip.entity)
            .insert(Animator::new(tween))
            .remove::<ChipArea>();
    }
}

pub fn render_hand_area(mut commands: Commands) {
    commands.spawn((
        Name::new("HandArea - Player 1"),
        TransformBundle {
            local: Transform::from_translation(Vec3::new(0.0, 3.5, 5.8))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 4.0)),
            ..default()
        },
        HandArea { player: 1 },
    ));

    commands.spawn((
        Name::new("HandArea - Player 2"),
        TransformBundle {
            local: Transform::from_translation(Vec3::new(0.0, 3.5, -5.8)).with_rotation(
                Quat::from_rotation_x(-std::f32::consts::PI / 4.0)
                    * Quat::from_rotation_y(std::f32::consts::PI),
            ),
            ..default()
        },
        HandArea { player: 2 },
    ));
}
