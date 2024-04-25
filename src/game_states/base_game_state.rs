use crate::structs::ActionWithRaise;

pub trait GameState {
    fn new_empty(player_amount: usize, seed: Option<u64>) -> Self;
    fn get_total_rounds() -> usize;
    fn get_player_amount(&self) -> usize;
    fn get_current_round_bet_raise_amount(&self) -> usize;
    fn get_history(&self) -> &Vec<Vec<ActionWithRaise>>;
    fn is_leaf_node(&self, subgame_end_situation: usize) -> bool;
    fn get_current_round_index(&self) -> usize;
    fn is_terminal(&self) -> bool;
    fn get_payoffs(&self) -> Vec<i32>;
    fn get_active_player_index(&self) -> usize;
    fn get_active_player_actions(&self, available_actions: &Vec<ActionWithRaise>) -> Vec<ActionWithRaise>;
    fn handle_action(&self, action: ActionWithRaise) -> Self;
    fn can_proceed_to_next_round(&self) -> bool;
}
