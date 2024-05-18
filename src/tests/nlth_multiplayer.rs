#[cfg(test)]
mod poker_tests_multiplayer {
    use hand_isomorphism_rust::deck::card_from_string;
    
    use crate::game_states::base_game_state::GameState;
    use crate::game_states::nlth_poker::game_state::NLTHGameState;
    use crate::structs::{ActionType, Action};
    use crate::tests::action_abstraction::AVAILABLE_ACTIONS;

    // Helper function to create a standardized game state with six players
    fn setup_game_state_six_players() -> NLTHGameState {
        let player_amount = 6;
        let mut nlth_game_state = NLTHGameState::new_empty(player_amount, false, None);
        nlth_game_state.private_hands = [
            [card_from_string("As".to_string()), card_from_string("Ks".to_string())], // Player 0
            [card_from_string("2c".to_string()), card_from_string("3d".to_string())], // Player 1
            [card_from_string("4h".to_string()), card_from_string("5s".to_string())], // Player 2
            [card_from_string("6d".to_string()), card_from_string("7c".to_string())], // Player 3
            [card_from_string("8s".to_string()), card_from_string("9d".to_string())], // Player 4
            [card_from_string("Th".to_string()), card_from_string("Jh".to_string())], // Player 5
        ];
        nlth_game_state.community_cards = [
            card_from_string("Qd".to_string()), card_from_string("Kh".to_string()),
            card_from_string("Ah".to_string()), card_from_string("2s".to_string()),
            card_from_string("3h".to_string()),
        ];
        nlth_game_state
    }

    /*
        //  TURN ORDER  \\
    */

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

