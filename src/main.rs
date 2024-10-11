mod board;
#[cfg(test)]
mod tests;
use std::io::{self, Write};

fn game_loop() {
    let mut game = board::Board::new();
    while !game.game_over() {
        let mut guess = String::new();
        println!("{game}");
        print!("player {} Please select a hole (0..7): ", game.turn());
        io::stdout().flush().expect("flush failed?");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let mut hole = guess.trim().parse::<usize>();
        while !match hole {
            Ok(num) if game.valid_move(num) => {
                game.place(num).unwrap();
                true
            }
            _ => false,
        } {
            println!("Invalid move!");
            print!("player {} Please select a hole (0..7): ", game.turn());
            io::stdout().flush().expect("flush failed?");
            guess.clear();
            io::stdin()
                .read_line(&mut guess)
                .expect("failed to read_line");
            hole = guess.trim().parse::<usize>();
        }
    }
    println!("{game}");
    match game.winner() {
        Some(player) => println!("Player {} has won!", player),
        None => println!("The game was a draw"),
    }
}

fn main() {
    game_loop()
}
