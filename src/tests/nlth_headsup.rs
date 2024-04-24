
#[cfg(test)]
mod poker_tests_headsup {
    use hand_isomorphism_rust::deck::card_from_string;
    
    use crate::game_states::base_game_state::GameState;
    use crate::game_states::nlth_poker::game_state::NLTHGameState;
    use crate::structs::Action;

    // Helper function to create a standardized game state
    fn setup_game_state() -> NLTHGameState {
        let player_amount = 2;
        let mut nlth_game_state = NLTHGameState::new_empty(player_amount, None);
        nlth_game_state.private_hands = vec![
            vec![card_from_string("As".to_string()), card_from_string("Ks".to_string())],
            vec![card_from_string("2c".to_string()), card_from_string("3d".to_string())],
        ];
        nlth_game_state.community_cards = vec![
            card_from_string("Jd".to_string()), card_from_string("Qh".to_string()),
            card_from_string("Td".to_string()), card_from_string("5s".to_string()),
            card_from_string("3h".to_string()),
        ];
        nlth_game_state
    }

    #[test]
    fn test_pre_flop_folding() {
        let mut game_state = setup_game_state();
        assert_eq!(game_state.get_active_player_actions().contains(&Action::Bet4), true);
        game_state = game_state.handle_action(Action::Bet0_25);
        assert_eq!(game_state.get_active_player_actions().contains(&Action::Fold), true);
        game_state = game_state.handle_action(Action::Fold);
        assert_eq!(game_state.is_terminal(), true);
        let payoffs = game_state.get_payoffs();
        assert_eq!(payoffs[0] > 0, true); // Assuming player 1 is the one who folded
    }

