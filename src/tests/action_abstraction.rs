use lazy_static::lazy_static;

use crate::structs::{ActionType, ActionWithRaise, PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID};

lazy_static! {
    pub static ref AVAILABLE_ACTIONS: Vec<Vec<Vec<ActionWithRaise>>> = vec![
        // Round 0
        vec![
            // Initial bet
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 134 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 150 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 200 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 400 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 800 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1300 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1500 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 2500 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Raise
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 50 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 200 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 400 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 700 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1000 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Three-bets
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 50 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 200 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 400 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Four-bets
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 50 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Five-bets
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Six-bets
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
            ],
        ],
        // Round 1
        vec![
            // Initial bet
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 50 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 200 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 400 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 700 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 1300 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Raise
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 50 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 200 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Three-bets
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 50 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Four-bets
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
            ],
        ],
        // Round 2
        vec![
            // Initial bet
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 50 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Raise
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Three-bets
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
            ],
        ],
        // Round 3
        vec![
            // Initial bet
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 50 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Raise
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Bet, raise_amount: 100 },
                ActionWithRaise { action_type: ActionType::AllIn, raise_amount: 0 },
            ],
            // Three-bets
            vec![
                ActionWithRaise { action_type: ActionType::Fold, raise_amount: 0 },
                ActionWithRaise { action_type: ActionType::Call, raise_amount: 0 },
            ],
        ],
    ];
}
