use hand_isomorphism_rust::deck::{card_from_string, Card};
use rand::prelude::*;
use lazy_static::lazy_static;

use crate::{game_states::base_game_state::GameState, structs::{ActionType, ActionWithRaise}};

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
    pub private_hands: Vec<Card>,
    pub history: Vec<Vec<ActionWithRaise>>,
    pub bets: Vec<usize>
}

impl GameState for KPGameState {
    fn new_empty(_player_amount: usize, rng_seed: Option<u64>) -> Self {
        let mut rng = if let Some(seed) = rng_seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::seed_from_u64(thread_rng().next_u64())
        };

        let mut shuffled_cards = DECK.to_vec();
        shuffled_cards.shuffle(&mut rng);

        // Draw 2 items
        let drawn_items: Vec<Card> = shuffled_cards.iter().take(2).cloned().collect();
        
        return KPGameState {
            player_amount: 2,
            private_hands: vec![drawn_items[0], drawn_items[1]],
            history: vec![vec![]],
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

    fn get_history(&self) -> &Vec<Vec<ActionWithRaise>> {
        return &self.history;
    }

    fn is_leaf_node(&self, _subgame_end_situation: usize) -> bool {
        // There are never leaf nodes in Kuhn Poker
        return false;
    }

    fn get_current_round_bet_raise_amount(&self) -> usize {
        return self.history[0].iter().filter(|action_with_raise| action_with_raise.action_type == ActionType::Bet).count();
    }

    fn is_terminal(&self) -> bool {
        let terminal_histories = vec![
            vec![ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1 }, ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 }],
            vec![ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1 }, ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }],
            vec![ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }],
            vec![ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1 }, ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }],
            vec![ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1 }, ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 }],
        ];

        if terminal_histories.contains(&self.history[0]) {
            return true
        }

        return false
    }

    fn get_payoffs(&self) -> Vec<i32> {
        let winning_player_identifier: usize;

        if let Some((i, _)) = self.history[0].iter().enumerate().find(|(_, &action)| action == ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 }) {
            winning_player_identifier = (i+1) % 2;
        } else {
            // Showoff
            if self.private_hands[0] > self.private_hands[1] {
                winning_player_identifier = 0;
            } else {
                winning_player_identifier = 1;
            }
        }

        let winning_player_payoff = self.bets[(winning_player_identifier + 1) % 2];
        let mut payoffs = vec![winning_player_payoff as i32, winning_player_payoff as i32];
        payoffs[(winning_player_identifier + 1) % 2] *= -1;

        return payoffs;
    }

    fn get_active_player_index(&self) -> usize {
        return self.history[0].len() % 2;
    }

    fn get_active_player_actions(&self, _available_actions: &Vec<ActionWithRaise>) -> Vec<ActionWithRaise> {
        if let Some(&previous_action) = self.history[0].iter().rev().next() {
            if previous_action.action_type == ActionType::Bet {
                return vec![ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }]
            }
        }

        return vec![ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 }, ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1 }]
    }

    fn can_proceed_to_next_round(&self) -> bool {
        // There is only 1 round in Kuhn Poker
        return false;
    }

    fn handle_action(&self, action: ActionWithRaise) -> Self {
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
