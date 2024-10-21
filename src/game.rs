use std::io;

use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};
//use ratatui::widgets::StatefulWidget;
use ratatui::DefaultTerminal;
use ratatui::{prelude::*, widgets::*};

use crate::MAX_DEPTH;
use crate::board::{Board, LENGTH, Coin};
#[allow(unused_imports)]
use crate::player_agent::human::human;
#[allow(unused_imports)]
use crate::player_agent::minimax::alpha_beta_search;
#[allow(unused_imports)]
use crate::player_agent::random::random_move;
use crate::player_agent::PlayerAgent;

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

pub(crate) fn console_game() {
    println!("reduce MAX_DEPTH in main.rs if the game is too slow.");
    let player_2 = |game: &Board| alpha_beta_search(game, MAX_DEPTH);
    game_loop(human, player_2, true)
}

pub(crate) fn tui() -> io::Result<()> {
    //let player_2 = |game: &Board| alpha_beta_search(game, MAX_DEPTH);
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = game_loop_tui(terminal);
    ratatui::restore();
    app_result
}
const PLACEMENT_KEY: [KeyCode;2] = [KeyCode::Enter, KeyCode::Down];
fn game_loop_tui(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut state = BoardState::default();
    let mut game = Board::new();
    loop {
        terminal.draw(|frame| {
            let b = BoardWidget::new(game.clone());
            frame.render_stateful_widget(b, frame.area(), &mut state);
        })?;
        //TODO refactor into new function
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(())
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Right {
                state.selection += 1;
                state.selection %= 8;
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Left {
                if state.selection == 0 {
                    state.selection = 7;
                } else {
                    state.selection -= 1;
                }
            }
            if key.kind == KeyEventKind::Press && PLACEMENT_KEY.contains(&key.code) {
                game.place(state.selection.into())
                    .expect("Selection should be a valid move");
                if game.game_over() {
                    return Ok(())
                }
            }

        }
    }

}

//clone the board into this each frame
struct BoardWidget {
    board: Board
}

impl BoardWidget {
    fn new(board: Board) -> Self {
        Self {board}
    }
}

/*
impl Default for BoardWidget {
    fn default() -> Self {
        Self { board: Board::default() }
    }
}
*/

impl StatefulWidget for BoardWidget {
    type State = BoardState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let row_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, LENGTH.try_into().unwrap()); LENGTH
            ])
            .split(area);
        for row in 0..LENGTH {
            let column_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Ratio(1, LENGTH.try_into().unwrap()); LENGTH
                ])
                .split(row_layout[row]);
            for column in 0..LENGTH {
                if row == 0 && column == state.selection.into() {
                    let selection_style = Style::default()
                        .add_modifier(Modifier::RAPID_BLINK)
                        .on_green();
                    let selection_block = Block::bordered()
                        .style(selection_style);

                    selection_block.render(column_layout[column], buf);
                };
                let spot = match self.board[(column, 7-row)] {
                    Some(player) => {

                        match player {
                            Coin::Red => Block::bordered()
                                .on_red(),
                            Coin::Blue => Block::bordered()
                                .on_blue(),
                        }
                    }
                    None => Block::bordered(),
                };
                spot.render(column_layout[column], buf)
            }
                
        } 
    }
}


struct BoardState {
    selection: u8
}
impl BoardState {
    fn new(selection: u8) -> Self {
        Self {selection}
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new( 0)
    }
}
