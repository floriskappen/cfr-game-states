use std::collections::HashMap;

use hand_isomorphism_rust::deck::card_from_string;
use holdem_hand_evaluator::Hand;
use lazy_static::lazy_static;

lazy_static! {
    static ref CARD_ISOMORPHISM_TO_INDEX_LOOKUP: HashMap<u8, usize> = {
        let mut m = HashMap::new();
        let ranks = vec!["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
        let suits = vec!["c", "d", "h", "s",];
        for (i, rank) in ranks.iter().enumerate() {
            for (j, suit) in suits.iter().enumerate() {
                let card_isomorphism_id = card_from_string(format!("{}{}", rank, suit));
                let card_index = i * suits.len() + j;
                m.insert(card_isomorphism_id, card_index);
            }
        }
        return m
    };
}

pub fn rank_hand(hand: Vec<u8>) -> u16 {
    let hand_index = hand.into_iter()
        .map(|card| CARD_ISOMORPHISM_TO_INDEX_LOOKUP[&card])
        .collect::<Vec<usize>>();
    let evaluator_hand = Hand::from_slice(&hand_index);

    return evaluator_hand.evaluate()
}
