use rand::Rng;

use crate::structs::Action;

use super::action_abstraction::BLUEPRINT_AVAILABLE_ACTIONS; // Import the Rng trait to use random number generation

pub fn pseudo_harmonic_mapping_randomized(x: f64, a: f64, b: f64) -> f64 {
    let f_ab_x = ((b - x) * (1.0 + a)) / ((b - a) * (1.0 + x));
    let mut rng = rand::thread_rng();
    let threshold: f64 = rng.gen(); // Generates a random float between 0.0 and 1.0

    if threshold <= f_ab_x {
        a
    } else {
        b
    }
}

pub fn get_lower_upper_action_from_abstraction(action: Action, round: usize, raise_amount: usize) -> (Option<Action>, Option<Action>) {
    let mut closest_lower: Option<Action>  = None;
    let mut closest_upper: Option<Action> = None;
    for &abstracted_action in BLUEPRINT_AVAILABLE_ACTIONS[round][raise_amount].iter() {
        if abstracted_action.raise_amount < action.raise_amount && (
            closest_lower.is_none() ||
            closest_lower.is_some_and(|closest_lower| closest_lower.raise_amount < abstracted_action.raise_amount)
        ) {
            closest_lower = Some(abstracted_action);
        } else if abstracted_action.raise_amount > action.raise_amount && (
            closest_upper.is_none() ||
            closest_upper.is_some_and(|closest_upper| closest_upper.raise_amount > abstracted_action.raise_amount)
        ) {
            closest_upper = Some(abstracted_action);
        }
    }

    return (closest_lower, closest_upper)
}

pub fn translate_action(action: Action, round: usize, raise_amount: usize) -> Action {
    let abstracted_action =
        if !BLUEPRINT_AVAILABLE_ACTIONS[round][raise_amount].contains(&action)
    {
        // Use randomized pseudo-harmonic mapping for action translation
        println!("Using action translation");
        let (closest_lower, closest_upper) = get_lower_upper_action_from_abstraction(action, round, raise_amount);
        println!("closest_lower: {:?}, closest_upper: {:?}", closest_lower, closest_upper);
        if closest_lower.is_none() && closest_upper.is_some() {
            // If there's only an upper bound, we use that one
            closest_upper.unwrap()
        } else if closest_upper.is_none() && closest_lower.is_some() {
            // If there's only a lower bound, we use that one
            closest_lower.unwrap()
        } else {
            // Use randomized pseudo-harmonic mapping which will return either the lower or upper raise_amount as f64
            let lower = closest_lower.unwrap();
            let upper = closest_upper.unwrap();
            let mapped_raise_amount = pseudo_harmonic_mapping_randomized(
                action.raise_amount as f64,
                lower.raise_amount as f64,
                upper.raise_amount as f64
            );
            Action { action_type: action.action_type, raise_amount: mapped_raise_amount as u16 }
        }
    } else {
        action
    };

    return abstracted_action
}

