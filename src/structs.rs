use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref BET_RAISE_ACTIONS: Vec<Action> = vec![
        Action::Bet0_25, Action::Bet0_5, Action::Bet0_75, Action::Bet1, Action::Bet1_34, Action::Bet1_5, Action::Bet2, Action::Bet4, Action::Bet7, Action::Bet8, Action::Bet10, Action::Bet13, Action::Bet15, Action::Bet25
    ];
}

lazy_static! {
    pub static ref BET_RAISE_ALL_IN_ACTIONS: Vec<Action> = {
        let mut bet_raise_actions = BET_RAISE_ACTIONS.to_vec();
        bet_raise_actions.push(Action::AllIn);
        return bet_raise_actions;
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Fold,
    Call,
    Bet,
    Bet0_25,
    Bet0_5,
    Bet0_75,
    Bet1,
    Bet1_34,
    Bet1_5,
    Bet2,
    Bet4,
    Bet7,
    Bet8,
    Bet10,
    Bet13,
    Bet15,
    Bet25,
    AllIn,
}
impl Action {
    pub fn as_string(&self) -> String {
        return format!("{:?}", self)
    }

    pub fn as_string_value(&self) -> String {
        return format!("{:?}", self.as_value())
    }

    pub fn as_value(&self) -> u8 {
        match self {
            Action::Fold    => 52,
            Action::Call    => 53,
            Action::Bet     => 54,
            Action::Bet0_25 => 55,
            Action::Bet0_5  => 56,
            Action::Bet0_75 => 57,
            Action::Bet1    => 58,
            Action::Bet1_34 => 59,
            Action::Bet1_5  => 60,
            Action::Bet2    => 61,
            Action::Bet4    => 62,
            Action::Bet7    => 63,
            Action::Bet8    => 64,
            Action::Bet10   => 66,
            Action::Bet13   => 67,
            Action::Bet15   => 68,
            Action::Bet25   => 69,
            Action::AllIn   => 70,
        }
    }
    pub fn from_string(value: &str) -> Self {
        return Action::from_value(value.parse::<u8>().unwrap())
    }

    pub fn from_value(value: u8) -> Self {
        match value {
            52 => Action::Fold,
            53 => Action::Call,
            54 => Action::Bet,
            55 => Action::Bet0_25,
            56 => Action::Bet0_5,
            57 => Action::Bet0_75,
            58 => Action::Bet1,
            59 => Action::Bet1_34,
            60 => Action::Bet1_5,
            61 => Action::Bet2,
            62 => Action::Bet4,
            63 => Action::Bet7,
            64 => Action::Bet8,
            66 => Action::Bet10,
            67 => Action::Bet13,
            68 => Action::Bet15,
            69 => Action::Bet25,
            70 => Action::AllIn,
            _ => Action::Call
        }
    }

    pub fn get_pot_multiplier(&self) -> f32 {
        match self {
            Action::Bet0_25 => 0.25,
            Action::Bet0_5 => 0.5,
            Action::Bet0_75 => 0.75,
            Action::Bet1 => 1.0,
            Action::Bet1_34 => 1.34,
            Action::Bet1_5 => 1.5,
            Action::Bet2 => 2.0,
            Action::Bet4 => 4.0,
            Action::Bet7 => 7.0,
            Action::Bet8 => 8.0,
            Action::Bet10 => 10.0,
            Action::Bet13 => 13.0,
            Action::Bet15 => 15.0,
            Action::Bet25 => 25.0,
            _ => 0.0
        }
    }
}
