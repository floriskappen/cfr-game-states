use std::env;

use itertools::Itertools;
use rand::prelude::*;
use lazy_static::lazy_static;

use hand_isomorphism_rust::deck::{card_from_string, Card, RANK_TO_CHAR, SUIT_TO_CHAR};

use crate::abstraction::action_abstraction::BLUEPRINT_AVAILABLE_ACTIONS;
use crate::abstraction::action_translation::pseudo_harmonic_mapping_randomized;
use crate::game_states::base_game_state::GameState;
use crate::structs::{ActionType, Action};
use super::rank::rank_hand;

lazy_static! {
    pub static ref USE_ACTION_TRANSLATION: bool = env::var("USE_ACTION_TRANSLATION").is_ok();
}


const ROUND_PREFLOP: usize = 0;
const ROUND_FLOP: usize = 1;
const ROUND_TURN: usize = 2;
const ROUND_RIVER: usize = 3;

const STACK_SIZE: usize = 10_000;
const SMALL_BLIND: usize = 50;
const BIG_BLIND: usize = 100;

#[derive(Clone, Debug)]
pub struct NLTHGameState {
    pub round: usize,
    pub player_amount: usize,

    pub private_hands: Vec<Vec<Card>>,
    pub community_cards: Vec<Card>,
    pub stacks: Vec<usize>,
    pub bets: Vec<Vec<usize>>,

    pub previous_raise_amount: usize,
    pub history: Vec<Vec<Action>>,
    pub history_abstracted: Vec<Vec<Action>>,
    pub active_player_index: usize,
    pub folded_players: Vec<bool>,
    pub all_in_players: Vec<i32>,
    /*
        Each pot contains a bet amount per player. Each time a bet is made, it gets added to the newest pot.
        Initially there is just a single (main) pot.
        When a player goes all-in, a new pot created.
        It's important that for every pot we still keep track of which player made which bets. This way we can divide them evenly later.
    */
    pub pots: Vec<Vec<usize>>,
    // Keeping track of active_player_amount in a variable is quicker than performing the necessary Vec loops to get this number each time
    pub active_player_amount: usize,
}

impl GameState for NLTHGameState {
    fn new_empty(player_amount: usize, draw_cards: bool, rng_seed: Option<u64>) -> Self {
        let private_hands;
        let community_cards;
        if draw_cards {
            let mut deck = Vec::new();
    
            for &rank in RANK_TO_CHAR.iter() {
                for &suit in SUIT_TO_CHAR.iter() {
                    let card = card_from_string(format!("{}{}", rank, suit));
                    deck.push(card);
                }
            }
    
            let mut rng = if let Some(seed) = rng_seed {
                StdRng::seed_from_u64(seed)
            } else {
                StdRng::seed_from_u64(thread_rng().next_u64())
            };
            deck.shuffle(&mut rng);
    
            // Draw 2 cards for each player + 5 community cards
            let drawn_items: Vec<Card> = deck.into_iter().take(
                (2 * player_amount) + 5
            ).collect();

            private_hands = (0..player_amount).map(|i| {
                return vec![
                    drawn_items[i*2], drawn_items[(i*2)+1]
                ]
            }).collect::<Vec<Vec<Card>>>();
            community_cards = drawn_items[drawn_items.len() - 5..].to_vec();
        } else {
            private_hands = (0..player_amount).map(|_| vec![]).collect();
            community_cards = vec![];
        }

        let blinds = (0..player_amount).map(|player_index| {
            if player_index == 0 {
                return SMALL_BLIND
            } else if player_index == 1 {
                return BIG_BLIND
            }
            return 0
        }).collect::<Vec<_>>();

        return NLTHGameState {
            round: ROUND_PREFLOP,
            player_amount,

            private_hands,
            community_cards,
            stacks: (0..player_amount).map(|i| STACK_SIZE - blinds[i]).collect(),
            bets: vec![
                (0..player_amount).map(|i| blinds[i]).collect(),
                vec![0; player_amount], // Flop
                vec![0; player_amount], // Turn
                vec![0; player_amount], // River
            ],

            previous_raise_amount: BIG_BLIND,
            history: vec![
                vec![], vec![], vec![], vec![]
            ],
            history_abstracted: vec![
                vec![], vec![], vec![], vec![]
            ],
            // In headsup poker, the small blind acts first preflop. Postflop the big blind acts first
            // In 3+ player poker, in the preflop round the FTA is the player after the big blind, so in our case player at index 2 (player 3)
            active_player_index: if player_amount == 2 { 0 } else { 2 },
            folded_players: vec![false; player_amount],
            all_in_players: vec![-1; player_amount],
            pots: vec![
                (0..player_amount).map(|i| blinds[i]).collect(), // Main pot, more will be added if someone goes all-in
            ],
            active_player_amount: player_amount
        }
    }

