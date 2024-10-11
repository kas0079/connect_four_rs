use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Coin {
    Red,
    Blue,
}

type Player = Coin;

#[derive(Debug)]
pub(crate) struct Board {
    placements: [[Option<Coin>; 8]; 8],
    turn: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            placements: [[None; 8]; 8],
            turn: Coin::Red,
        }
    }

    pub fn turn(&self) -> Player {
        self.turn
    }

    pub fn draw(&self) -> bool {
        self.placements.iter().all(|hole| hole[7].is_some())
    }

    pub fn game_over(&self) -> bool {
        self.draw() || self.winner().is_some()
    }

    pub fn valid_move(&self, hole: usize) -> bool {
        if hole > 7 {
            return false;
        }

        if self.placements[hole][7].is_some() {
            return false;
        }

        if self.game_over() {
            return false;
        }
        true
    }
    #[allow(dead_code)]
    pub(crate) fn put_debug(&mut self, x: usize, y: usize) {
        self.placements[x][y] = Some(Coin::Red)
    }

    pub fn place(&mut self, hole: usize) -> Result<(), ()> {
        if !self.valid_move(hole) {
            return Err(());
        }

        for spot in &mut self.placements[hole] {
            if spot.is_none() {
                *spot = Some(self.turn);
                break;
            }
        }

        match self.turn {
            Coin::Red => self.turn = Coin::Blue,
            Coin::Blue => self.turn = Coin::Red,
        }
        Ok(())
    }

    pub fn winner(&self) -> Option<Player> {
        let _four_in_row_old = |coins: [Option<Coin>; 4]| {
            if coins[0].is_some() && coins.iter().all(|c| c == &coins[0]) {
                coins[0]
            } else {
                None
            }
        };
        let four_in_row = |coins: &[Option<Coin>]| {
            if coins[0].is_some() && coins.iter().all(|c| c == &coins[0]) {
                coins[0]
            } else {
                None
            }
        };

        //test vertical
        for hole in &self.placements {
            let windows = hole.windows(4);
            for window in windows {
                let result = four_in_row(window);
                if result.is_some() {
                    return result;
                }
            }
        }

        //test horizontal
        for line in self.lines() {
            let windows = line.windows(4);
            for window in windows {
                let result = four_in_row(window);
                if result.is_some() {
                    return result;
                }
            }
        }

        //test diagonal
        //idea: maybe do one of the two previous with rotated iterators
        //(rot(1,[1,2,3]) == [3,1,2])
        for x in 0..5 {
            for y in 0..5 {
                let placements = self.placements;
                let diagonal = [
                    placements[x][y],
                    placements[x + 1][y + 1],
                    placements[x + 2][y + 2],
                    placements[x + 3][y + 3],
                ];
                let result = four_in_row(&diagonal);
                if result.is_some() {
                    return result;
                }
            }
        }

        for x in 0..5 {
            for y in (3..8).rev() {
                let placements = self.placements;
                let reverse_diagonal = [
                    placements[x][y],
                    placements[x + 1][y - 1],
                    placements[x + 2][y - 2],
                    placements[x + 3][y - 3],
                ];
                let reverse_result = four_in_row(&reverse_diagonal);
                if reverse_result.is_some() {
                    return reverse_result;
                }
            }
        }
        None
    }

    fn lines(&self) -> LineIter<'_> {
        return LineIter::new(self);
    }
}

impl Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "r"),
            Self::Blue => write!(f, "b"),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut a = String::new();
        for x in (0..self.placements.len()).rev() {
            for y in 0..self.placements[0].len() {
                let coin_string = match self.placements[y][x] {
                    Some(player) => &player.to_string(),
                    None => " ",
                };
                if y == 0 {
                    a.push('|')
                }
                a.push_str(coin_string);
                a.push('|');
            }
            if x != 0 {
                a.push('\n')
            }
        }
        a.push_str("\n 0 1 2 3 4 5 6 7");
        write!(f, "{a}")
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

struct LineIter<'a> {
    line: usize,
    board: &'a Board,
}
impl<'a> LineIter<'a> {
    fn new(board: &'a Board) -> Self {
        LineIter { line: 0, board }
    }
}
impl<'a> Iterator for LineIter<'a> {
    type Item = [Option<Coin>; 8];

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.line;
        self.line += 1;

        if self.line <= 8 {
            let placements = &self.board.placements;
            //hate this
            Some([
                placements[0][line],
                placements[1][line],
                placements[2][line],
                placements[3][line],
                placements[4][line],
                placements[5][line],
                placements[6][line],
                placements[7][line],
            ])
        } else {
            None
        }
    }
}
