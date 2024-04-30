use hand_isomorphism_rust::deck::{card_from_string, Card};
use rand::prelude::*;
use lazy_static::lazy_static;
use smallvec::{smallvec, SmallVec};

use crate::{constants::{COMMUNITY_CARD_AMOUNT, MAX_PLAYERS, NO_CARD_PLACEHOLDER, PRIVATE_CARD_AMOUNT, ROUNDS}, game_states::base_game_state::GameState, structs::{Action, ActionType}};

lazy_static! {
    static ref DECK: [Card; 3] = {
        [
            card_from_string("Kh".to_string()),
            card_from_string("Qh".to_string()),
            card_from_string("Jh".to_string()),
        ]
    };
}

#[derive(Clone, Debug)]
pub struct KPGameState {
    pub player_amount: usize,
    pub private_hands: [[Card; PRIVATE_CARD_AMOUNT]; MAX_PLAYERS],
    pub history: [SmallVec<[Action; 200]>; ROUNDS],
    pub bets: Vec<usize>
}

impl GameState for KPGameState {
    fn new_empty(_player_amount: usize, draw_cards: bool, rng_seed: Option<u64>) -> Self {
        let private_hands;
        if draw_cards {
            let mut rng = if let Some(seed) = rng_seed {
                StdRng::seed_from_u64(seed)
            } else {
                StdRng::seed_from_u64(thread_rng().next_u64())
            };
    
            let mut shuffled_cards = DECK.to_vec();
            shuffled_cards.shuffle(&mut rng);
    
            // Draw 2 items
            let drawn_items: Vec<Card> = shuffled_cards.iter().take(2).cloned().collect();
            private_hands = (0..MAX_PLAYERS).map(|i| {
                if i < 2 {
                    return [drawn_items[i], NO_CARD_PLACEHOLDER];
                }
                return [NO_CARD_PLACEHOLDER; 2];
            }).collect::<Vec<[Card; 2]>>().try_into().unwrap();
        } else {
            private_hands = (0..MAX_PLAYERS).map(|_| [NO_CARD_PLACEHOLDER; 2])
                .collect::<Vec<[Card; 2]>>().try_into().unwrap();
        }

        return KPGameState {
            player_amount: 2,
            private_hands,
            history: [
                SmallVec::new(), SmallVec::new(), SmallVec::new(), SmallVec::new()
            ],
            bets: vec![1, 1] // Default 1$ bet
        }
    }

    fn get_total_rounds() -> usize {
        return 1;
    }

    fn get_current_round_index(&self) -> usize {
        return 0;
    }

    fn get_player_amount(&self) -> usize {
        return self.player_amount;
    }

    fn get_history(&self) -> &[SmallVec<[Action; 200]>; 4] {
        return &self.history;
    }

    fn get_community_cards(&self) -> &[Card; COMMUNITY_CARD_AMOUNT] {
        return &[NO_CARD_PLACEHOLDER; COMMUNITY_CARD_AMOUNT] // Wrong placeholder value, is never used
    }

    fn set_community_cards(&mut self, _community_cards: [Card; COMMUNITY_CARD_AMOUNT]) {}

    fn get_private_hands(&self) -> &[[Card; PRIVATE_CARD_AMOUNT]; MAX_PLAYERS] {
        return &self.private_hands
    }

    fn set_private_hands(&mut self, private_hands: [[Card; PRIVATE_CARD_AMOUNT]; MAX_PLAYERS]) {
        self.private_hands = private_hands;
    }

    fn is_leaf_node(&self, _subgame_end_situation: u8) -> bool {
        // There are never leaf nodes in Kuhn Poker
        return false;
    }

    fn get_current_bet_count(&self) -> usize {
        return self.history[0].iter().filter(|action| action.action_type == ActionType::Bet).count() as usize;
    }

    fn is_terminal(&self) -> bool {
        let terminal_histories: Vec<SmallVec<[Action; 200]>> = vec![
            smallvec![Action { action_type: ActionType::Bet, raise_amount: 0 }, Action { action_type: ActionType::Fold, raise_amount: 0 }],
            smallvec![Action { action_type: ActionType::Bet, raise_amount: 0 }, Action { action_type: ActionType::Call, raise_amount: 0 }],
            smallvec![Action { action_type: ActionType::Call, raise_amount: 0 }, Action { action_type: ActionType::Call, raise_amount: 0 }],
            smallvec![Action { action_type: ActionType::Call, raise_amount: 0 }, Action { action_type: ActionType::Bet, raise_amount: 0 }, Action { action_type: ActionType::Call, raise_amount: 0 }],
            smallvec![Action { action_type: ActionType::Call, raise_amount: 0 }, Action { action_type: ActionType::Bet, raise_amount: 0 }, Action { action_type: ActionType::Fold, raise_amount: 0 }],
        ];

        if terminal_histories.contains(&self.history[0]) {
            return true
        }

        return false
    }

    fn get_payoffs(&self) -> [i32; MAX_PLAYERS] {
        let winning_player_identifier: usize;

        if let Some((i, _)) = self.history[0].iter().enumerate().find(|(_, &action)| action == Action { action_type: ActionType::Fold, raise_amount: 0 }) {
            winning_player_identifier = (i+1) % 2;
        } else {
            // Showoff
            if self.private_hands[0][0] > self.private_hands[1][0] {
                winning_player_identifier = 0;
            } else {
                winning_player_identifier = 1;
            }
        }

        let winning_player_payoff = self.bets[(winning_player_identifier + 1) % 2];
        let mut payoffs: [i32; 6] = (0..MAX_PLAYERS).map(|i| {
            if i < 2 {
                return winning_player_payoff as i32;
            }
            return 0;
        }).collect::<Vec<i32>>().try_into().unwrap();
        payoffs[(winning_player_identifier + 1) % 2] *= -1;

        return payoffs;
    }

    fn get_active_player_index(&self) -> usize {
        return self.history[0].len() % 2;
    }

    fn get_active_player_actions(&self, _actions_in_abstraction: Option<&SmallVec<[Action; 40]>>) -> SmallVec<[Action; 40]> {
        if let Some(&previous_action) = self.history[0].iter().rev().next() {
            if previous_action.action_type == ActionType::Bet {
                return smallvec![Action { action_type: ActionType::Fold, raise_amount: 0 }, Action { action_type: ActionType::Call, raise_amount: 0 }]
            }
        }

        return smallvec![Action { action_type: ActionType::Call, raise_amount: 0 }, Action { action_type: ActionType::Bet, raise_amount: 0 }]
    }

    fn can_proceed_to_next_round(&self) -> bool {
        // There is only 1 round in Kuhn Poker
        return false;
    }

    fn handle_action(&self, action: Action) -> Self {
        let mut new_bets = self.bets.clone();

        let active_player_index = self.get_active_player_index();

        let active_player_current_round_bet = self.bets[active_player_index];
        let opponent_current_round_bet = self.bets[(active_player_index + 1) % 2];
        
        if action.action_type == ActionType::Bet || action.action_type == ActionType::Call {
            // Always match the opponent bet first
            let mut bet_increase_amount: usize = opponent_current_round_bet - active_player_current_round_bet;

            if action.action_type == ActionType::Bet {
                bet_increase_amount += 1;
            }

            new_bets[active_player_index] += bet_increase_amount;
        }

        let mut next_state = KPGameState {
            player_amount: self.player_amount,
            private_hands: self.private_hands.clone(),
            history: self.history.clone(),
            bets: new_bets,
        };

        next_state.history[0].push(action);

        return next_state
    }
}