    /*
        //  Checking/calling  \\
    */

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
        assert_eq!(game_state.folded_players, [true, true, true, false, false, false]);
        assert_eq!(game_state.is_terminal(), false);
    }

    /*
        //  Folding  \\
    */

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

    /*
        //  Action validation after raises  \\
    */

    #[test]
    fn test_minimum_raise_requirement() {
        let mut game_state = setup_game_state_six_players();
        // Player 2 raises the minimum amount
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 200 });

        let actions = game_state.get_active_player_actions(
            AVAILABLE_ACTIONS[game_state.get_current_round_index()].get(game_state.get_current_bet_count())
        );
        let contains_illegal_action = actions.iter().find(|action| action.raise_amount > 0 && action.raise_amount < 79);
        assert!(contains_illegal_action.is_none());
    }

    #[test]
    fn test_all_in_below_minimum_raise() {
        let mut game_state = setup_game_state_six_players();
        game_state.stacks[3] = 180; // Player 3 has only 180 left

        // Player 2 raises 300
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 300 });

        // Player 3 goes all-in with less than the minimum raise
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert_eq!(game_state.all_in_players[3], 0); // Ensure that player 3 is marked as all-in
        assert!(game_state.current_pot > 0); // Ensure that a new pot is possibly created if needed

        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });

        // The first pots should not go above 180
        for bet in game_state.pots[0] {
            assert!(bet <= 180);
        }

        // Bets should be distributed properly
        assert_eq!(game_state.pots[1][2], 570);
        assert_eq!(game_state.pots[1][4], 570);
        assert_eq!(game_state.pots[1][5], 2400);
    }

    #[test]
    fn test_turn_order() {
        let mut game_state = setup_game_state_six_players();

        // Player 2 calls
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });

        // Player 3 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });

        // Player 4 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });

        // Player 5 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });

        // Player 0 calls
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });

        // Player 1 raises
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });

        // Check that we're still in round 0
        assert_eq!(game_state.round, 0);
        assert_eq!(game_state.active_player_index, 2);

        // Player 2 calls
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });

        assert_eq!(game_state.active_player_index, 0);
    }

    /*
        //  End of Round Scenarios  \\
    */

    /* 
        This test ensures that the game correctly evaluates the hands at the showdown and awards the pot to the winner.
        This test will involve simulating a scenario where the game goes to a showdown after a series of bets.
    */
    #[test]
    fn test_showdown_processing() {
        let mut game_state = setup_game_state_six_players();
        // Simulating bets and calls to reach showdown
        for _ in 0..6 {
            game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        }
        // Assuming all players check on the flop, turn and river
        for _round in 1..4 {
            for _ in 0..6 {
                game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
            }
        }
        assert!(game_state.is_terminal());
        let expected_winner = 5; // Assuming Player 6 (index 5) wins with a straight
        let payoffs = game_state.get_payoffs();
        let winner_index: Option<usize> = payoffs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index);
        assert_eq!(expected_winner, winner_index.unwrap());
    }

    #[test]
    fn test_folding_down_to_one_player() {
        let mut game_state = setup_game_state_six_players();
        // Players folding, leaving one player
        for _ in 1..6 {
            game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        }
        // Ensure the game is terminal when only one player is left
        assert!(game_state.is_terminal());
        let remaining_player = 1;
        assert_eq!(game_state.get_payoffs()[remaining_player], (game_state.get_total_pot() - game_state.bets[0][remaining_player]) as i32);
    }

    /*
        //  Betting behavior  \\
    */

    #[test]
    fn test_under_the_gun_dynamics() {
        let mut game_state = setup_game_state_six_players();

        // Set initial pot after the small blind (50) and big blind (100)
        let initial_pot = game_state.get_total_pot();
        let call_amount = game_state.get_call_amount();

        // UTG player (index 2) decides to fold due to a large bet from the next player
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        assert!(game_state.folded_players[2]);

        // Now the action is on player 3 who decides to go all-in
        let all_in_bet = game_state.stacks[3];
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert_eq!(game_state.stacks[3], 0);
        assert_eq!(game_state.get_total_pot(), initial_pot + all_in_bet);

        assert_eq!(game_state.get_active_player_actions(
            AVAILABLE_ACTIONS[game_state.get_current_round_index()].get(game_state.get_current_bet_count())
        ).len(), 2); // There should only be 2 actions - Fold or All-in

        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert_eq!(game_state.stacks[4], 10_000 - all_in_bet);
        assert_eq!(game_state.get_total_pot(), initial_pot + 2 * all_in_bet);

        assert_eq!(game_state.get_active_player_actions(
            AVAILABLE_ACTIONS[game_state.get_current_round_index()].get(game_state.get_current_bet_count())
        ).len(), 2); // There should only be 2 actions - Fold or All-in

        // Now player 5 acts with a minimum raise
        let min_raise = game_state.minimum_raise_amount;
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert_eq!(game_state.stacks[5], 10000 - min_raise - call_amount);
        assert_eq!(game_state.get_total_pot(), initial_pot + 2 * all_in_bet + min_raise + call_amount);

        // Verify that the round continues correctly
        assert_eq!(game_state.active_player_index, 0); // Back to player 0, who is SB
    }

    #[test]
    fn test_blind_post_and_skip() {
        let mut game_state = setup_game_state_six_players();

        // Assert that the blinds are posted correctly
        assert_eq!(game_state.stacks[0], 9950); // Small Blind
        assert_eq!(game_state.stacks[1], 9900); // Big Blind

        // After blinds, UTG (player 2) should be the first to act
        assert_eq!(game_state.active_player_index, 2);

        // Initial pot should include blinds
        let initial_pot = 150; // Small Blind + Big Blind
        assert_eq!(game_state.get_total_pot(), initial_pot);

        // Player 2 decides to call the big blind
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        assert_eq!(game_state.stacks[2], 9900); // Called the big blind, matching player 1's bet
        assert_eq!(game_state.get_total_pot(), initial_pot + 100); // Player 2 matched the big blind

        // Player 3 raises, increasing the action
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });
        assert_eq!(game_state.stacks[3], 9650); // Player 3's stack after raising
        assert_eq!(game_state.get_total_pot(), initial_pot + 100 + 350); // Includes original blinds, player 2's call, and player 3's raise
    }

    /*
        //  All-Ins and Side Pots  \\
    */
    #[test]
    fn test_multiple_all_ins_with_uneven_stacks() {
        let mut game_state = setup_game_state_six_players();
        
        // Assume Player 2, 4, and 5 go all-in with uneven stacks
        // Player 2 has a smaller stack
        game_state.stacks[2] = 2000;
        game_state.stacks[4] = 3000;
        game_state.stacks[5] = 6000;

        // Player 2 goes all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        // Player 3 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 4 goes all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        // Player 5 goes all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });

        // Check the pots and player stacks
        assert_eq!(game_state.stacks[2], 0);
        assert_eq!(game_state.stacks[4], 0);
        assert_eq!(game_state.stacks[5], 0);
        assert_eq!(game_state.pots[0], [50, 100, 2000, 0, 2000, 2000]); // Main pot
        assert_eq!(game_state.pots[1], [0, 0, 0, 0, 1000, 1000]); // Side pot 1
        assert_eq!(game_state.pots[2], [0, 0, 0, 0, 0, 3000]); // Side pot 2
        assert_eq!(game_state.current_pot, 3); // 3 pots were used, we're now at pot 4
    }

    #[test]
    fn test_sequential_all_ins() {
        let mut game_state = setup_game_state_six_players();

        // Player 5 and Player 6 go all-in one after another
        game_state.stacks[4] = 4000;
        game_state.stacks[5] = 6000;

        // Player 3 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 4 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 5 goes all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        // Player 6 goes all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });

        // Check the pots and player stacks
        assert_eq!(game_state.stacks[4], 0);
        assert_eq!(game_state.stacks[5], 0);
        assert_eq!(game_state.pots[0], [50, 100, 0, 0, 4000, 4000]); // Main pot
        assert_eq!(game_state.pots[1], [0, 0, 0, 0, 0, 2000]); // Side pot 1
        assert_eq!(game_state.current_pot, 2); // 2 pots in total
    }

    #[test]
    fn test_all_in_with_exact_match() {
        let mut game_state = setup_game_state_six_players();
        
        // Player 3 and Player 5 go all-in with the same amount
        game_state.stacks[3] = 4000;
        game_state.stacks[5] = 4000;
        
        // Player 2 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 3 goes all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        // Player 4 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 5 matches the all-in exactly
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });

        // Check the pots and player stacks
        assert_eq!(game_state.stacks[3], 0);
        assert_eq!(game_state.stacks[5], 0);
        assert_eq!(game_state.pots[0], [50, 100, 0, 4000, 0, 4000]); // Only the main pot
        assert_eq!(game_state.current_pot, 2); // 1 pot in total
    }

    /*
        //  Edge Cases  \\
    */
    #[test]
    fn test_big_blind_uncalled() {
        let mut game_state = setup_game_state_six_players();

        // Player 2 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 3 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 4 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 5 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        // Player 0 folds
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });

        assert!(game_state.is_terminal());

        assert_eq!(game_state.get_payoffs()[1], 50) // Big blind wins the small blind (its own blind is removed because zero-sum)
    }

    #[test]
    fn test_small_blind_all_in_calls_and_raises() {
        let mut game_state = setup_game_state_six_players();

        // Player 0 and Player 1 go have a smaller stack
        game_state.stacks[0] = 1000;
        game_state.stacks[1] = 1000;

        // Players index 2 to 5 check
        for _ in 2..6 {
            game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        }

        // Small blind all-in
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert_eq!(game_state.stacks[0], 0);

        // Big blind calls
        game_state = game_state.handle_action(Action { action_type: ActionType::AllIn, raise_amount: 0 });
        assert_eq!(game_state.stacks[1], 0);

        // Another player raises
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });
        assert_eq!(game_state.stacks[2], 10000 - 3_550); // This player's new stack after raising
    }

    /*
        //  Payoffs  \\
    */

    #[test]
    fn test_payoffs() {
        let mut game_state = setup_game_state_six_players();

        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 0 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Bet, raise_amount: 100 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 100 });
        game_state = game_state.handle_action(Action { action_type: ActionType::Fold, raise_amount: 100 });

        while !game_state.is_terminal() {
            game_state = game_state.handle_action(Action { action_type: ActionType::Call, raise_amount: 0 });
        }

        assert_eq!(game_state.get_payoffs(), [-50, -350, -350, 0, -350, 1100]); // This player's new stack after raising
    }
}
