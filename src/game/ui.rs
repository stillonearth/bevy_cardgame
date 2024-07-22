use bevy::prelude::*;
use bevy_la_mesa::{
    events::{DeckShuffle, DrawHand},
    DeckArea,
};

use super::{
    cards::{GameState, NextPhase, TurnPhase},
    spawn::ui::CardGameUIAction,
};
use crate::ui::prelude::InteractionQuery;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (handle_gameplay_action, handle_labels));
}

fn handle_gameplay_action(
    mut button_query: InteractionQuery<&CardGameUIAction>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    mut ew_draw: EventWriter<DrawHand>,
    mut ew_next_phase: EventWriter<NextPhase>,
    decks: Query<(Entity, &DeckArea)>,
) {
    for (interaction, action) in &mut button_query {
        let deck_entity = decks.iter().next().unwrap().0;

        if matches!(interaction, Interaction::Pressed) {
            match action {
                CardGameUIAction::ShuffleDeck => {
                    let event = DeckShuffle { deck_entity };
                    ew_shuffle.send(event);
                }
                CardGameUIAction::DrawHand => {
                    let event = DrawHand {
                        deck_entity,
                        num_cards: 5,
                        player: 1,
                    };
                    ew_draw.send(event);
                    ew_next_phase.send(NextPhase);
                }
                _ => {}
            }
        }
    }
}

fn handle_labels(
    mut label_query: Query<(Entity, &mut Visibility, &CardGameUIAction)>,
    mut text_query: Query<(&Parent, &mut Text)>,
    state: Res<GameState>,
) {
    for (entity, mut visibility, ui_element) in &mut label_query {
        for (parent, mut text) in text_query.iter_mut() {
            if parent.index() != entity.index() {
                continue;
            }
            match ui_element {
                CardGameUIAction::TurnNumber => {
                    text.sections[0].value = format!("Turn number: {}", state.turn_number);
                }
                CardGameUIAction::TurnPhase => {
                    text.sections[0].value = format!("Turn phase: {:?}", state.phase);
                }
                CardGameUIAction::ShuffleDeck => {
                    if state.phase == TurnPhase::Prepare {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                CardGameUIAction::DrawHand => {
                    if state.phase == TurnPhase::Prepare {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
                CardGameUIAction::PhaseDescription => {
                    text.sections[0].value = match state.phase {
                        TurnPhase::Prepare => {
                            "You may shuffle the deck and draw 5 cards".to_string()
                        }
                        TurnPhase::Action => {
                            "You may play cards from your hand or draw".to_string()
                        }
                        TurnPhase::Event => "Draw a card from event deck and play it".to_string(),
                        TurnPhase::End => "Update your counters and pass turn".to_string(),
                    };
                }
            }
        }
    }
}
