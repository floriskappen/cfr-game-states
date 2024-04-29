use hand_isomorphism_rust::deck::Card;
use smallvec::SmallVec;

use crate::{constants::{COMMUNITY_CARD_AMOUNT, MAX_PLAYERS, PRIVATE_CARD_AMOUNT, ROUNDS}, structs::Action};

pub trait GameState {
    fn new_empty(player_amount: usize, draw_cards: bool, seed: Option<u64>) -> Self;
    fn get_total_rounds() -> usize;
    fn get_player_amount(&self) -> usize;
    fn get_current_round_bet_raise_amount(&self) -> usize;
    fn get_history(&self) -> &[SmallVec<[Action; 200]>; ROUNDS];
    fn get_community_cards(&self) -> &[Card; COMMUNITY_CARD_AMOUNT];
    fn set_community_cards(&mut self, community_cards: [Card; COMMUNITY_CARD_AMOUNT]);
    fn get_private_hands(&self) -> &[[Card; PRIVATE_CARD_AMOUNT]; MAX_PLAYERS];
    fn set_private_hands(&mut self, private_hands: [[Card; PRIVATE_CARD_AMOUNT]; MAX_PLAYERS]);
    fn is_leaf_node(&self, subgame_end_situation: u8) -> bool;
    fn get_current_round_index(&self) -> usize;
    fn is_terminal(&self) -> bool;
    fn get_payoffs(&self) -> [i32; MAX_PLAYERS];
    fn get_active_player_index(&self) -> usize;
    fn get_active_player_actions(&self, available_actions: SmallVec<[Action; 40]>) -> SmallVec<[Action; 40]>;
    fn handle_action(&self, action: Action) -> Self;
    fn can_proceed_to_next_round(&self) -> bool;
}
