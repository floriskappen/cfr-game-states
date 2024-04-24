mod proto {
    include!("proto/build/_.rs");
}
pub mod structs;
pub mod abstraction {
    pub mod action_abstraction;
    pub mod information_abstraction;
}
pub mod game_states {
    pub mod kuhn_poker {
        pub mod game_state;
    }
    pub mod leduc_poker {
        pub mod game_state;
    }
    pub mod nlth_poker {
        pub mod game_state;
        pub mod rank;
    }
    pub mod base_game_state;
}