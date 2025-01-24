mod board;
mod player_agent;
mod game;
#[cfg(test)]
mod tests;
use std::io;

use game::*;
const MAX_DEPTH: u32 = 10;


fn main() -> io::Result<()> {
    //console_game();
    tui()
}
