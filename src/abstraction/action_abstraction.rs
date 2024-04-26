use lazy_static::lazy_static;

use crate::structs::{Action, ActionType};

lazy_static! {
    pub static ref BLUEPRINT_AVAILABLE_ACTIONS: Vec<Vec<Vec<Action>>> = vec![
        // Round 0
        vec![
            // Initial bet
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 134 },
                Action { action_type: ActionType::Bet, raise_amount: 150 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::Bet, raise_amount: 400 },
                Action { action_type: ActionType::Bet, raise_amount: 800 },
                Action { action_type: ActionType::Bet, raise_amount: 1300 },
                Action { action_type: ActionType::Bet, raise_amount: 1500 },
                Action { action_type: ActionType::Bet, raise_amount: 2500 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Raise
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::Bet, raise_amount: 400 },
                Action { action_type: ActionType::Bet, raise_amount: 700 },
                Action { action_type: ActionType::Bet, raise_amount: 1000 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Three-bets
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::Bet, raise_amount: 400 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Four-bets
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Five-bets
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Six-bets
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
            ],
        ],
        // Round 1
        vec![
            // Initial bet
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::Bet, raise_amount: 400 },
                Action { action_type: ActionType::Bet, raise_amount: 700 },
                Action { action_type: ActionType::Bet, raise_amount: 1300 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Raise
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Three-bets
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Four-bets
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
            ],
        ],
        // Round 2
        vec![
            // Initial bet
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Raise
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Three-bets
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
            ],
        ],
        // Round 3
        vec![
            // Initial bet
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Raise
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Three-bets
            vec![
                Action { action_type: ActionType::Fold, raise_amount: 0 },
                Action { action_type: ActionType::Call, raise_amount: 0 },
            ],
        ],
    ];
}
