use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub type ActionIdentifier = u8;

lazy_static! {

    pub static ref PREDEFINED_ACTION_ID_TO_ACTION_WITH_RAISE: HashMap<ActionIdentifier, Action> = {
        let mut result = HashMap::new();
        result.insert(0, Action { action_type: ActionType::Fold, raise_amount: 0 });
        result.insert(1, Action { action_type: ActionType::Call, raise_amount: 0 });
        result.insert(2, Action { action_type: ActionType::AllIn, raise_amount: 0 });
        result.insert(3, Action { action_type: ActionType::Bet, raise_amount: 0 });
        result.insert(4, Action { action_type: ActionType::Bet, raise_amount: 25 });
        result.insert(5, Action { action_type: ActionType::Bet, raise_amount: 50 });
        result.insert(6, Action { action_type: ActionType::Bet, raise_amount: 75 });
        result.insert(7, Action { action_type: ActionType::Bet, raise_amount: 80 });
        result.insert(8, Action { action_type: ActionType::Bet, raise_amount: 100 });
        result.insert(9, Action { action_type: ActionType::Bet, raise_amount: 134 });
        result.insert(10, Action { action_type: ActionType::Bet, raise_amount: 150 });
        result.insert(11, Action { action_type: ActionType::Bet, raise_amount: 200 });
        result.insert(12, Action { action_type: ActionType::Bet, raise_amount: 400 });
        result.insert(13, Action { action_type: ActionType::Bet, raise_amount: 700 });
        result.insert(14, Action { action_type: ActionType::Bet, raise_amount: 800 });
        result.insert(15, Action { action_type: ActionType::Bet, raise_amount: 1000 });
        result.insert(16, Action { action_type: ActionType::Bet, raise_amount: 1300 });
        result.insert(17, Action { action_type: ActionType::Bet, raise_amount: 1500 });
        result.insert(18, Action { action_type: ActionType::Bet, raise_amount: 2500 });

        return result;
    };

    pub static ref PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID: HashMap<Action, ActionIdentifier> = {
        return PREDEFINED_ACTION_ID_TO_ACTION_WITH_RAISE.iter()
            .map(|(&id, action)| (action.clone(), id))
            .collect();
    };
}

#[derive(Eq, Hash, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename(serialize = "fold"))]
    Fold,
    #[serde(rename(serialize = "call"))]
    Call,
    #[serde(rename(serialize = "bet"))]
    Bet,
    #[serde(rename(serialize = "all_in"))]
    AllIn,
}
impl ActionType {
    pub fn from_string(action_type_string: &str) -> Option<Self> {
        match action_type_string {
            "fold"      => return Some(ActionType::Fold),
            "call"      => return Some(ActionType::Call),
            "bet"       => return Some(ActionType::Bet),
            "all_in"    => return Some(ActionType::AllIn),
            _ => return None
        };
    }
    pub fn to_string(&self) -> String {
        match self {
            ActionType::Fold        => "fold".to_owned(),
            ActionType::Call        => "call".to_owned(),
            ActionType::Bet         => "bet".to_owned(),
            ActionType::AllIn       => "all_in".to_owned(),
        }
    }
}

#[derive(Eq, Hash, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub raise_amount: u16
}

impl Action {
    pub fn as_string(&self) -> String {
        if self.raise_amount != 0 {
            return format!("{:?} x{}", self.action_type, self.raise_amount as f32 / 100.0)
        }
        return format!("{:?}", self.action_type)
    }
    pub fn get_multiplier(&self) -> f32 {
        return self.raise_amount as f32 / 100.0
    }

    pub fn from_string(value: &str) -> Option<Self> {
        if let Ok(value_u8) = value.parse::<ActionIdentifier>() {
            if let Some(action) = PREDEFINED_ACTION_ID_TO_ACTION_WITH_RAISE.get(&value_u8) {
                return Some(action.clone());
            }
        }
        return None;
    }

    pub fn into_identifier(&self) -> Option<&ActionIdentifier> {
        return PREDEFINED_ACTION_WITH_RAISE_TO_ACTION_ID.get(&self)
    }

    pub fn is_bet_raise(&self) -> bool {
        return self.action_type == ActionType::Bet;
    }
}