    fn get_total_rounds() -> usize {
        return 4;
    }

    fn get_current_round_index(&self) -> usize {
        return self.round;
    }

    fn get_player_amount(&self) -> usize {
        return self.player_amount;
    }

    fn get_active_player_index(&self) -> usize {
        return self.active_player_index;
    }

    fn get_history(&self) -> &Vec<Vec<Action>> {
        return &self.history;
    }

    fn abstract_history(&mut self) {
        self.history = self.history_abstracted.clone();
    }

    /*
        0 => None, until the game ends
        1 => Until the second round starts
        2 => Until the third round starts or until a second raise action 
    */
    fn is_leaf_node(&self, leaf_node_placement: usize) -> bool {
        if leaf_node_placement == 1 && self.round > 0 {
            return true;
        } else if leaf_node_placement == 2 && (
            self.round > 2 || self.get_current_round_bet_raise_amount() > 1
        ) {
            return true;
        }
        return false;
    }

    fn get_current_round_bet_raise_amount(&self) -> usize {
        return self.history[self.round].iter().filter(|&action| action.is_bet_raise()).count();
    }

    fn get_active_player_actions(&self, available_actions: &Vec<Action>) -> Vec<Action> {
        let pot = self.get_total_pot();

        return available_actions.iter().filter_map(|&action| {
            if action.action_type == ActionType::Fold {
                return Some(action)
            };

            // We should be able to afford it
            let previous_bet_cover_cost = self.get_call_amount();


            if action.action_type == ActionType::Call {
                // We need to have chips left after calling, otherwise it would be an all-in
                if self.stacks[self.active_player_index] as i32 - previous_bet_cover_cost as i32 > 0 {
                    return Some(action)
                }
                return None
            };

            if action.action_type == ActionType::AllIn {
                if self.stacks[self.active_player_index] as i32 - previous_bet_cover_cost as i32 >= 0 {
                    return Some(action)
                }
                return None
            };

            // ...and if it's a bet action_type, it has to be equal or more than the previous raise amount
            let raise_amount = (pot as f32 * action.get_multiplier()) as usize;
            let is_more_or_equal_previous_raise_amount = if raise_amount == 0 {
                true
            } else {
                raise_amount >= self.previous_raise_amount
            };

            let total_cost = previous_bet_cover_cost + raise_amount;

            let can_afford = self.stacks[self.active_player_index] as i32 - total_cost as i32 >= 0;

            if is_more_or_equal_previous_raise_amount && can_afford {
                return Some(action)
            }

            return None
        }).collect::<Vec<Action>>();
    }

    fn is_terminal(&self) -> bool {
        // No more active players (all folded and went all-in)
        if self.active_player_amount == 0 {
            return true
        }

        // All but 1 folded
        if self.folded_players.iter().filter(|&value| value == &true).count() == self.player_amount-1 {
            return true
        }

        // If there are no active players left (indicating that the rest went all-in and folded) and the last player responded
        if self.active_player_amount < 2 && (self.all_remaining_players_checked() || self.bet_or_raise_finished()) {
            return true
        }

        // Last round and everyone acted
        if self.round == ROUND_RIVER && (self.all_remaining_players_checked() || self.bet_or_raise_finished()) {
            return true
        }

        return false
    }

