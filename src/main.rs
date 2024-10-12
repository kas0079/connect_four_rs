mod board;
mod player_agent;
#[cfg(test)]
mod tests;
use board::Board;
#[allow(unused_imports)]
use player_agent::human::human;
#[allow(unused_imports)]
use player_agent::random::random_move;
use player_agent::PlayerAgent;
#[allow(unused_imports)]
use player_agent::minimax::alpha_beta_search;

const MAX_DEPTH: u32 = 8;

fn game_loop(player_1: PlayerAgent, player_2: PlayerAgent, print_game: bool) {
    let mut game = Board::new();
    let mut players = [player_1, player_2].into_iter().cycle();
    while !game.game_over() {
        if print_game {
            println!("{game}");
        }
        let turn_player = players.next().unwrap();
        let player_move = turn_player(&game);
        game.place(player_move).unwrap()
    }
    println!("{game}");
    match game.winner() {
        Some(player) => println!("Player {} has won!", player),
        None => println!("The game was a draw"),
    }
}

fn main() {
    println!("reduce MAX_DEPTH in main.rs if the game is too slow.");
    let player_2 = |game: &Board| alpha_beta_search(game, MAX_DEPTH);
    game_loop(human, player_2, true)
}
