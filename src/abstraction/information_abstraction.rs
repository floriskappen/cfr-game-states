use std::{fs::File, io::{BufReader, Read}};
use std::env;

use hand_isomorphism_rust::hand_indexer::HandIndexer;
use lazy_static::lazy_static;
use prost::Message;

use crate::proto::ClusteredDataLabels;

lazy_static! {
    pub static ref LABELS_FLOP: Vec<u8> = {
        log::info!("ABSTRACTION: Starting to load FLOP labels...");
        let mut buf_reader = BufReader::new(File::open(
            env::var("BLUEPRINT_ABSTRACTION_FLOP_LABELS_FILEPATH").expect("env var BLUEPRINT_ABSTRACTION_FLOP_LABELS_FILEPATH not set")
        ).expect("Error opening ./imports/labels_round_1"));
        let mut buf = Vec::new();
        buf_reader.read_to_end(&mut buf).expect("Error reading buffer from flop labels file");
        let labels = ClusteredDataLabels::decode(&*buf).expect("Error decoding flop ClusteredDataLabels");
        let data = labels.data.into_iter().map(|value| value as u8).collect();
        
        log::info!("ABSTRACTION: Loaded flop labels");
        return data;
    };
    pub static ref LABELS_TURN: Vec<u8> = {
        log::info!("ABSTRACTION: Starting to load TURN labels...");
        let mut buf_reader = BufReader::new(File::open(
            env::var("BLUEPRINT_ABSTRACTION_TURN_LABELS_FILEPATH").expect("env var BLUEPRINT_ABSTRACTION_TURN_LABELS_FILEPATH not set")
        ).expect("Error opening ./imports/labels_round_2"));
        let mut buf = Vec::new();
        buf_reader.read_to_end(&mut buf).expect("Error reading buffer from turn labels file");
        let labels = ClusteredDataLabels::decode(&*buf).expect("Error decoding turn ClusteredDataLabels");
        let data = labels.data.into_iter().map(|value| value as u8).collect();
        
        log::info!("ABSTRACTION: Loaded turn labels");
        return data;
    };
    pub static ref LABELS_RIVER: Vec<u8> = {
        log::info!("ABSTRACTION: Starting to load RIVER labels...");
        let mut buf_reader = BufReader::new(File::open(
            env::var("BLUEPRINT_ABSTRACTION_RIVER_LABELS_FILEPATH").expect("env var not set")
        ).expect("Error opening ./imports/labels_round_3"));
        let mut buf = Vec::new();
        buf_reader.read_to_end(&mut buf).expect("Error reading buffer from river labels file");
        let labels = ClusteredDataLabels::decode(&*buf).expect("Error decoding river ClusteredDataLabels");
        let data = labels.data.into_iter().map(|value| value as u8).collect();

        log::info!("ABSTRACTION: Loaded river labels");
        return data;
    };
    pub static ref HAND_INDEXER: HandIndexer = HandIndexer::new(4, &[2, 3, 1, 1]).unwrap();
}
