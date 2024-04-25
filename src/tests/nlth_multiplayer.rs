#[cfg(test)]
mod poker_tests_multiplayer {
    use hand_isomorphism_rust::deck::card_from_string;
    
    use crate::game_states::base_game_state::GameState;
    use crate::game_states::nlth_poker::game_state::NLTHGameState;
    use crate::structs::{ActionType, Action};

    // Helper function to create a standardized game state with six players
    fn setup_game_state_six_players() -> NLTHGameState {
        let player_amount = 6;
        let mut nlth_game_state = NLTHGameState::new_empty(player_amount, None);
        nlth_game_state.private_hands = vec![
            vec![card_from_string("As".to_string()), card_from_string("Ks".to_string())], // Player 0
            vec![card_from_string("2c".to_string()), card_from_string("3d".to_string())], // Player 1
            vec![card_from_string("4h".to_string()), card_from_string("5s".to_string())], // Player 2
            vec![card_from_string("6d".to_string()), card_from_string("7c".to_string())], // Player 3
            vec![card_from_string("8s".to_string()), card_from_string("9d".to_string())], // Player 4
            vec![card_from_string("Th".to_string()), card_from_string("Jh".to_string())], // Player 5
        ];
        nlth_game_state.community_cards = vec![
            card_from_string("Qd".to_string()), card_from_string("Kh".to_string()),
            card_from_string("Ah".to_string()), card_from_string("2s".to_string()),
            card_from_string("3h".to_string()),
        ];
        nlth_game_state
    }

    #[test]
    fn test_turn_order_simple() {
        let mut game_state = setup_game_state_six_players();
        // Simulate several players folding
        assert_eq!(game_state.active_player_index, 2);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 3 calls
        assert_eq!(game_state.active_player_index, 3);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 4 calls
        assert_eq!(game_state.active_player_index, 4);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 5 calls
        assert_eq!(game_state.active_player_index, 5);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 6 calls
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 1 calls
        assert_eq!(game_state.active_player_index, 1);
        assert_eq!(game_state.round, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 2 calls
        assert_eq!(game_state.round, 1);
        assert_eq!(game_state.active_player_index, 0);
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_turn_order_complex_betting() {
        let mut game_state = setup_game_state_six_players();
        // Simulate several players folding
        assert_eq!(game_state.active_player_index, 2);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 3 calls
        assert_eq!(game_state.active_player_index, 3);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 4 calls
        assert_eq!(game_state.active_player_index, 4);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 5 calls
        assert_eq!(game_state.active_player_index, 5);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 6 calls
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 }); // Player 1 bets 1x pot
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 2 calls
        assert_eq!(game_state.active_player_index, 2);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 3 calls
        assert_eq!(game_state.active_player_index, 3);
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 50 }); // Player 4 bets 0.5x pot
        assert_eq!(game_state.active_player_index, 4);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 5 calls
        assert_eq!(game_state.active_player_index, 5);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 6 folds
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 1 calls
        assert_eq!(game_state.active_player_index, 2);
        assert_eq!(game_state.round, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 3 calls (skip player 2 cuz he folded)
        assert_eq!(game_state.round, 1);
        assert_eq!(game_state.active_player_index, 0);
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_complex_checking_1() {
        let mut game_state = setup_game_state_six_players();
        // Simulate several players folding
        assert_eq!(game_state.active_player_index, 2);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 3 calls
        assert_eq!(game_state.active_player_index, 3);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 4 calls
        assert_eq!(game_state.active_player_index, 4);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 5 calls
        assert_eq!(game_state.active_player_index, 5);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 6 calls
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 1 bets 1x pot
        assert_eq!(game_state.active_player_index, 1);
        assert_eq!(game_state.round, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 2 calls
        assert_eq!(game_state.round, 1);
        assert_eq!(game_state.active_player_index, 0);
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_complex_checking_2() {
        let mut game_state = setup_game_state_six_players();
        // Simulate several players folding
        assert_eq!(game_state.active_player_index, 2);
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 150 }); // Player 3 bets
        assert_eq!(game_state.active_player_index, 3);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 4 folds
        assert_eq!(game_state.active_player_index, 4);
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 }); // Player 5 bets
        assert_eq!(game_state.active_player_index, 5);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 6 calls
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 1 bets 1x pot
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 2 calls
        assert_eq!(game_state.round, 0);
        assert_eq!(game_state.all_remaining_players_checked(), false);
        assert_eq!(game_state.active_player_index, 2);
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_complex_checking_3() {
        let mut game_state = setup_game_state_six_players();
        // Simulate several players folding
        assert_eq!(game_state.active_player_index, 2);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 3 calls
        assert_eq!(game_state.active_player_index, 3);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 4 calls
        assert_eq!(game_state.active_player_index, 4);
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 150 }); // Player 5 bets
        assert_eq!(game_state.active_player_index, 5);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 6 calls
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 1 folds
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 2 folds
        assert_eq!(game_state.active_player_index, 2);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 3 folds
        assert_eq!(game_state.active_player_index, 3);
        assert_eq!(game_state.round, 0);
        assert_eq!(game_state.all_remaining_players_checked(), false);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 4 calls
        assert_eq!(game_state.round, 1);
        assert_eq!(game_state.active_player_amount, 3); // Players 4, 5 and 6 remain
        assert_eq!(game_state.folded_players, vec![true, true, true, false, false, false]);
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_initial_folds() {
        let mut game_state = setup_game_state_six_players();
        // Simulate several players folding
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 0 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 1 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 2 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player 3 folds
        // Two players remaining should continue the game
        assert_eq!(game_state.is_terminal(), false);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 4 calls
        assert_eq!(game_state.round, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 5 calls
        // Game should now proceed to post-flop
        assert_eq!(game_state.round, 1);
        assert_eq!(game_state.is_terminal(), false);
    }
}
