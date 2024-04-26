pub mod structs;
pub mod abstraction {
    pub mod action_abstraction;
    pub mod action_translation;
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
mod tests {
    mod nlth_headsup;
    mod nlth_multiplayer;
    mod action_abstraction;
}

use dotenv::dotenv;

fn main() {
    dotenv().ok();
}