    fn get_payoffs(&self) -> Vec<i32> {
        // All but 1 folded. No need to deal with all-ins and multiple pots because if there was an all-in not everyone has folded
        if self.folded_players.iter().filter(|&value| value == &true).count() == self.player_amount-1 {
            let winning_player_index = self.folded_players.iter().enumerate().find(|(_, &value)| value == false).unwrap().0;
            
            let payoffs: Vec<i32> = (0..self.player_amount).map(|player_index| {
                if player_index == winning_player_index {
                    return self.get_total_pot() as i32 - self.bets.iter().map(|round_bets| round_bets[winning_player_index]).sum::<usize>() as i32;
                }

                return -self.pots.iter().map(|round_pots| round_pots[player_index] as i32).sum::<i32>();
            }).collect();

            return payoffs;
        }

        // // Showdown // //
        let mut payoffs: Vec<i32> = vec![0; self.player_amount];

        // Iterate through all pots and divide them amongst eligible players
        for (pot_index, pot) in self.pots.iter().enumerate() {
            let pot_sum = pot.iter().sum::<usize>();
            // Decide who can contest the pot
            let participating_player_indices = (0..self.player_amount).filter_map(|player_index| {
                if
                    // Players that folded at any point cannot contest the pot (their losses will be calculated later)
                    self.folded_players[player_index] ||
                    // Neither can players who went all-in in a previous pot
                    self.all_in_players[player_index] > pot_index as i32
                { return None; }

                return Some(player_index);
            }).collect::<Vec<usize>>();

            // Calculate hand ranks for the players that can contest this pot
            let participating_player_hand_ranks = (0..self.player_amount).filter_map(|player_index| {
                if participating_player_indices.contains(&player_index) {
                    let mut hand = self.private_hands[player_index].clone();
                    hand.extend(self.community_cards.clone());
                    let rank = rank_hand(hand);
                    return Some(rank)
                }
                return None;
            }).collect::<Vec<_>>();

            let highest_hand_rank = participating_player_hand_ranks.iter().sorted().rev().next().unwrap();
            // Grab the index of the players with the highest rank. This way we account for ties
            let winning_player_indices = participating_player_hand_ranks.iter()
                .enumerate()
                .filter(|(_, rank)| rank == &highest_hand_rank)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();

            for player_index in participating_player_indices {
                if winning_player_indices.contains(&player_index) {
                    payoffs[player_index] += (pot_sum / winning_player_indices.len()) as i32 - pot[player_index] as i32
                } else {
                    payoffs[player_index] -= pot[player_index] as i32
                }
            }
        }

        // Calculate losses for players who folded as they were not accounted for in the pot division
        for player_index in 0..self.player_amount {
            if self.folded_players[player_index] {
                payoffs[player_index] -= self.pots.iter().map(|pot| pot[player_index]).sum::<usize>() as i32
            }
        }

        return payoffs;
    }

    fn can_proceed_to_next_round(&self) -> bool {
        if self.round < ROUND_RIVER && self.active_player_amount > 1 && (self.all_remaining_players_checked() || self.bet_or_raise_finished()) {
            return true;
        }

        return false;
    }

    fn handle_action(&self, action: Action) -> Self {
        // Save abstracted action
        let abstracted_action =
            if *USE_ACTION_TRANSLATION && // Should be optional as it is not necessary during blueprint calculation
            !BLUEPRINT_AVAILABLE_ACTIONS[self.round][self.get_current_round_bet_raise_amount()].contains(&action)
        {
            // Use randomized pseudo-harmonic mapping for action translation
            println!("Using action translation");
            let mut closest_lower: Option<Action>  = None;
            let mut closest_upper: Option<Action> = None;
            for &abstracted_action in BLUEPRINT_AVAILABLE_ACTIONS[self.round][self.get_current_round_bet_raise_amount()].iter() {
                if abstracted_action.raise_amount < action.raise_amount && (
                    closest_lower.is_some_and(|closest_lower| closest_lower.raise_amount < abstracted_action.raise_amount)
                ) {
                    closest_lower = Some(abstracted_action);
                } else if abstracted_action.raise_amount > action.raise_amount && (
                    closest_upper.is_some_and(|closest_upper| closest_upper.raise_amount > abstracted_action.raise_amount)
                ) {
                    closest_upper = Some(abstracted_action);
                }
            }

            if closest_lower.is_none() && closest_upper.is_some() {
                // If there's only an upper bound, we use that one
                closest_upper.unwrap()
            } else if closest_upper.is_none() && closest_lower.is_some() {
                // If there's only a lower bound, we use that one
                closest_lower.unwrap()
            } else {
                // Use randomized pseudo-harmonic mapping which will return either the lower or upper raise_amount as f64
                let lower = closest_lower.unwrap();
                let upper = closest_upper.unwrap();
                let mapped_raise_amount = pseudo_harmonic_mapping_randomized(
                    action.raise_amount as f64,
                    lower.raise_amount as f64,
                    upper.raise_amount as f64
                );
                Action { action_type: action.action_type, raise_amount: mapped_raise_amount as u16 }
            }
        } else {
            action
        };

        let mut next_state = self.clone();

        if action.action_type == ActionType::Fold {
            next_state.folded_players[next_state.active_player_index] = true;

            // The player becomes inactive from this point on
            next_state.active_player_amount -= 1;
        } else {
            let mut bet_increase_amount = next_state.get_call_amount();

            if action.is_bet_raise() {
                let current_pot = next_state.get_total_pot();
                let new_raise_amount = (current_pot as f32 * action.get_multiplier()).round() as usize;
                bet_increase_amount += new_raise_amount;
                next_state.previous_raise_amount = new_raise_amount;
            } else if action.action_type == ActionType::AllIn {
                // Everything left in the player's stack
                let new_raise_amount = next_state.stacks[next_state.active_player_index] - bet_increase_amount;
                bet_increase_amount += new_raise_amount;
                next_state.previous_raise_amount = new_raise_amount;

                // Set the value on the index of the active player to the current pot
                next_state.all_in_players[next_state.active_player_index] = (next_state.pots.len() as i32) -1;

                // The player becomes inactive from this point on
                next_state.active_player_amount -= 1;
            }

            next_state.stacks[next_state.active_player_index] -= bet_increase_amount;
            next_state.bets[next_state.round][next_state.active_player_index] += bet_increase_amount;
            let pot_amount = next_state.pots.len();
            next_state.pots[pot_amount-1][next_state.active_player_index] += bet_increase_amount;

            // If the action_type was an all-in, we create a new pot
            if action.action_type == ActionType::AllIn {
                next_state.pots.push(vec![0; next_state.player_amount])
            }
        }

        next_state.history[next_state.round].push(action);
        next_state.history_abstracted[next_state.round].push(abstracted_action);

        // Set the new active player index
        let mut current_new_active_player_index = (next_state.active_player_index + 1) % next_state.player_amount;
        for _ in 0..next_state.player_amount {
            // The player should not have folded or have gone all-in in order to be active
            if !next_state.folded_players[current_new_active_player_index] && next_state.all_in_players[current_new_active_player_index] == -1 {
                next_state.active_player_index = current_new_active_player_index;
                break;
            }
            current_new_active_player_index = (current_new_active_player_index + 1) % next_state.player_amount;
        }

        if next_state.can_proceed_to_next_round() {
            // Transition to next round
            next_state.round += 1;
            next_state.previous_raise_amount = BIG_BLIND;
            if next_state.player_amount == 2 {
                // In heads-up poker the big blind (player 2) acts first post-flop
                next_state.active_player_index = 1;
            } else {
                // Otherwise the small blind (player 1) acts first
                next_state.active_player_index = 0;
            }
        }

        return next_state;
    }
}

