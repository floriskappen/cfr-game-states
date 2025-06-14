# CFR Game States

This crate provides minimal implementations of several poker variants for use with Counterfactual Regret Minimization (CFR) algorithms.  The goal is to offer simple game state representations that can be used for experimentation or as building blocks for CFR based solvers.

## Crate Layout

Game logic is organised in `src/game_states/`:

- `base_game_state.rs` – defines the `GameState` trait used by all variants.
- `kuhn_poker/game_state.rs` – contains `KPGameState` for the three-card Kuhn Poker variant.
- `leduc_poker/game_state.rs` – contains `LPGameState` for two round Leduc Poker.
- `nlth_poker/game_state.rs` – contains `NLTHGameState` for No‑Limit Texas Hold’em.
- `nlth_poker/rank.rs` – helper functions for evaluating NLTH hands.

Common types used across the crate live in:

- `src/structs.rs` – defines the `ActionType` enum and `Action` struct along with mappings between predefined actions and identifiers.
- `src/constants.rs` – global constants describing deck and game parameters.

## Building and Testing

Run the usual cargo commands:

```bash
cargo build
cargo test
```

Note: building requires the `hand-isomorphism-rust` and `holdem-hand-evaluator` dependencies. They are pulled from external repositories and must be available for the build to succeed.
