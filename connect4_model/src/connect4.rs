#![allow(non_snake_case)]
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiscType {
    Empty,
    Red,
    Yellow,
    T,
    O,
}

impl Display for DiscType {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            DiscType::Red => write!(w, "R"),
            DiscType::Yellow => write!(w, "Y"),
            DiscType::T => write!(w, "T"),
            DiscType::O => write!(w, "O"),
            DiscType::Empty => write!(w, "E"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PlayerType {
    Human,
    AI
}

pub enum GameEvent {
    PlaceSuccess(usize),
    Player1Win(usize),
    Player2Win(usize),
    Draw(usize),
    PlaceColumnFull,
    UnexpectedErr,
}


#[derive(Debug, PartialEq)]
pub struct Player {
    pub player_type: PlayerType,
    pub disc_type: DiscType
}

impl Player {
    pub fn new(player_type: PlayerType, disc_type: DiscType) -> Self {
        Self {
            player_type,
            disc_type
        }
    }

    pub fn change_disc_type(&mut self, new_type: DiscType) {
        self.disc_type = new_type;
    }
}

// pub struct Disc {
//     position_row: usize,
//     position_column: usize,
//     disc_type: DiscType,
//     belong_to: Player
// }

pub struct Board {
    board_rows: usize,
    board_columns: usize,
    board: Vec<Vec<DiscType>>
}

impl Board {
    pub fn new(board_rows: usize, board_columns: usize) -> Self {
        Self {
            board_rows,
            board_columns,
            board: vec![vec![DiscType::Empty; board_columns]; board_rows]
        }
    }

    /// Arg: 
    ///     column: the column of the player that wants to place the disc
    ///     disc_type: the type of the disc to place
    /// Return: 
    ///     GameEvent: one of the custom events defined
    pub fn place_disc(&mut self, column: usize, disc_type: DiscType) -> GameEvent {
        if self.board[0][column] != DiscType::Empty {
            return GameEvent::PlaceColumnFull
        }

        for i in (0..self.board_rows).rev() {
            if self.board[i][column] == DiscType::Empty {
                self.board[i][column] = disc_type;
                return GameEvent::PlaceSuccess(i as usize);
            }
        }

        // should never reach here
        GameEvent::UnexpectedErr
    }

    pub fn is_full(&self) -> bool {
        let mut count = 0;
        for i in 0..self.board_columns {
            if self.board[0][i] != DiscType::Empty {
                count += 1;
            }
        }

        count == self.board_rows
    }

    pub fn is_connect4(&self, check_disc_type: DiscType) -> bool {
        let (mut horizontal_count, mut vertical_count, mut left_diagonal_count, mut right_diagonal_count) = (0, 0, 0, 0);

        for i in 0..self.board_rows {
            for j in 0..self.board_columns {
                horizontal_count = 0;
                vertical_count = 0;
                left_diagonal_count = 0;
                right_diagonal_count = 0;
                for k in 0..4 {
                    if j + k < self.board_columns && self.board[i][j + k] == check_disc_type {
                        horizontal_count += 1; 
                    }

                    if i + k < self.board_rows && self.board[i + k][j] == check_disc_type {
                        vertical_count += 1;
                    }

                    if i + k < self.board_rows && j + k < self.board_columns && self.board[i + k][j + k] == check_disc_type {
                        left_diagonal_count += 1;
                    }

                    if (i as isize - k as isize) >= 0 && j + k < self.board_columns && self.board[i - k][j + k] == check_disc_type {
                        right_diagonal_count += 1;
                    }
                }

                if horizontal_count == 4 || vertical_count == 4 || left_diagonal_count == 4 || right_diagonal_count == 4 {
                    return true
                }
            }
        }

        false
    }
}

impl Display for Board {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for i in 0..self.board_rows {
            for j in 0..self.board_columns {
                write!(w, "{}", self.board[i][j])?;
                write!(w, " ")?;
            }
            write!(w, "\n")?;
        }
        write!(w, "\n")
    }
}

pub struct Connect4 {
    pub game_board: Board,
    pub player1: Player,
    pub player2: Player,
    pub current_player: usize
}

impl Connect4 {
    pub fn new(board_rows: usize, board_columns: usize, vs_ai: bool) -> Self {
        let player2_type = if vs_ai {PlayerType::AI} else {PlayerType::Human};

        Self {
            game_board: Board::new(board_rows, board_columns),
            player1: Player::new(PlayerType::Human, DiscType::Red),
            player2: Player::new(player2_type, DiscType::Yellow),
            current_player: 1
        }
    }

    fn switch_turn(&mut self) {
        if self.current_player == 1 {
            self.current_player = 2
        } else {
            self.current_player = 1
        }
    }

    fn get_current_disc_type(&self) -> DiscType {
        if self.current_player == 1 {
            self.player1.disc_type
        } else {
            self.player2.disc_type
        }
    }

    pub fn place_disc(&mut self, column: usize) -> GameEvent {
        let game_event = self.game_board.place_disc(column, self.get_current_disc_type());
        match game_event {
            GameEvent::PlaceSuccess(row) => {
                self.check(row)
            },
            _ => return game_event
        }
    }

    pub fn check(&mut self, row: usize) -> GameEvent {
        if self.game_board.is_full() {
            return GameEvent::Draw(row)
        } else if self.game_board.is_connect4(self.get_current_disc_type()) {
            if self.current_player == 1 {
                return GameEvent::Player1Win(row)
            } else {
                return GameEvent::Player2Win(row)
            }
        } else {
            self.switch_turn();
            return GameEvent::PlaceSuccess(row)
        }
    }
}