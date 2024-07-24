use bevy::ecs::query;
use bevy::{app::App, prelude::*};
use bevy_la_mesa::{events::CardPress, Card, CardMetadata, Deck, LaMesaPluginSettings};

use std::fmt::Debug;
use std::marker::Send;

use crate::GameCamera;

#[derive(Clone, Copy, Debug, Default)]
enum CardType {
    Attack,
    BigDeal,
    #[default]
    Cocaine,
    Marijuana,
    Drought,
    Espionage,
    Export,
    LocalMarket,
    PoliceBribe,
    Train,
    Truck,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ChipType {
    #[default]
    Cocaine,
    Cannabis,
}

#[derive(Default, Clone, Debug)]
pub struct Kard {
    pub card_type: CardType,
    pub filename: String,
}

impl CardMetadata for Kard {
    type Output = Kard;

    fn filename(&self) -> String {
        self.filename.clone()
    }
}

pub fn load_deck(num_players: usize) -> Vec<Kard> {
    let attack = Kard {
        card_type: CardType::Attack,
        filename: "tarjetas/attack.png".to_string(),
    };

    let big_deal = Kard {
        card_type: CardType::BigDeal,
        filename: "tarjetas/big-deal.png".to_string(),
    };

    let cocaine = Kard {
        card_type: CardType::Cocaine,
        filename: "tarjetas/cocaine.png".to_string(),
    };

    let drought = Kard {
        card_type: CardType::Drought,
        filename: "tarjetas/drought.png".to_string(),
    };

    let espinage = Kard {
        card_type: CardType::Espionage,
        filename: "tarjetas/espionage.png".to_string(),
    };

    let export = Kard {
        card_type: CardType::Export,
        filename: "tarjetas/export.png".to_string(),
    };

    let local_market = Kard {
        card_type: CardType::LocalMarket,
        filename: "tarjetas/local-market.png".to_string(),
    };

    let marijuana = Kard {
        card_type: CardType::Marijuana,
        filename: "tarjetas/marijuana.png".to_string(),
    };

    let police_bribe = Kard {
        card_type: CardType::PoliceBribe,
        filename: "tarjetas/police-bribe.png".to_string(),
    };

    let train = Kard {
        card_type: CardType::Train,
        filename: "tarjetas/train.png".to_string(),
    };

    let truck = Kard {
        card_type: CardType::Truck,
        filename: "tarjetas/truck.png".to_string(),
    };

    let mut deck: Vec<Kard> = vec![];
    for _ in 0..num_players {
        deck.push(cocaine.clone());
        deck.push(cocaine.clone());

        deck.push(marijuana.clone());
        deck.push(marijuana.clone());

        deck.push(truck.clone());
        deck.push(truck.clone());

        deck.push(train.clone());

        deck.push(local_market.clone());
        deck.push(local_market.clone());

        deck.push(export.clone());

        deck.push(espinage.clone());

        deck.push(attack.clone());

        deck.push(police_bribe.clone());

        deck.push(drought.clone());

        deck.push(big_deal.clone());
    }

    deck
}

#[derive(Default, Debug, PartialEq)]
pub enum TurnPhase {
    Prepare,
    #[default]
    Action,
    Event,
    End,
}

#[derive(Resource)]
pub struct GameState {
    pub turn_number: usize,
    pub phase: TurnPhase,
    pub player_number: usize,
}

impl GameState {
    pub fn advance(&mut self, num_players: usize) {
        self.phase = match self.phase {
            TurnPhase::Prepare => TurnPhase::Action,
            TurnPhase::Action => TurnPhase::Event,
            TurnPhase::Event => TurnPhase::End,
            TurnPhase::End => {
                if self.player_number == num_players {
                    self.turn_number += 1;
                    self.player_number = 1;
                } else {
                    self.player_number += 1;
                }
                TurnPhase::Prepare
            }
        }
    }
}

// Events

#[derive(Event)]
pub struct NextPhase;

#[derive(Event)]
pub struct SwitchPlayer;

#[derive(Event)]
pub struct DropChip {
    pub chip_type: ChipType,
    pub area: usize,
}

#[derive(Debug, Event)]
pub struct MoveChip {
    pub entity: Entity,
    pub area: usize,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GameState {
        turn_number: 0,
        phase: TurnPhase::Prepare,
        player_number: 1,
    })
    .add_event::<NextPhase>()
    .add_event::<DropChip>()
    .add_event::<MoveChip>()
    .add_event::<SwitchPlayer>()
    .add_systems(
        Update,
        (
            handle_next_phase,
            handle_drop_chip,
            handle_move_chip,
            handle_switch_player,
        ),
    );
}

pub fn handle_next_phase(
    mut er_next_phase: EventReader<NextPhase>,
    mut game_state: ResMut<GameState>,
    plugin_settings: Res<LaMesaPluginSettings<Kard>>,
) {
    for _ in er_next_phase.read() {
        game_state.advance(plugin_settings.num_players);
    }
}

pub fn handle_drop_chip(
    mut er_drop_chip: EventReader<DropChip>,
    mut game_state: ResMut<GameState>,
    plugin_settings: Res<LaMesaPluginSettings<Kard>>,
) {
    for drop_chip in er_drop_chip.read() {
        println!("Dropping chip: {:?}", drop_chip.chip_type);
    }
}

pub fn handle_move_chip(
    mut er_drop_chip: EventReader<MoveChip>,
    mut game_state: ResMut<GameState>,
    plugin_settings: Res<LaMesaPluginSettings<Kard>>,
) {
    for move_chip in er_drop_chip.read() {
        println!("Moving chip: {:?}", move_chip);
    }
}

pub fn handle_switch_player(
    mut er_drop_chip: EventReader<SwitchPlayer>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut Transform, &GameCamera)>,
) {
    for _ in er_drop_chip.read() {
        game_state.player_number = match game_state.player_number {
            1 => 2,
            2 => 1,
            _ => 1,
        };

        for (mut transform, _) in query.iter_mut() {
            if game_state.player_number == 1 {
                *transform = Transform::from_xyz(0.0, 12.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y);
            } else {
                *transform = Transform::from_xyz(0.0, 12.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y);
            }
        }
    }
}
