mod board;
mod player_agent;
#[cfg(test)]
mod tests;
use board::Board;
use player_agent::human::human;
use player_agent::random::random_move;

fn game_loop(player_1: fn(&Board) -> usize, player_2: fn(&Board) -> usize, print_game: bool) {
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
    game_loop(human, random_move, true)
}
