use std::env;

use itertools::Itertools;
use rand::prelude::*;
use lazy_static::lazy_static;

use hand_isomorphism_rust::deck::{card_from_string, Card, RANK_TO_CHAR, SUIT_TO_CHAR};
use smallvec::{smallvec, SmallVec};

use crate::constants::{COMMUNITY_CARD_AMOUNT, MAX_PLAYERS, NO_CARD_PLACEHOLDER, PRIVATE_CARD_AMOUNT, ROUNDS};
use crate::game_states::base_game_state::GameState;
use crate::structs::{ActionType, Action};
use super::rank::rank_hand;

lazy_static! {
    pub static ref USE_ACTION_TRANSLATION: bool = env::var("USE_ACTION_TRANSLATION").is_ok();
}

const ROUND_PREFLOP: usize = 0;
const _ROUND_FLOP: usize = 1;
const _ROUND_TURN: usize = 2;
const ROUND_RIVER: usize = 3;

const STACK_SIZE: u32 = 10_000;
const SMALL_BLIND: u32 = 50;
const BIG_BLIND: u32 = 100;

#[derive(Clone, Debug)]
pub struct NLTHGameState {
    pub round: usize, // Used for indexing so it's usize
    pub player_amount: usize, // Used for indexing so it's usize

    pub private_hands: [[Card; PRIVATE_CARD_AMOUNT]; MAX_PLAYERS],
    pub community_cards: [Card; COMMUNITY_CARD_AMOUNT],
    pub stacks: [u32; MAX_PLAYERS],
    pub bets: [[u32; MAX_PLAYERS]; ROUNDS],
    pub minimum_raise_amount: u32,

    pub history: [SmallVec<[Action; 200]>; ROUNDS],
    pub active_player_index: usize, // Used for indexing so it's usize
    pub folded_players: [bool; MAX_PLAYERS],
    pub all_in_players: [i32; MAX_PLAYERS],
    /*
        Each pot contains a bet amount per player. Each time a bet is made, it gets added to the newest pot.
        Initially there is just a single (main) pot.
        When a player goes all-in, a new pot created.
        It's important that for every pot we still keep track of which player made which bets. This way we can divide them evenly later.
    */
    pub pots: [[u32; MAX_PLAYERS]; MAX_PLAYERS], // There cannot be more than MAX_PLAYERS pots
    pub current_round_pot_all_in_amounts: [u32; MAX_PLAYERS],
    pub current_pot: usize, // Used for indexing so it's usize
    // Keeping track of active_player_amount in a variable is quicker than performing the necessary Vec loops to get this number each time
    pub active_player_amount: u8,
}

impl GameState for NLTHGameState {
    fn new_empty(player_amount: usize, draw_cards: bool, rng_seed: Option<u64>) -> Self {
        let private_hands: [[Card; 2]; MAX_PLAYERS];
        let community_cards: [Card; 5];
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
            private_hands = (0..MAX_PLAYERS).map(|i| {
                if i < player_amount {
                    return [
                        drawn_items[i*2], drawn_items[(i*2)+1]
                    ]
                }
                return [NO_CARD_PLACEHOLDER, NO_CARD_PLACEHOLDER]
            }).collect::<Vec<[Card; 2]>>().try_into().unwrap();

            community_cards = drawn_items[drawn_items.len() - 5..].to_vec().try_into().unwrap();
        } else {
            private_hands = (0..MAX_PLAYERS).map(|_| [NO_CARD_PLACEHOLDER, NO_CARD_PLACEHOLDER]).collect::<Vec<[Card; 2]>>().try_into().unwrap();
            community_cards = [NO_CARD_PLACEHOLDER; 5];
        }

