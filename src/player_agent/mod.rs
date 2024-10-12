pub(crate) mod human;
pub(crate) mod random;
pub(crate) mod minimax;
use crate::board::Board;
pub type PlayerAgent = fn(&Board) -> usize;
