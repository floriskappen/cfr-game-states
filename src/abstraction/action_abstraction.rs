use lazy_static::lazy_static;

use crate::structs::Action;

lazy_static! {
    pub static ref AVAILABLE_ACTIONS: Vec<Vec<Vec<Action>>> = vec![
        // Round 0
        vec![
            // Initial bet
            vec![Action::Fold, Action::Call, Action::Bet1_34, Action::Bet1_5, Action::Bet2, Action::Bet4, Action::Bet8, Action::Bet13, Action::Bet15, Action::Bet25, Action::AllIn],
            // Raise
            vec![Action::Fold, Action::Call, Action::Bet0_5, Action::Bet1, Action::Bet2, Action::Bet4, Action::Bet7, Action::Bet10, Action::AllIn],
            // Three-bets
            vec![Action::Fold, Action::Call, Action::Bet0_5, Action::Bet1, Action::Bet2, Action::Bet4, Action::AllIn],
            // Four-bets
            vec![Action::Fold, Action::Call, Action::Bet0_5, Action::Bet1, Action::AllIn],
            // Five-bets
            vec![Action::Fold, Action::Call, Action::Bet1, Action::AllIn],
            // Six-bets
            vec![Action::Fold, Action::Call],
        ],
        // Round 1
        vec![
            // Initial bet
            vec![Action::Fold, Action::Call, Action::Bet0_5, Action::Bet1, Action::Bet2, Action::Bet4, Action::Bet7, Action::Bet13, Action::AllIn],
            // Raise
            vec![Action::Fold, Action::Call, Action::Bet0_5, Action::Bet1, Action::Bet2, Action::AllIn],
            // Three-bets
            vec![Action::Fold, Action::Call, Action::Bet0_5, Action::Bet1, Action::AllIn],
            // Four-bets
            vec![Action::Fold, Action::Call],
        ],
        // Round 2
        vec![
            // Initial bet
            vec![Action::Fold, Action::Call, Action::Bet0_5, Action::Bet1, Action::AllIn],
            // Raise
            vec![Action::Fold, Action::Call, Action::Bet1, Action::AllIn],
            // Three-bets
            vec![Action::Fold, Action::Call],
        ],
        // Round 3
        vec![
            // Initial bet
            vec![Action::Fold, Action::Call, Action::Bet0_5, Action::Bet1, Action::AllIn],
            // Raise
            vec![Action::Fold, Action::Call, Action::Bet1, Action::AllIn],
            // Three-bets
            vec![Action::Fold, Action::Call],
        ],
    ];
}
