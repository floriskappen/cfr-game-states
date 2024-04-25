pub mod structs;
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
mod tests {
    mod nlth_headsup;
    mod nlth_multiplayer;
    mod action_abstraction;
}

fn main() {}
