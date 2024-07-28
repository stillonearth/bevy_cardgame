//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_la_mesa::events::{CardPress, PlaceCardOnTable};
use bevy_la_mesa::{Card, CardOnTable, Hand};

use crate::game::cards::{GameState, Kard};
use crate::screen::Screen;
use crate::ui::widgets::Widgets;

use super::level::SpawnBoard;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum CardGameUIAction {
    ButtonShuffleDeck,
    ButtonDrawHand,
    ButtonDropChip,
    ButtonMoveChip,
    ButtonAdvancePhase,
    ButtonSwitchPlayer,
    LabelPlayerNumber,
    LabelTurnNumber,
    LabelTurnPhase,
    LabelPhaseDescription,
    LabelBank,
    LabelEffects,
    LabelGameOver,
    ContainerGameOver,
}

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_card_game_ui)
        .add_systems(Update, handle_card_press);
}

fn spawn_card_game_ui(_trigger: Trigger<SpawnBoard>, mut commands: Commands) {
    commands
        .spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Val::Px(216.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    right: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK),
                ..default()
            },
        ))
        .insert(StateScoped(Screen::Playing))
        .with_children(|children| {
            children
                .label("Turn number: 1")
                .insert(CardGameUIAction::LabelTurnNumber);
            children
                .label("Player number: 1")
                .insert(CardGameUIAction::LabelPlayerNumber);
            children
                .label("Turn phase: Prepare")
                .insert(CardGameUIAction::LabelTurnPhase);
            children
                .label("Phase Description")
                .insert(CardGameUIAction::LabelPhaseDescription);
            children
                .label("Effects")
                .insert(CardGameUIAction::LabelEffects);
            children
                .label("Bank: $0")
                .insert(CardGameUIAction::LabelBank);
            // children
            //     .button("Switch Player")
            //     .insert(CardGameUIAction::ButtonSwitchPlayer);
            children
                .button("Shuffle Deck")
                .insert(CardGameUIAction::ButtonShuffleDeck);
            children
                .button("Draw Hand")
                .insert(CardGameUIAction::ButtonDrawHand);
            children
                .button("Advance Phase")
                .insert(CardGameUIAction::ButtonAdvancePhase);
            // children
            //     .button("Drop Chip")
            //     .insert(CardGameUIAction::ButtonDropChip);
            // children
            //     .button("Move Chip")
            //     .insert(CardGameUIAction::ButtonMoveChip);
        });

    let text = Text::from_section(
        "GAME OVER",
        TextStyle {
            font_size: 100.0,
            color: Color::WHITE,
            ..default()
        },
    );

    // root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Name::new("Game Over"),
            CardGameUIAction::ContainerGameOver,
        ))
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text,
                    ..Default::default()
                })
                .insert((CardGameUIAction::LabelGameOver));
        });
}

pub fn handle_card_press(
    mut card_press: EventReader<CardPress>,
    query_cards_in_hand: Query<(Entity, &Card<Kard>, &Hand)>,
    query_cards_on_table: Query<(Entity, &Card<Kard>, &CardOnTable)>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    mut state: ResMut<GameState>,
) {
    let player = state.player;
    for event in card_press.read() {
        let hand = query_cards_in_hand.get(event.card_entity).ok();
        if hand.is_none() {
            continue;
        }
        let (_, kard, hand) = hand.unwrap();

        if kard.data.price > state.get_balance(state.player) {
            continue;
        }

        state.change_balance(player, -kard.data.price);
        let markers: Vec<usize> = query_cards_on_table
            .iter()
            .filter(|(_, _, t)| t.player == hand.player)
            .map(|(_, _, t)| t.marker)
            .collect();

        let marker = markers.iter().max().unwrap_or(&0) + 1;
        if marker > 5 {
            continue;
        }

        ew_place_card_on_table.send(PlaceCardOnTable {
            card_entity: event.card_entity,
            marker,
            player: hand.player,
        });
    }
}
