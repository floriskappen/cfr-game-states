mod structs;
mod abstraction {
    pub mod action_abstraction;
}
mod game_states {
    mod kuhn_poker {
        pub mod game_state;
    }
    mod leduc_poker {
        pub mod game_state;
    }
    pub mod nlth_poker {
        pub mod game_state;
        pub mod rank;
    }
    pub mod base_game_state;
}
mod tests {
    pub mod nlth_headsup;
    pub mod nlth_multiplayer;
}

fn main() {
    println!("Hello, world!");
}
