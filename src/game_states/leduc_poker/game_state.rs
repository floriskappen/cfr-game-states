use std::collections::HashMap;
use std::fmt::Debug;

use hand_isomorphism_rust::deck::card_from_string;
use hand_isomorphism_rust::deck::card_to_string;
use hand_isomorphism_rust::deck::deck_get_rank;
use hand_isomorphism_rust::deck::Card;
use itertools::Itertools;
use lazy_static::lazy_static;
use rand::rngs::StdRng;
use rand::prelude::*;

use crate::game_states::base_game_state::GameState;
use crate::structs::ActionType;
use crate::structs::ActionWithRaise;

lazy_static! {
    static ref CARD_RANKS: HashMap<String, i32> = {
        let mut m = HashMap::new();
        m.insert(format!("{}{}", "K", "K"), 1);
        m.insert(format!("{}{}", "Q", "Q"), 2);
        m.insert(format!("{}{}", "J", "J"), 3);
        m.insert(format!("{}{}", "K", "Q"), 4);
        m.insert(format!("{}{}", "Q", "K"), 4);
        m.insert(format!("{}{}", "K", "J"), 5);
        m.insert(format!("{}{}", "J", "K"), 5);
        m.insert(format!("{}{}", "Q", "J"), 6);
        m.insert(format!("{}{}", "J", "Q"), 6);
        return m
    };
}

const PRE_FLOP_INDEX: usize = 0;
const POST_FLOP_INDEX: usize = 1;

lazy_static! {
    static ref DECK: [Card; 6] = {
        [
            card_from_string("Kh".to_string()), card_from_string("Kd".to_string()),
            card_from_string("Qh".to_string()), card_from_string("Qd".to_string()),
            card_from_string("Jh".to_string()), card_from_string("Jd".to_string()),
        ]
    };
}

#[derive(Clone, Debug)]
pub struct LPGameState {
    pub player_amount: usize,
    pub round: usize,

    pub private_hands: Vec<Vec<Card>>,
    pub community_cards: Vec<Card>,
    pub bets: Vec<Vec<u16>>,

    pub history: Vec<Vec<ActionWithRaise>>,
    pub folded_players: Vec<u8>,
}

impl GameState for LPGameState {
    fn new_empty(_player_amount: usize, rng_seed: Option<u64>) -> Self {
        let mut rng = if let Some(seed) = rng_seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::seed_from_u64(thread_rng().next_u64())
        };

        let mut shuffled_cards = DECK.to_vec();

        shuffled_cards.shuffle(&mut rng);

        // Draw 3 items
        let drawn_items: Vec<Card> = shuffled_cards.iter().take(3).cloned().collect();

