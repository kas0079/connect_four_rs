use crate::board;
use std::io::{self, Write};
#[allow(dead_code)]
fn get_move(game: &board::Board) -> Result<usize, std::num::ParseIntError> {
    let mut guess = String::new();
    print!("Player {} Please select a hole (0..7): ", game.turn());
    io::stdout().flush().expect("flush failed?");
    guess.clear();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read_line");
    guess.trim().parse::<usize>()
}

#[allow(dead_code)]
pub(crate) fn human(game: &board::Board) -> usize {
    let mut hole = get_move(game);
    loop {
        match hole {
            Ok(n) if game.valid_move(n) => return n,
            _ => (),
        }
        println!("Invalid move!");
        hole = get_move(game);
    }
}