impl NLTHGameState {
    pub fn get_total_pot(&self) -> usize {
        return self.pots.iter().map(|pot| pot.iter().sum::<usize>()).sum();
    }

    pub fn get_call_amount(&self) -> usize {
        let mut call_amount = 0;
        let option = self.bets[self.round].iter().sorted().rev().next();
        if let Some(&highest_bet) = option {
            let active_player_current_round_bet = self.bets[self.round][self.active_player_index];
            call_amount = highest_bet - active_player_current_round_bet;
        }

        return call_amount
    }

    // Returns True if all remaining (not folded) players checked in the current round (noone bet or raised)
    // TODO: Test if this works correctly in all situations:
    // - Check, Check, Fold, Check, Check, Check -> True
    // - Bet, Fold, Raise, Call, Call, Call -> False
    // - Check, Check, Bet, Call, Fold, Fold, Fold, Call -> False
    pub fn all_remaining_players_checked(&self) -> bool {
        let num_checked = self.history[self.round].iter().filter(|&action| action.action_type == ActionType::Call).count();

        return num_checked == self.active_player_amount &&
            !self.history[self.round].iter().any(|action| action.is_bet_raise())
    }

    // Determines if the current betting round is finished, i.e., a bet or raise has been called or everyone has folded.
    fn bet_or_raise_finished(&self) -> bool {
        for i in 0..self.history[self.round].len() {
            let reversed_index = self.history[self.round].len() - 1 - i;
            let reversed_action_with_raise = &self.history[self.round][reversed_index];

            if reversed_action_with_raise.is_bet_raise() {
                if self.history[self.round][reversed_index..]
                    .iter()
                    .filter(|&action| action.action_type != ActionType::Fold) // Exclude fold actions as they have impact on active_player_amount
                    .count() == self.active_player_amount
                {
                    return true
                }
                break
            }

            if reversed_action_with_raise.action_type == ActionType::AllIn {
                // When someone went all-in the active player amount is reduced by 1 so we need to check if it is bigger
                if self.history[self.round][reversed_index..]
                    .iter()
                    .filter(|&action| action.action_type != ActionType::Fold) // Exclude fold actions as they have impact on active_player_amount
                    .count() > self.active_player_amount
                {
                    return true
                }
                break
            }
        }
        return false
    }
}
