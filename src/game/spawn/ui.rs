//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::screen::Screen;
use crate::ui::widgets::Widgets;

use super::level::SpawnBoard;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum CardGameUIAction {
    ShuffleDeck,
    DrawHand,
    TurnNumber,
    TurnPhase,
    PhaseDescription,
}

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_card_game_ui);
}

fn spawn_card_game_ui(_trigger: Trigger<SpawnBoard>, mut commands: Commands) {
    commands
        .spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Val::Px(200.0),
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
        // todo: this does not nothing
        .insert(StateScoped(Screen::Playing))
        // .insert(StateScoped(TurnPhase::Prepare))
        .with_children(|children| {
            children
                .label("Turn number: 1")
                .insert(CardGameUIAction::TurnNumber);
            children
                .label("Turn phase: Prepare")
                .insert(CardGameUIAction::TurnPhase);
            children
                .label("Phase Description")
                .insert(CardGameUIAction::PhaseDescription);
            children
                .button("Shuffle Deck")
                .insert(CardGameUIAction::ShuffleDeck);
            children
                .button("Draw Hand")
                .insert(CardGameUIAction::DrawHand);
        });
}
