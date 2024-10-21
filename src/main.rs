mod board;
mod player_agent;
mod game;
#[cfg(test)]
mod tests;
use game::console_game;
const MAX_DEPTH: u32 = 10;


fn main() {
    console_game();
    
}
