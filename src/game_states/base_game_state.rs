
use hand_isomorphism_rust::hand_indexer::HandIndexer;

use crate::structs::Action;

pub trait GameState {
    fn new_empty(player_amount: usize, seed: Option<u64>) -> Self;
    fn get_total_rounds() -> usize;
    fn get_player_amount(&self) -> usize;
    fn get_current_round_index(&self) -> usize;
    fn is_terminal(&self) -> bool;
    fn get_payoffs(&self) -> Vec<i32>;
    fn get_active_player_index(&self) -> usize;
    fn get_active_player_actions(&self) -> Vec<Action>;
    fn handle_action(&self, action: Action) -> Self;
    fn get_representation(&self, hand_indexer: Option<&HandIndexer>, abstraction_labels_per_round: Option<Vec<&Vec<u8>>>) -> Option<Vec<u8>>;
}
