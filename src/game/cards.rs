use bevy::{app::App, prelude::*};
use bevy_la_mesa::events::{AlignCardsInHand, PlaceCardOffTable};
use bevy_la_mesa::{Card, CardMetadata, CardOnTable, Chip, ChipArea, LaMesaPluginSettings};

use std::fmt::Debug;
use std::marker::Send;

use crate::GameCamera;

#[derive(Clone, Copy, Debug, Default)]
enum CardType {
    Attack,
    BigDeal,
    #[default]
    Cocaine,
    Cannabis,
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

    let espionage = Kard {
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
        card_type: CardType::Cannabis,
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

        // deck.push(espionage.clone());

        // deck.push(attack.clone());

        // deck.push(police_bribe.clone());

        // ---

        // deck.push(drought.clone());

        // deck.push(big_deal.clone());
    }

    deck
}

#[derive(Default, Debug, PartialEq)]
pub enum TurnPhase {
    Prepare,
    #[default]
    Action,
    Event,
    ApplyCards,
    End,
}

#[derive(Resource)]
pub struct GameState {
    pub turn_number: usize,
    pub phase: TurnPhase,
    pub player: usize,
    pub bank: Vec<u16>,
    num_players: usize,
}

impl GameState {
    pub fn advance(&mut self) {
        self.phase = match self.phase {
            TurnPhase::Prepare => TurnPhase::Action,
            TurnPhase::Action => TurnPhase::ApplyCards,
            // TurnPhase::Event => TurnPhase::End,
            TurnPhase::ApplyCards => TurnPhase::End,
            TurnPhase::End => {
                if self.player == self.num_players {
                    self.turn_number += 1;
                    // self.player = 1;
                } else {
                    // self.player += 1;
                }
                TurnPhase::Prepare
            }
            _ => TurnPhase::End,
        }
    }

    pub fn new(num_players: usize) -> Self {
        Self {
            turn_number: 1,
            phase: TurnPhase::Prepare,
            player: 1,
            bank: vec![0; num_players],
            num_players,
        }
    }

    pub fn reset(&mut self) {
        self.turn_number = 1;
        self.phase = TurnPhase::Prepare;
        self.player = 1;
        self.bank = vec![0; self.num_players];
    }

    pub fn increase_bank(&mut self, player: usize, amount: u16) {
        self.bank[player - 1] += amount;
    }

    pub fn draw_bank(&mut self, player: usize, amount: u16) {
        self.bank[player - 1] -= amount;
    }

    pub fn get_balance(&self, player: usize) -> u16 {
        self.bank[player - 1]
    }
}

// Events

#[derive(Event)]
pub struct AdvancePhase;

#[derive(Event)]
pub struct SwitchPlayer;

#[derive(Event)]
pub struct DropChip {
    pub chip_type: ChipType,
    pub area: usize,
    pub player: usize,
}

#[derive(Debug, Event)]
pub struct MoveChip {
    pub entity: Entity,
    pub area: usize,
    pub player: usize,
}

#[derive(Debug, Event)]
pub struct DiscardChip {
    pub entity: Entity,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GameState::new(2))
        .add_event::<AdvancePhase>()
        .add_event::<DropChip>()
        .add_event::<MoveChip>()
        .add_event::<DiscardChip>()
        .add_event::<SwitchPlayer>()
        .add_systems(
            Update,
            (
                apply_card_effects,
                handle_next_phase,
                handle_drop_chip,
                handle_move_chip,
                handle_switch_player,
            ),
        );
}

