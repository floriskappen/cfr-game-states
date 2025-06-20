use lazy_static::lazy_static;
use smallvec::{smallvec, SmallVec};

use crate::structs::{ActionType, Action};

lazy_static! {
    pub static ref AVAILABLE_ACTIONS: [SmallVec<[SmallVec<[Action; 40]>; 8]>; 4] = [
        // Round 0
        smallvec![
            // Initial bet
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 75 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 134 },
                Action { action_type: ActionType::Bet, raise_amount: 150 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::Bet, raise_amount: 400 },
                Action { action_type: ActionType::Bet, raise_amount: 800 },
                Action { action_type: ActionType::Bet, raise_amount: 1300 },
                Action { action_type: ActionType::Bet, raise_amount: 1500 },
                Action { action_type: ActionType::Bet, raise_amount: 2500 },
            ],
            // Raise
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 75 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::Bet, raise_amount: 400 },
                Action { action_type: ActionType::Bet, raise_amount: 700 },
                Action { action_type: ActionType::Bet, raise_amount: 1000 },
                Action { action_type: ActionType::Bet, raise_amount: 1500 },
            ],
            // Three-bets
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 75 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::Bet, raise_amount: 400 },
                Action { action_type: ActionType::Bet, raise_amount: 800 },
            ],
            // Four-bets
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
            ],
        ],
        // Round 1
        smallvec![
            // Initial bet
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
                Action { action_type: ActionType::Bet, raise_amount: 400 },
                Action { action_type: ActionType::Bet, raise_amount: 700 },
                Action { action_type: ActionType::Bet, raise_amount: 1300 },
            ],
            // Raise
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
                Action { action_type: ActionType::Bet, raise_amount: 200 },
            ],
            // Three-bets
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
            ],
        ],
        // Round 2
        smallvec![
            // Initial bet
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
            ],
            // Raise
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 100 },
            ],
        ],
        // Round 3
        smallvec![
            // Initial bet
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 50 },
                Action { action_type: ActionType::Bet, raise_amount: 100 },
            ],
            // Raise
            smallvec![
                Action { action_type: ActionType::Bet, raise_amount: 100 },
            ],
        ],
    ];
}