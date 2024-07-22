use bevy::{app::App, prelude::*};
use bevy_la_mesa::{CardMetadata, LaMesaPluginSettings};

use std::fmt::Debug;

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

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GameState {
        turn_number: 0,
        phase: TurnPhase::Prepare,
        player_number: 1,
    })
    .add_event::<NextPhase>()
    .add_systems(Update, handle_next_phase);
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