    #[test]
    fn test_pre_flop_calling() {
        let mut game_state = setup_game_state();
        assert!(game_state.get_active_player_actions().contains(&Action::Call));
        game_state = game_state.handle_action(Action::Call); // Player 1 calls
        game_state = game_state.handle_action(Action::Call); // Player 2 calls
        assert_eq!(game_state.is_terminal(), false); // Should not be terminal yet
        game_state = game_state.handle_action(Action::Call); // Move to next round
        game_state = game_state.handle_action(Action::Call);
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_preflop_raise_reraise_and_call() {
        let mut game_state = setup_game_state();
        game_state = game_state.handle_action(Action::Bet2); // Player A raises
        game_state = game_state.handle_action(Action::Bet4); // Player B re-raises (3-bet)
        game_state = game_state.handle_action(Action::Call); // Player A calls
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_preflop_raise_allin_and_fold() {
        let mut game_state = setup_game_state();
        game_state = game_state.handle_action(Action::Bet2); // Player A raises
        game_state = game_state.handle_action(Action::AllIn); // Player B goes all-in
        assert_eq!(game_state.is_terminal(), false);
        game_state = game_state.handle_action(Action::Fold); // Player A folds
        assert_eq!(game_state.is_terminal(), true);
    }

    #[test]
    fn test_preflop_raise_allin_and_call() {
        let mut game_state = setup_game_state();
        game_state = game_state.handle_action(Action::Bet2); // Player A raises
        game_state = game_state.handle_action(Action::AllIn); // Player B goes all-in
        game_state = game_state.handle_action(Action::AllIn); // Player A calls with their own all-in
        assert_eq!(game_state.is_terminal(), true);
    }

    #[test]
    fn test_all_in_and_fold() {
        let mut game_state = setup_game_state();
        game_state = game_state.handle_action(Action::AllIn);
        assert!(game_state.get_active_player_actions().contains(&Action::Fold));
        game_state = game_state.handle_action(Action::Fold);
        assert_eq!(game_state.is_terminal(), true);
        let payoffs = game_state.get_payoffs();
        assert_eq!(payoffs[0] > 0, true);
    }

    #[test]
    fn test_multiple_betting_rounds() {
        let mut game_state = setup_game_state();
        // Initial bets
        game_state = game_state.handle_action(Action::Bet2);
        game_state = game_state.handle_action(Action::Call);
        // Flop bets
        game_state = game_state.handle_action(Action::Bet1);
        game_state = game_state.handle_action(Action::Call);
        // Turn bets
        game_state = game_state.handle_action(Action::Bet0_5);
        game_state = game_state.handle_action(Action::Call);
        // River bets
        game_state = game_state.handle_action(Action::Bet1);
        game_state = game_state.handle_action(Action::Call);
        assert_eq!(game_state.is_terminal(), true);
        assert!(game_state.get_payoffs().iter().any(|&x| x > 0));
    }

    #[test]
    fn test_stack_size_after_bets() {
        let mut game_state = setup_game_state();
        let initial_pot = game_state.get_total_pot(); // Pot after blinds should be 150

        // Player 1 acts first after blinds, betting 1.5x the current pot.
        // Also account for the call of the big blind, so 50 will be added to the pot apart from the bet
        let bet_size = (1.5 * initial_pot as f64) as usize;
        game_state = game_state.handle_action(Action::Bet1_5);
        assert_eq!(game_state.stacks[0], 9900 /* 9950 minus 50 for the call */ - bet_size);
        assert_eq!(game_state.get_total_pot(), initial_pot + 50 + bet_size);

        // Player 2 responds with a call
        game_state = game_state.handle_action(Action::Call);
        assert_eq!(game_state.stacks[1], 9900 - bet_size);
        assert_eq!(game_state.get_total_pot(), initial_pot + 50 + 2 * bet_size);

        // Moving to the next betting round, Player 2 is FTA and decides to bet 1x the new pot size
        let new_pot = game_state.get_total_pot();
        let new_bet_size = new_pot; // 1x the pot
        game_state = game_state.handle_action(Action::Bet1);
        assert_eq!(game_state.stacks[1], 9900 - bet_size - new_bet_size);
        assert_eq!(game_state.get_total_pot(), new_pot + new_bet_size);

        // Player 1 goes all-in, which is less than a normal Bet1 due to previous betting
        let all_in_amount = game_state.stacks[0]; // All remaining stack
        game_state = game_state.handle_action(Action::AllIn);
        assert_eq!(game_state.stacks[0], 0);
        assert_eq!(game_state.get_total_pot(), new_pot + new_bet_size + all_in_amount);

        // Player 2 also goes all-in
        game_state = game_state.handle_action(Action::AllIn);
        assert_eq!(game_state.get_total_pot(), 20000);

        // Verify game ends
        assert!(game_state.is_terminal());
    }

    #[test]
    fn test_turn_order() {
        let mut game_state = setup_game_state();
        // Preflop
        // SB (player 0) starts
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action::Call);  // SB calls
        // BB (player 1) acts
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action::Call); // BB checks

        // Move to flop
        // Postflop, BB (player 1) starts
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action::Call); // BB checks
        // SB (player 0) acts
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action::Call); // SB checks

        // Move to turn
        // Turn, BB (player 1) starts
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action::Call); // BB checks
        // SB (player 0) acts
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action::Call); // SB checks

        // Move to river
        // River, BB (player 1) starts
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action::Call); // BB checks
        // SB (player 0) acts
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action::Call); // SB checks

        assert_eq!(game_state.is_terminal(), true); // Game should be terminal
    }

    #[test]
    fn test_payoffs() {
        let mut game_state = setup_game_state();
        // Initial bets
        game_state = game_state.handle_action(Action::Bet2);
        game_state = game_state.handle_action(Action::Call);
        // Flop bets
        game_state = game_state.handle_action(Action::Bet1);
        game_state = game_state.handle_action(Action::Call);
        // Turn bets
        game_state = game_state.handle_action(Action::Bet0_5);
        game_state = game_state.handle_action(Action::Call);
        // // River bets
        game_state = game_state.handle_action(Action::Bet1);
        game_state = game_state.handle_action(Action::Call);
        assert_eq!(game_state.is_terminal(), true);

        assert_eq!(game_state.get_payoffs(), vec![7200, -7200]);
    }
}