        return LPGameState {
            round: PRE_FLOP_INDEX,
            player_amount: 2,
            private_hands: vec![
                vec![drawn_items[0]],
                vec![drawn_items[1]]
            ],
            bets: vec![
                vec![1, 1], // First betting round with blinds
                vec![0, 0]  // Second betting round
            ],
            history: vec![vec![], vec![]],
            community_cards: vec![drawn_items[2]],
            folded_players: vec![],
        }
    }

    fn get_player_amount(&self) -> usize {
        return self.player_amount;
    }

    fn get_current_round_index(&self) -> usize {
        return self.round;
    }

    fn get_total_rounds() -> usize {
        return 2;
    }

    fn get_history(&self) -> &Vec<Vec<ActionWithRaise>> {
        return &self.history;
    }

    fn get_active_player_index(&self) -> usize {
        if self.round == 1 {
            return (self.history[self.round].len() + 1) % 2
        }

        return self.history[self.round].len() % 2;
    }

    fn get_current_round_bet_raise_amount(&self) -> usize {
        return self.history[self.round].iter().filter(|&action_with_raise| action_with_raise.is_bet_raise()).count();
    }

    fn get_active_player_actions(&self, _available_actions: &Vec<ActionWithRaise>) -> Vec<ActionWithRaise> {
        let bet_raise_amount = self.get_current_round_bet_raise_amount();

        // If there was a bet this round
        if bet_raise_amount > 0 {            
            // If there were less than 2 raises we can still raise more
            if bet_raise_amount < 2 {
                return vec![ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1 }]
            }

            return vec![ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }]
        }

        return vec![ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1 }];
    }

    fn is_terminal(&self) -> bool {
        // If anyone folded at any point, it's terminal
        if self.history.iter()
            .map(|round_history| {
                if round_history.iter().contains(&ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 }) {
                    return true
                }
                return false
            })
            .contains(&true) {
                return true
            }

        // If the second round is finished
        if self.round == 1 && (self.all_players_checked() || self.bet_or_raise_finished()) {
            return true
        }

        return false
    }

    fn is_leaf_node(&self, _leaf_node_situation: usize) -> bool {
        // In LP we always search until the end of the game, so we're never in a leaf node.
        return false;
    }

    fn can_proceed_to_next_round(&self) -> bool {
        if self.round == 0 {
            if self.all_players_checked() || self.bet_or_raise_finished() {
                return true
            }
        }

        return false
    }

    fn get_payoffs(&self) -> Vec<i32> {
        // All but 1 folded
        if self.history.concat().contains(&ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 }) {
            let mut folded_player_index: usize = usize::MIN;
            for i in 0..self.history.len() {
                for (j, action) in self.history[i].iter().enumerate() {
                    if action.action_type == ActionType::Fold {
                        if i == 0 {
                            folded_player_index = j % 2;
                        } else {
                            folded_player_index = (j + 1) % 2;
                        }
                        break;
                    }
                }
            }

            let payoffs: Vec<i32> = (0..self.player_amount).map(|i| {
                if i == folded_player_index {
                    return -((self.bets[0][i] + self.bets[1][i]) as i32)
                } else {
                    return (self.bets[0][(i+1) % 2] + self.bets[1][(i+1) % 2]) as i32
                }
            }).collect();

            return payoffs;
        }

        // Tie
        if self.private_hands.iter().all(|private_hand| deck_get_rank(private_hand[0]) == deck_get_rank(self.private_hands[0][0])) {
            return vec![0; self.player_amount];
        }

        // Showdown
        let mut player_card_ranks: Vec<i32> = Vec::new();
        for player_index in 0..self.player_amount {
            let player_cards = vec![self.private_hands[player_index][0], self.community_cards[0]];

            let player_cards_string = player_cards
                .iter()
                .map(|card: &Card| card_to_string(card.clone()).chars().next().unwrap().to_string())
                .collect::<Vec<String>>()
                .join("");

            player_card_ranks.push(*CARD_RANKS.get(&player_cards_string).unwrap());
        }

        let player_winner_index = player_card_ranks.iter().position(
            |&rank| rank == *player_card_ranks
                                    .iter()
                                    .min()
                                    .unwrap()
        ).unwrap();

        let payoffs = (0..self.player_amount).map(|player_index| {

            let mut payoff: i32 = 0;
            if player_index == player_winner_index {
                for round_index in 0..self.bets.len() {
                    // Winning player gets payout of the bets of each other player
                    for player_bets_index in 0..self.bets[round_index].len() {
                        if player_bets_index != player_index {
                            payoff += self.bets[round_index][player_bets_index] as i32
                        }
                    }
                }
            } else {
                // Losing player gets a payout of (-1 * their own bet)
                for round_index in 0..self.bets.len() {
                    payoff -= self.bets[round_index][player_index] as i32
                }
            }

            return payoff;
        }).collect();

        return payoffs;
    }

    fn handle_action(&self, action_with_raise: ActionWithRaise) -> Self {
        let mut new_bets = self.bets.clone();
        let active_player_index = self.get_active_player_index();

        let active_player_current_round_bet = self.bets[self.round][active_player_index];
        let opponent_current_round_bet = self.bets[self.round][(active_player_index + 1) % 2];

        if action_with_raise.action_type != ActionType::Fold {
            // Always match the opponent bet first
            let mut bet_increase_amount = opponent_current_round_bet - active_player_current_round_bet;
    
            if action_with_raise.action_type == ActionType::Bet {
                let mut raise_amount = 2;
                if self.round == 1 {
                    raise_amount = 4;
                }

                bet_increase_amount += raise_amount;
            }

            new_bets[self.round][active_player_index] += bet_increase_amount;
        }

        let mut next_state = LPGameState {
            player_amount: self.player_amount,
            private_hands: self.private_hands.clone(),
            bets: new_bets,
            round: self.round,
            history: self.history.clone(),
            community_cards: self.community_cards.clone(),
            folded_players: self.folded_players.clone()
        };
        next_state.history[next_state.round].push(action_with_raise);

        if next_state.can_proceed_to_next_round() {
            next_state.round = 1;
        }

        return next_state
    }
}

impl LPGameState {
    // Returns True if all players checked in the current round
    fn all_players_checked(&self) -> bool {
        let num_checked = self.history[self.round].iter().filter(|&action| action == &ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }).count();

        return num_checked == self.player_amount && !self.history[self.round].iter().any(|action| action == &ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1 })
    }

    // Determines if the current betting round is finished, i.e., a bet or raise has been called or everyone has folded.
    fn bet_or_raise_finished(&self) -> bool {
        for i in 0..self.history[self.round].len() {
            let reversed_index = self.history[self.round].len() - 1 - i;
            let reversed_action_with_raise = &self.history[self.round][reversed_index];
            if reversed_action_with_raise.action_type == ActionType::Bet {
                if self.history[self.round][reversed_index..].len() == self.player_amount {
                    return true
                }
                break
            }
        }

        return false
    }
}
