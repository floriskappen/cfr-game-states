use std::collections::HashMap;
use hand_isomorphism_rust::deck::{card_from_string, card_to_string, deck_get_rank, Card};
use hand_isomorphism_rust::hand_indexer::HandIndexer;
use itertools::Itertools;
use rand::prelude::*;
use lazy_static::lazy_static;

use crate::game_states::base_game_state::GameState;
use crate::structs::Action;

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
    pub round: usize,
    pub player_amount: usize,
    pub bets: Vec<Vec<usize>>,
    pub history: Vec<Vec<Action>>,
    pub private_hands: Vec<Vec<Card>>,
    pub community_cards: Vec<Card>,
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
            community_cards: vec![drawn_items[2]]
        }
    }

    fn get_total_rounds() -> usize {
        return 2;
    }

    fn get_current_round_index(&self) -> usize {
        return self.round;
    }

    fn get_player_amount(&self) -> usize {
        return self.player_amount;
    }

    fn get_active_player_index(&self) -> usize {
        if self.round == POST_FLOP_INDEX {
            return (self.history[self.round].len() + 1) % 2
        }

        return self.history[self.round].len() % 2;
    }

    fn get_active_player_actions(&self) -> Vec<Action> {
        // If there was a bet this round
        if self.history[self.round].contains(&Action::Bet) {            
            let raise_occurrence_count = self.history[self.round].iter().filter(|&action| action == &Action::Bet).count();

            // If there were less than 2 raises we can still raise more
            if raise_occurrence_count < 2 {
                return vec![Action::Fold, Action::Call, Action::Bet]
            }

            return vec![Action::Fold, Action::Call]
        }

        return vec![Action::Call, Action::Bet];
    }

    fn is_terminal(&self) -> bool {
        // Qh, Jd, Call, Call, Bet, Bet
        // if self.private_hands[1] == vec![card_from_string("Qh".to_string())] && self.community_cards == vec![card_from_string("Jd".to_string())] && self.history[0] == vec![Action::Call, Action::Call] && self.history[1] == vec![Action::Bet_1, Action::Bet_1] {
        //     println!("YYOOOO 0")
        // }
        // if self.private_hands[1] == vec![card_from_string("Qh".to_string())] && self.community_cards == vec![card_from_string("Jd".to_string())] {
        //     println!("YYOOOO 1")
        // }
        // If anyone folded at any point, it's terminal
        if self.history.iter()
            .map(|round_history| {
                if round_history.iter().contains(&Action::Fold) {
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

    fn get_payoffs(&self) -> Vec<i32> {
        // All but 1 folded
        if self.history.concat().contains(&Action::Fold) {
            let mut folded_player_index: usize = usize::MIN;
            for i in 0..self.history.len() {
                for (j, action) in self.history[i].iter().enumerate() {
                    if action == &Action::Fold {
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

        return payoffs
    }


    fn handle_action(&self, action: Action) -> Self {
        let mut new_bets = self.bets.clone();
        let active_player_index = self.get_active_player_index();

        let active_player_current_round_bet = self.bets[self.round][active_player_index];
        let opponent_current_round_bet = self.bets[self.round][(active_player_index + 1) % 2];

        if action != Action::Fold {
            // Always match the opponent bet first
            let mut bet_increase_amount: usize = opponent_current_round_bet - active_player_current_round_bet;
    
            if action == Action::Bet {
                let mut raise_amount = 2;
                if self.round == POST_FLOP_INDEX {
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
            community_cards: self.community_cards.clone()
        };
        next_state.history[next_state.round].push(action);

        if next_state.round == PRE_FLOP_INDEX {
            // Transition to post-flop is necessary
            if next_state.all_players_checked() || next_state.bet_or_raise_finished() {
                next_state.round = POST_FLOP_INDEX
            }
        }

        return next_state
    }

    fn get_representation(
        &self,
        _hand_indexer_option: Option<&HandIndexer>,
        _abstraction_labels_per_round_option: Option<Vec<&Vec<u8>>>,
    ) -> Option<Vec<u8>> {
        let mut representation = self.private_hands[self.get_active_player_index()].clone();

        
        if self.round == POST_FLOP_INDEX {
            representation.extend(self.community_cards.clone());
        }

        for action in &self.history[PRE_FLOP_INDEX] {
            representation.push(action.as_value());
        }

        if self.round == POST_FLOP_INDEX {
            for action in &self.history[POST_FLOP_INDEX] {
                representation.push(action.as_value());
            }
        }

        return Some(representation)
    }
}

impl LPGameState {
    // Returns True if all players checked in the current round
    fn all_players_checked(&self) -> bool {
        let num_checked = self.history[self.round].iter().filter(|&action| action == &Action::Call).count();

        return num_checked == self.player_amount && !self.history[self.round].iter().any(|action| action == &Action::Bet)
    }

    // Determines if the current betting round is finished, i.e., a bet or raise has been called or everyone has folded.
    fn bet_or_raise_finished(&self) -> bool {
        for i in 0..self.history[self.round].len() {
            let reversed_index = self.history[self.round].len() - 1 - i;
            let reversed_action = &self.history[self.round][reversed_index];
            if reversed_action == &Action::Bet {
                if self.history[self.round][reversed_index..].len() == self.player_amount {
                    return true
                }
                break
            }
        }

        return false

    }
}