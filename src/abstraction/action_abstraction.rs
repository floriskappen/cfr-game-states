use lazy_static::lazy_static;

use crate::structs::{Action, ActionIdentifier, ActionWithRaise, PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID};

lazy_static! {
    pub static ref AVAILABLE_ACTIONS: Vec<Vec<Vec<ActionIdentifier>>> = vec![
        // Round 0
        vec![
            // Initial bet
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 134 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 150 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 200 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 400 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 800 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 1300 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 1500 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 2500 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Raise
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 50 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 200 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 400 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 700 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 1000 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Three-bets
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 50 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 200 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 400 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Four-bets
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 50 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Five-bets
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Six-bets
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
            ],
        ],
        // Round 1
        vec![
            // Initial bet
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 50 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 200 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 400 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 700 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 1300 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Raise
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 50 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 200 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Three-bets
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 50 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Four-bets
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
            ],
        ],
        // Round 2
        vec![
            // Initial bet
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 50 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Raise
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Three-bets
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
            ],
        ],
        // Round 3
        vec![
            // Initial bet
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 50 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Raise
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Bet, raise_amount: 100 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::AllIn, raise_amount: 0 }).unwrap().clone(),
            ],
            // Three-bets
            vec![
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Fold, raise_amount: 0 }).unwrap().clone(),
                PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&ActionWithRaise { action: Action::Call, raise_amount: 0 }).unwrap().clone(),
            ],
        ],
    ];
}
