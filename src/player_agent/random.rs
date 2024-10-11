use crate::board::Board;
use rand::{thread_rng, Rng};

pub(crate) fn random_move(game: &Board) -> usize {
    let mut rng = thread_rng();
    let mut hole: usize = rng.gen_range(0..8);
    while !game.valid_move(hole) {
        hole = rng.gen_range(0..8)
    }
    hole
}