pub fn apply_card_effects(
    mut state: ResMut<GameState>,
    cards_on_table: Query<(Entity, &Card<Kard>, &CardOnTable)>,
    chips_on_table: Query<(Entity, &Transform, &Chip<ChipType>, &ChipArea)>,
    mut ew_place_card_off_table: EventWriter<PlaceCardOffTable>,
    mut ew_drop_chip: EventWriter<DropChip>,
    mut ew_move_chip: EventWriter<MoveChip>,
    mut ew_discard_chip: EventWriter<DiscardChip>,
) {
    if state.phase == TurnPhase::ApplyCards {
        println!("Applying card effects");
        let player = state.player;

        for (entity, card, card_on_table) in cards_on_table.iter() {
            if card_on_table.player != player {
                continue;
            }

            match card.data.card_type {
                CardType::Cocaine => {
                    let event = DropChip {
                        chip_type: ChipType::Cocaine,
                        area: 1,
                        player: player,
                    };
                    ew_drop_chip.send(event);
                }
                CardType::Cannabis => {
                    let event = DropChip {
                        chip_type: ChipType::Cannabis,
                        area: 1,
                        player: player,
                    };
                    ew_drop_chip.send(event);
                }
                CardType::Truck | CardType::Train => {
                    let mut cannabis_chips_on_table = chips_on_table
                        .iter()
                        .filter(|(_, _, chip, area)| {
                            chip.data == ChipType::Cannabis
                                && area.marker == 1
                                && area.player == player
                        })
                        .collect::<Vec<_>>();
                    cannabis_chips_on_table.sort_by(|(_, t1, _, _), (_, t2, _, _)| {
                        t1.translation.z.partial_cmp(&t2.translation.z).unwrap()
                    });

                    let mut cocaine_chips_on_table = chips_on_table
                        .iter()
                        .filter(|(_, _, chip, area)| {
                            chip.data == ChipType::Cocaine
                                && area.marker == 1
                                && area.player == player
                        })
                        .collect::<Vec<_>>();

                    cocaine_chips_on_table.sort_by(|(_, t1, _, _), (_, t2, _, _)| {
                        t1.translation.z.partial_cmp(&t2.translation.z).unwrap()
                    });

                    let mut entities_to_move: Vec<Entity> = vec![];
                    let mut chip_value = match card.data.card_type {
                        CardType::Truck => 20,
                        CardType::Train => 50,
                        _ => 0,
                    };

                    let common_chips =
                        std::cmp::min(cannabis_chips_on_table.len(), cocaine_chips_on_table.len());

                    for i in 0..common_chips {
                        let cannabis_chip = cannabis_chips_on_table[i].0;
                        let cocaine_chip = cocaine_chips_on_table[i].0;

                        entities_to_move.push(cannabis_chip);
                        entities_to_move.push(cocaine_chip);
                    }

                    if common_chips < cannabis_chips_on_table.len() {
                        for i in common_chips..cannabis_chips_on_table.len() {
                            entities_to_move.push(cannabis_chips_on_table[i].0);
                        }
                    }

                    if common_chips < cocaine_chips_on_table.len() {
                        for i in common_chips..cocaine_chips_on_table.len() {
                            entities_to_move.push(cocaine_chips_on_table[i].0);
                        }
                    }

                    for entity in entities_to_move {
                        if chip_value <= 0 {
                            break;
                        }

                        let event = MoveChip {
                            entity,
                            area: 2,
                            player: player,
                        };
                        ew_move_chip.send(event);

                        chip_value -= 10;
                    }
                }
                CardType::Export | CardType::LocalMarket => {
                    let mut cannabis_chips_on_table = chips_on_table
                        .iter()
                        .filter(|(_, _, chip, area)| {
                            chip.data == ChipType::Cannabis
                                && area.marker == 2
                                && area.player == player
                        })
                        .collect::<Vec<_>>();
                    cannabis_chips_on_table.sort_by(|(_, t1, _, _), (_, t2, _, _)| {
                        t1.translation.z.partial_cmp(&t2.translation.z).unwrap()
                    });

                    let mut cocaine_chips_on_table = chips_on_table
                        .iter()
                        .filter(|(_, _, chip, area)| {
                            chip.data == ChipType::Cocaine
                                && area.marker == 2
                                && area.player == player
                        })
                        .collect::<Vec<_>>();

                    cocaine_chips_on_table.sort_by(|(_, t1, _, _), (_, t2, _, _)| {
                        t1.translation.z.partial_cmp(&t2.translation.z).unwrap()
                    });

                    let mut entities_to_discard: Vec<Entity> = vec![];
                    let mut chip_value = match card.data.card_type {
                        CardType::Export => 10,
                        CardType::LocalMarket => 50,
                        _ => 0,
                    };

                    let common_chips =
                        std::cmp::min(cannabis_chips_on_table.len(), cocaine_chips_on_table.len());

                    for i in 0..common_chips {
                        let cannabis_chip = cannabis_chips_on_table[i].0;
                        let cocaine_chip = cocaine_chips_on_table[i].0;

                        entities_to_discard.push(cannabis_chip);
                        entities_to_discard.push(cocaine_chip);
                    }

                    if common_chips < cannabis_chips_on_table.len() {
                        for i in common_chips..cannabis_chips_on_table.len() {
                            entities_to_discard.push(cannabis_chips_on_table[i].0);
                        }
                    }

                    if common_chips < cocaine_chips_on_table.len() {
                        for i in common_chips..cocaine_chips_on_table.len() {
                            entities_to_discard.push(cocaine_chips_on_table[i].0);
                        }
                    }

                    for entity in entities_to_discard {
                        if chip_value <= 0 {
                            break;
                        }

                        let event = DiscardChip { entity };

                        ew_discard_chip.send(event);
                        chip_value -= 10;

                        state.increase_bank(player, 100);
                    }
                }
                CardType::Attack => {}
                CardType::BigDeal => {}
                CardType::Drought => {}
                CardType::Espionage => {}
                CardType::PoliceBribe => {}
            }

            ew_place_card_off_table.send(PlaceCardOffTable {
                card_entity: entity,
                deck_marker: 1,
            });
        }

        state.advance();
    }
}

pub fn handle_next_phase(
    mut er_next_phase: EventReader<AdvancePhase>,
    mut ew_align_cards_in_hand: EventWriter<AlignCardsInHand>,
    mut game_state: ResMut<GameState>,
) {
    for _ in er_next_phase.read() {
        game_state.advance();

        println!("Aligning hand {:?}", game_state.phase);

        if game_state.phase == TurnPhase::ApplyCards {
            println!("Alinging hand");
            ew_align_cards_in_hand.send(AlignCardsInHand {
                player: game_state.player,
            });
        }
    }
}

pub fn handle_drop_chip(mut er_drop_chip: EventReader<DropChip>) {
    for drop_chip in er_drop_chip.read() {
        println!("Dropping chip: {:?}", drop_chip.chip_type);
    }
}

pub fn handle_move_chip(mut er_drop_chip: EventReader<MoveChip>) {
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
        game_state.player = match game_state.player {
            1 => 2,
            2 => 1,
            _ => 1,
        };

        for (mut transform, _) in query.iter_mut() {
            if game_state.player == 1 {
                *transform = Transform::from_xyz(0.0, 12.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y);
            } else {
                *transform = Transform::from_xyz(-3.0, 12.0, -15.0)
                    .looking_at(Vec3::ZERO + Vec3::new(-3.0, 0.0, 0.0), Vec3::Y);
            }
        }
    }
}
