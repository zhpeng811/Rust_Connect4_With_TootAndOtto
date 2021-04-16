pub use crate::disc::DiscType;
pub use crate::game::GameEvent;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Board {
    pub board_rows: usize,
    pub board_columns: usize,
    pub board: Vec<Vec<DiscType>>
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
                return GameEvent::PlaceSuccess(i as usize)
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

        count == self.board_columns
    }

    pub fn get_valid_columns(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for i in 0..self.board_columns {
            if self.board[0][i] == DiscType::Empty {
                result.push(i);
            }
        }

        result
    }

    pub fn is_connect4(&self, check_disc_type: DiscType) -> bool {
        let (mut horizontal_count, mut vertical_count, mut left_diagonal_count, mut right_diagonal_count);

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

    pub fn is_toot_or_otto(&self) -> GameEvent {
        let (mut horizontal_count, mut vertical_count, mut left_diagonal_count, mut right_diagonal_count);


        for i in 0..self.board_rows {
            for j in 0..self.board_columns {
                horizontal_count = [DiscType::Empty; 4];
                vertical_count = [DiscType::Empty; 4];
                left_diagonal_count = [DiscType::Empty; 4];
                right_diagonal_count = [DiscType::Empty; 4];
                for k in 0..4 {
                    if j + k < self.board_columns {
                        horizontal_count[k] = self.board[i][j + k];
                    }

                    if i + k < self.board_rows {
                        vertical_count[k] = self.board[i + k][j];
                    }

                    if i + k < self.board_rows && j + k < self.board_columns {
                        left_diagonal_count[k] = self.board[i + k][j + k];
                    }

                    if (i as isize - k as isize) >= 0 && j + k < self.board_columns {
                        right_diagonal_count[k] = self.board[i - k][j + k];
                    }
                }

                if  horizontal_count[0] == DiscType::T && 
                    horizontal_count[1] == DiscType::O &&
                    horizontal_count[2] == DiscType::O && 
                    horizontal_count[3] == DiscType::T ||
                    vertical_count[0] == DiscType::T && 
                    vertical_count[1] == DiscType::O &&
                    vertical_count[2] == DiscType::O && 
                    vertical_count[3] == DiscType::T ||
                    left_diagonal_count[0] == DiscType::T && 
                    left_diagonal_count[1] == DiscType::O &&
                    left_diagonal_count[2] == DiscType::O && 
                    left_diagonal_count[3] == DiscType::T ||
                    right_diagonal_count[0] == DiscType::T && 
                    right_diagonal_count[1] == DiscType::O &&
                    right_diagonal_count[2] == DiscType::O && 
                    right_diagonal_count[3] == DiscType::T
                {
                    return GameEvent::IsTOOT
                } else if 
                    horizontal_count[0] == DiscType::O && 
                    horizontal_count[1] == DiscType::T &&
                    horizontal_count[2] == DiscType::T && 
                    horizontal_count[3] == DiscType::O ||
                    vertical_count[0] == DiscType::O && 
                    vertical_count[1] == DiscType::T &&
                    vertical_count[2] == DiscType::T && 
                    vertical_count[3] == DiscType::O ||
                    left_diagonal_count[0] == DiscType::O && 
                    left_diagonal_count[1] == DiscType::T &&
                    left_diagonal_count[2] == DiscType::T && 
                    left_diagonal_count[3] == DiscType::O ||
                    right_diagonal_count[0] == DiscType::O && 
                    right_diagonal_count[1] == DiscType::T &&
                    right_diagonal_count[2] == DiscType::T && 
                    right_diagonal_count[3] == DiscType::O
                {
                    return GameEvent::IsOTTO
                }
            }
        }

        GameEvent::Ongoing
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
