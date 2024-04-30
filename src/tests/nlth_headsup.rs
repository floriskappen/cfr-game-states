
#[cfg(test)]

mod poker_tests_headsup {
    use cfr_game_states::constants::NO_CARD_PLACEHOLDER;
    use hand_isomorphism_rust::deck::card_from_string;
    
    use crate::game_states::base_game_state::GameState;
    use crate::game_states::nlth_poker::game_state::NLTHGameState;
    use crate::structs::{ActionType, Action};
    use crate::tests::action_abstraction::AVAILABLE_ACTIONS;

    // Helper function to create a standardized game state
    fn setup_game_state() -> NLTHGameState {
        let player_amount = 2;
        let mut nlth_game_state = NLTHGameState::new_empty(player_amount, false, None);
        nlth_game_state.private_hands = [
            [card_from_string("As".to_string()), card_from_string("Ks".to_string())],
            [card_from_string("2c".to_string()), card_from_string("3d".to_string())],
            [NO_CARD_PLACEHOLDER; 2],
            [NO_CARD_PLACEHOLDER; 2],
            [NO_CARD_PLACEHOLDER; 2],
            [NO_CARD_PLACEHOLDER; 2],
        ];
        nlth_game_state.community_cards = [
            card_from_string("Jd".to_string()), card_from_string("Qh".to_string()),
            card_from_string("Td".to_string()), card_from_string("5s".to_string()),
            card_from_string("3h".to_string()),
        ];
        nlth_game_state
    }

