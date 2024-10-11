pub(crate) mod human;
pub(crate) mod random;
use crate::board::Board;
pub type PlayerAgent = fn(&Board) -> usize;