        let blinds = (0..MAX_PLAYERS).map(|player_index| {
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
            stacks: (0..MAX_PLAYERS).map(|i| STACK_SIZE - blinds[i]).collect::<Vec<u32>>().try_into().unwrap(),
            bets: [
                (0..MAX_PLAYERS).map(|i| blinds[i]).collect::<Vec<u32>>().try_into().unwrap(),
                [0; 6], // Flop
                [0; 6], // Turn
                [0; 6], // River
            ],
            minimum_raise_amount: BIG_BLIND,

            history: [
                SmallVec::new(), SmallVec::new(), SmallVec::new(), SmallVec::new()
            ],
            // In headsup poker, the small blind acts first preflop. Postflop the big blind acts first
            // In 3+ player poker, in the preflop round the FTA is the player after the big blind, so in our case player at index 2 (player 3)
            active_player_index: if player_amount == 2 { 0 } else { 2 },
            folded_players: [false; MAX_PLAYERS],
            all_in_players: [-1; MAX_PLAYERS],
            pots: (0..MAX_PLAYERS).map(|i| {
                (0..MAX_PLAYERS).map(|j| {
                    if i == 0 {
                        return blinds[j];
                    }
                    return 0
                }).collect::<Vec<u32>>().try_into().unwrap()
            }).collect::<Vec<[u32; MAX_PLAYERS]>>().try_into().unwrap(),
            current_round_pot_all_in_amounts: [0; MAX_PLAYERS],
            current_pot: 0,
            active_player_amount: player_amount as u8
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

    fn get_history(&self) -> &[SmallVec<[Action; 200]>; ROUNDS] {
        return &self.history;
    }

    fn get_community_cards(&self) -> &[Card; COMMUNITY_CARD_AMOUNT] {
        return &self.community_cards
    }

    fn set_community_cards(&mut self, community_cards: [Card; COMMUNITY_CARD_AMOUNT]) {
        self.community_cards = community_cards;
    }

    fn get_private_hands(&self) -> &[[Card; PRIVATE_CARD_AMOUNT]; MAX_PLAYERS] {
        return &self.private_hands
    }

    fn set_private_hands(&mut self, private_hands: [[Card; PRIVATE_CARD_AMOUNT]; MAX_PLAYERS]) {
        self.private_hands = private_hands;
    }

    /*
        0 => None, until the game ends
        1 => Until the second round starts
        2 => Until the third round starts or until a second raise action 
    */
    fn is_leaf_node(&self, leaf_node_placement: u8) -> bool {
        if leaf_node_placement == 1 && self.round > 0 {
            return true;
        } else if leaf_node_placement == 2 && (
            self.round > 2 || self.get_current_bet_count() > 1
        ) {
            return true;
        }
        return false;
    }

    fn get_current_bet_count(&self) -> usize {
        return self.history[self.round].iter().filter(|&action| action.is_bet_raise()).count();
    }

    fn get_active_player_actions(&self, bets_in_abstraction_option: Option<&SmallVec<[Action; 40]>>) -> SmallVec<[Action; 40]> {
        let pot = self.get_total_pot();
        let mut actions_in_abstraction: SmallVec<[Action; 40]> = smallvec![
            Action {action_type: ActionType::Fold, raise_amount: 0 },
            Action {action_type: ActionType::Call, raise_amount: 0 },
            Action {action_type: ActionType::AllIn, raise_amount: 0 },
        ];

        if let Some(bets_in_abstraction) = bets_in_abstraction_option {
            actions_in_abstraction.extend(bets_in_abstraction.clone())
        }

        return actions_in_abstraction.into_iter().filter_map(|action| {
            // Going all-in is always an option
            if action.action_type == ActionType::AllIn {
                return Some(action);
            };

            let call_amount = self.get_call_amount();

            if action.action_type == ActionType::Fold {
                if call_amount == 0 {
                    // We shouldn't have the option to fold if we don't need to call any amount
                    return None;
                }
                return Some(action);
            };

            if action.action_type == ActionType::Call {
                // We need to have chips left after calling, otherwise it would be an all-in
                if self.stacks[self.active_player_index] as i32 - call_amount as i32 > 0 {
                    return Some(action);
                }
                return None;
            };

            // ...and if it's a bet action_type, it has to be equal or more than the previous raise amount
            let raise_amount = ((pot + call_amount) as f32 * action.get_multiplier()) as u32;

            // The the raise amount should be at least twice the size of the previous raise
            if raise_amount < self.minimum_raise_amount {
                return None;
            }

            // We should also be able to afford it
            if (self.stacks[self.active_player_index] as i32 - raise_amount as i32) < 0 {
                return None;
            }
            
            return Some(action);
        }).collect::<SmallVec<[Action; 40]>>();
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

    fn get_payoffs(&self) -> [i32; MAX_PLAYERS] {
        // All but 1 folded. No need to deal with all-ins and multiple pots because if there was an all-in not everyone has folded
        if self.folded_players.iter().filter(|&value| value == &true).count() == self.player_amount-1 {
            let winning_player_index = self.folded_players.iter().enumerate().find(|(_, &value)| value == false).unwrap().0;
            
            let payoffs: [i32; MAX_PLAYERS] = (0..MAX_PLAYERS).map(|player_index| {
                if player_index > self.player_amount { return 0 }

                if player_index == winning_player_index {
                    return self.get_total_pot() as i32 - self.bets.iter().map(|round_bets| round_bets[winning_player_index]).sum::<u32>() as i32;
                }

                return -self.pots.iter().map(|round_pots| round_pots[player_index] as i32).sum::<i32>();
            }).collect::<Vec<i32>>().try_into().unwrap();

            return payoffs;
        }

        // // Showdown // //
        let mut payoffs = [0; MAX_PLAYERS];

        // Iterate through all pots and divide them amongst eligible players
        for pot in self.pots.iter(){
            let pot_sum = pot.iter().sum::<u32>();
            // Decide who can contest the pot
            let participating_player_indices = (0..self.player_amount).filter_map(|player_index| {
                if
                    player_index > self.player_amount-1 ||
                    // Players that folded at any point cannot contest the pot (their losses will be calculated later)
                    self.folded_players[player_index]
                { return None; }

                return Some(player_index);
            }).collect::<Vec<usize>>();

            if participating_player_indices.len() == 0 {
                continue
            }

            // Calculate hand ranks for the players that can contest this pot
            let participating_player_hand_ranks = (0..self.player_amount).filter_map(|player_index| {
                if participating_player_indices.contains(&player_index) {
                    let mut hand = self.private_hands[player_index].to_vec();
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
                    payoffs[player_index] += (pot_sum / winning_player_indices.len() as u32) as i32 - pot[player_index] as i32
                } else {
                    payoffs[player_index] -= pot[player_index] as i32
                }
            }
        }

        // Calculate losses for players who folded as they were not accounted for in the pot division
        for player_index in 0..self.player_amount {
            if self.folded_players[player_index] {
                payoffs[player_index] -= self.pots.iter().map(|pot| pot[player_index]).sum::<u32>() as i32
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
        let mut next_state = self.clone();

        if action.action_type == ActionType::Fold {
            next_state.folded_players[next_state.active_player_index] = true;

            // The player becomes inactive from this point on
            next_state.active_player_amount -= 1;
        } else {
            let current_bets = next_state.bets[next_state.round][next_state.active_player_index];
            let call_amount = next_state.get_call_amount();
            let mut extra_bets = call_amount;

            if action.action_type == ActionType::AllIn {
                // The all-in is equal or more than the minimum raise amount
                if next_state.stacks[next_state.active_player_index] >= next_state.minimum_raise_amount * 2 {
                    // Set the minimum raise amount to the all-in amount
                    next_state.minimum_raise_amount = next_state.stacks[next_state.active_player_index] - call_amount;
                }

                extra_bets += next_state.stacks[next_state.active_player_index] - call_amount;

                // The all-in is added to the current pot like normal, and a new pot is created
                let mut pot_bets_left = extra_bets;
                for (pot_index, &all_in_amount) in next_state.current_round_pot_all_in_amounts.iter().enumerate() {
                    if all_in_amount > current_bets {
                        let pot_bets = (all_in_amount - current_bets).min(pot_bets_left);
                        next_state.pots[pot_index][next_state.active_player_index] += pot_bets;
                        pot_bets_left -= pot_bets;
                    }
                }
                next_state.pots[next_state.current_pot][next_state.active_player_index] += pot_bets_left;
                next_state.current_round_pot_all_in_amounts[next_state.current_pot] = current_bets + extra_bets;

                // Set the value on the index of the active player to the current pot
                next_state.all_in_players[next_state.active_player_index] = next_state.current_pot as i32;

                next_state.current_pot += 1;

                // The player becomes inactive from this point on
                next_state.active_player_amount -= 1;
            } else {
                if action.is_bet_raise() {
                    let current_pot = next_state.get_total_pot();
                    let raise_amount = ((current_pot + call_amount) as f32 * action.get_multiplier()) as u32;
                    extra_bets += raise_amount;
                    next_state.minimum_raise_amount = raise_amount;
                }
                
                /*
                    Managing sidepots
                    If players have gone all-in this round, they are entitled to a portion of the pot up until the amount they went all-in with.
                    So we divide the extra_bets amongst the different pots that may be active at the moment.
                    The remainder gets sent to the main pot.
                */
                let mut pot_bets_left = extra_bets;
                // Add the new wager to the current pot, managing any sidepots that may be active
                for (pot_index, &all_in_amount) in next_state.current_round_pot_all_in_amounts.iter().enumerate() {
                    if all_in_amount > current_bets {
                        let pot_bets = (all_in_amount - current_bets).min(pot_bets_left);
                        next_state.pots[pot_index][next_state.active_player_index] += pot_bets;
                        pot_bets_left -= pot_bets;
                    }
                }
                next_state.pots[next_state.current_pot][next_state.active_player_index] += pot_bets_left;
            }

            // Decrease the wager from the player's stack
            next_state.stacks[next_state.active_player_index] -= extra_bets;
            // Add the wager to the player's bet amount for the round
            next_state.bets[next_state.round][next_state.active_player_index] += extra_bets;
        }

        next_state.history[next_state.round].push(action);

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
            next_state.minimum_raise_amount = BIG_BLIND;
            // We don't have to keep track of sidepots created during this round anymore
            next_state.current_round_pot_all_in_amounts = [0; MAX_PLAYERS];
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
    pub fn get_total_pot(&self) -> u32 {
        return self.pots.iter().map(|pot| pot.iter().sum::<u32>()).sum();
    }

    pub fn get_call_amount(&self) -> u32 {
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

        return num_checked == self.active_player_amount.into() &&
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
                    .count() == self.active_player_amount.into()
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
                    .count() > self.active_player_amount.into()
                {
                    return true
                }
                break
            }
        }
        return false
    }
}
