use rand::{seq::SliceRandom, Rng};

use crate::board::{Player, Board};

/*
 * FIXME This would be prettier in main.rs, but currently broken
pub fn minimax(max_depth: u32) -> Box<dyn Fn(&Board) -> usize> {
    Box::new(move |state: &Board| alpha_beta_search(state, max_depth))
}
*/

#[allow(dead_code)]
pub fn alpha_beta_search(state: &Board, max_depth: u32) -> usize {
    let player = state.turn();
    let (_, game_move) = max_value(state, player, f64::NEG_INFINITY, f64::INFINITY, max_depth);
    game_move.unwrap()
}

/// Returns the utility and a move.
fn max_value(
    state: &Board,
    player: Player,
    mut alpha: f64,
    beta: f64,
    depth: u32,
) -> (f64, Option<usize>) {
    if depth == 0 || state.game_over() {
        return (evaluate(state, player), None);
    }
    let mut v = f64::NEG_INFINITY;
    let mut game_move: Option<usize> = None;
    let mut rng = rand::thread_rng();
    let mut actions = state.actions();
    actions.shuffle(&mut rng);
    for action in actions {
        let (v2, _) = min_value(&state.result(action), player, alpha, beta, depth - 1);
        if v2 > v {
            v = v2;
            game_move = Some(action);
            alpha = f64::max(alpha, v);
        }
        if v >= beta {
            return (v, game_move);
        }
    }
    (v, game_move)
}
fn min_value(
    state: &Board,
    player: Player,
    alpha: f64,
    mut beta: f64,
    depth: u32,
) -> (f64, Option<usize>) {
    if depth == 0 || state.game_over() {
        return (evaluate(state, player), None);
    }
    let mut v = f64::INFINITY;
    let mut game_move: Option<usize> = None;
    let mut rng = rand::thread_rng();
    let mut actions = state.actions();
    actions.shuffle(&mut rng);
    for action in actions {
        let (v2, _) = max_value(&state.result(action), player, alpha, beta, depth - 1);
        if v2 < v {
            v = v2;
            game_move = Some(action);
            beta = f64::min(beta, v);
        }
        if v <= alpha {
            return (v, game_move);
        }
    }
    (v, game_move)
}

fn evaluate(state: &Board, player: Player) -> f64 {
    if state.game_over() {
        utility(state, player)
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(-1f64..1f64)
    }
}

fn utility(state: &Board, player: Player) -> f64 {
    match state.winner() {
        Some(winner) => {
            if winner == player {
                1f64
            } else {
                -1f64
            }
        }
        None => 0f64,
    }
}