    #[test]
    fn test_pre_flop_folding() {
        let mut game_state = setup_game_state();
        let available_actions = AVAILABLE_ACTIONS[game_state.get_current_round_index()].get(game_state.get_current_bet_count());
        assert_eq!(game_state.get_active_player_actions(available_actions.clone()).contains(&Action { action_type: ActionType::Bet, raise_amount: 400 }), true);
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 25 });
        assert_eq!(game_state.get_active_player_actions(available_actions).contains(&Action { action_type: ActionType::Fold, raise_amount: 0 }), true);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        assert_eq!(game_state.is_terminal(), true);
        let payoffs = game_state.get_payoffs();
        assert_eq!(payoffs[0] > 0, true); // Assuming player 1 is the one who folded
    }

    #[test]
    fn test_pre_flop_calling() {
        let mut game_state = setup_game_state();
        let available_actions = AVAILABLE_ACTIONS[game_state.get_current_round_index()].get(game_state.get_current_bet_count());
        assert!(game_state.get_active_player_actions(available_actions).contains(&Action { action_type: ActionType::Call, raise_amount: 0 }));
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 1 calls
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player 2 calls
        assert_eq!(game_state.is_terminal(), false); // Should not be terminal yet
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Move to next round
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_preflop_raise_reraise_and_call() {
        let mut game_state = setup_game_state();
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 200 }); // Player A raises
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 400 }); // Player B re-raises (3-bet)
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // Player A calls
        assert_eq!(game_state.is_terminal(), false);
    }

    #[test]
    fn test_preflop_raise_allin_and_fold() {
        let mut game_state = setup_game_state();
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 200 }); // Player A raises
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 }); // Player B goes all-in
        assert_eq!(game_state.is_terminal(), false);
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 }); // Player A folds
        assert_eq!(game_state.is_terminal(), true);
    }

    #[test]
    fn test_preflop_raise_allin_and_call() {
        let mut game_state = setup_game_state();
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 200 }); // Player A raises
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 }); // Player B goes all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 }); // Player A calls with their own all-in
        assert_eq!(game_state.is_terminal(), true);
    }

    #[test]
    fn test_all_in_and_fold() {
        let mut game_state = setup_game_state();
        let available_actions = AVAILABLE_ACTIONS[game_state.get_current_round_index()].get(game_state.get_current_bet_count());
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert!(game_state.get_active_player_actions(available_actions).contains(&Action { action_type: ActionType::Fold, raise_amount: 0 }));
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        assert_eq!(game_state.is_terminal(), true);
        let payoffs = game_state.get_payoffs();
        assert_eq!(payoffs[0] > 0, true);
    }

    #[test]
    fn test_all_in_and_call() {
        let mut game_state = setup_game_state();
        let available_actions = AVAILABLE_ACTIONS[game_state.get_current_round_index()].get(game_state.get_current_bet_count());
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert!(game_state.get_active_player_actions(available_actions).contains(&Action { action_type: ActionType::AllIn, raise_amount: 0 }));
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert_eq!(game_state.is_terminal(), true);
        let payoffs = game_state.get_payoffs();
        assert_eq!(payoffs[0], 10_000);
        assert_eq!(payoffs[1], -10_000);
    }

    // #[test]
    // fn test_bet_all_in() {
    //     let mut game_state = setup_game_state();
    //     let available_actions = AVAILABLE_ACTIONS[game_state.get_current_round_index()].get(game_state.get_current_bet_count());
    //     game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 1300 });
    //     assert!(game_state.get_active_player_actions(available_actions).contains(&Action { action_type: ActionType::Call, raise_amount: 0 }));
    //     game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
    //     assert_eq!(game_state.is_terminal(), true);
    //     let payoffs = game_state.get_payoffs();
    //     assert_eq!(payoffs[0], 10_000);
    //     assert_eq!(payoffs[1], -10_000);
    // }

    #[test]
    fn test_multiple_betting_rounds() {
        let mut game_state = setup_game_state();
        // Initial bets
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 200 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        // Flop bets
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        // Turn bets
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 50 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        // River bets
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        assert_eq!(game_state.is_terminal(), true);
        assert!(game_state.get_payoffs().iter().any(|&x| x > 0));
    }

    #[test]
    fn test_stack_size_after_bets() {
        let mut game_state = setup_game_state();
        let initial_pot = game_state.get_total_pot(); // Pot after blinds should be 150

        // Player 1 acts first after blinds, betting 1.5x the current pot.
        // Also account for the call of the big blind, so 50 will be added to the pot apart from the bet
        let bet_size = (1.5 * (initial_pot as f64 + 50.0)) as u32;
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 150 });
        assert_eq!(game_state.stacks[0], 9900 /* 9950 minus 50 for the call */ - bet_size);
        assert_eq!(game_state.get_total_pot(), initial_pot + 50 + bet_size);

        // Player 2 responds with a call
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        assert_eq!(game_state.stacks[1], 9900 - bet_size);
        assert_eq!(game_state.get_total_pot(), initial_pot + 50 + 2 * bet_size);

        // Moving to the next betting round, Player 2 is FTA and decides to bet 1x the new pot size
        let new_pot = game_state.get_total_pot();
        let new_bet_size = new_pot; // 1x the pot
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });
        assert_eq!(game_state.stacks[1], 9900 - bet_size - new_bet_size);
        assert_eq!(game_state.get_total_pot(), new_pot + new_bet_size);

        // Player 1 goes all-in, which is less than a normal Bet1 due to previous betting
        let all_in_amount = game_state.stacks[0]; // All remaining stack
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert_eq!(game_state.stacks[0], 0);
        assert_eq!(game_state.get_total_pot(), new_pot + new_bet_size + all_in_amount);

        // Player 2 also goes all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
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
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });  // SB calls
        // BB (player 1) acts
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // BB checks

        // Move to flop
        // Postflop, BB (player 1) starts
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // BB checks
        // SB (player 0) acts
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // SB checks

        // Move to turn
        // Turn, BB (player 1) starts
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // BB checks
        // SB (player 0) acts
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // SB checks

        // Move to river
        // River, BB (player 1) starts
        assert_eq!(game_state.active_player_index, 1);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // BB checks
        // SB (player 0) acts
        assert_eq!(game_state.active_player_index, 0);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 }); // SB checks

        assert_eq!(game_state.is_terminal(), true); // Game should be terminal
    }

    #[test]
    fn test_payoffs() {
        let mut game_state = setup_game_state();
        // Initial bets
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 200 });
        assert_eq!(game_state.bets[0][0], 500);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        assert_eq!(game_state.bets[0][1], 500);
        assert_eq!(game_state.pots[0].iter().sum::<u32>(), 1000);
        // Flop bets
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });
        assert_eq!(game_state.bets[1][1], 1000);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        assert_eq!(game_state.bets[1][0], 1000);
        assert_eq!(game_state.pots[0].iter().sum::<u32>(), 3000);
        // Turn bets
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 50 });
        assert_eq!(game_state.bets[2][1], 1500);
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        assert_eq!(game_state.bets[2][0], 1500);
        assert_eq!(game_state.pots[0].iter().sum::<u32>(), 6000);
        // River bets
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        assert_eq!(game_state.is_terminal(), true);

        assert_eq!(game_state.get_payoffs(), [9000, -9000, 0, 0, 0, 0]);
    }
}
