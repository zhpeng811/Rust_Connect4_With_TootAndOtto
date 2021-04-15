#![allow(dead_code)]

use rand::prelude::*;
use crate::game::{BoardGame, GameEvent};
use crate::board::{Board};
use crate::disc::{DiscType};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

// needed to <Select> component display
impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Difficulty::Easy => String::from("Easy"),
            Difficulty::Medium => String::from("Medium"),
            Difficulty::Hard => String::from("Hard"),
        }
    }
}

impl Difficulty {
    pub fn to_vec() -> Vec<Difficulty> {
        vec![Difficulty::Easy, Difficulty::Medium, Difficulty::Hard]
    }
}

pub struct Connect4AI {
    board_rows: usize,
    board_columns: usize,
    max_depth: usize,
    difficulty: Difficulty,
    alpha: isize,
    beta: isize
}

impl Connect4AI {
    pub fn new(board_rows: usize, board_columns: usize, difficulty: Difficulty) -> Self {
        Self {
            board_rows,
            board_columns,
            max_depth: 6,
            difficulty,
            alpha: std::isize::MIN,
            beta: std::isize::MAX
        }
    }

    fn random_gen(&self, game_board: Board) -> usize {
        let valid_columns = game_board.get_valid_columns();
        *valid_columns.choose(&mut thread_rng()).unwrap()
    }

    pub fn find_best_move(&self, game: BoardGame) -> usize {
        match self.difficulty {
            Difficulty::Easy => {
                // pure random
                return self.random_gen(game.game_board.clone())
            },
            Difficulty::Medium => {
                // find if there's a winning move for AI
                // if not choose randomly
                let winning_move = self.find_winning_move(game.game_board.clone());
                if winning_move >= 0 {
                    return winning_move as usize
                } else {
                    return self.random_gen(game.game_board.clone())
                }
            }
            Difficulty::Hard => {
                // minmax algorithm for Connect 4, NOT working, DO NOT use
                // return self.minmax(game.game_board.clone(), self.max_depth, true).0

                // find if there's a winning move for AI
                let winning_move = self.find_winning_move(game.game_board.clone());
                if winning_move >= 0 {
                    return winning_move as usize
                } else { // find if there's a blocking move for player
                    let blocking_move = self.find_blocking_move(game.game_board.clone());
                    if blocking_move >= 0 {
                        return blocking_move as usize
                    } else {
                        return self.random_gen(game.game_board.clone())
                    }
                }
            },
            
        }
    }

    pub fn find_winning_move(&self, game_board: Board) -> isize {
        // find if there's a move that causes AI to win
        let valid_columns = game_board.get_valid_columns();
        for col in valid_columns.clone() {
            let mut clone_board = game_board.clone();
            clone_board.place_disc(col, DiscType::Yellow);
            if clone_board.is_connect4(DiscType::Yellow) {
                return col as isize
            }
        }

        -1
    }

    fn find_blocking_move(&self, game_board: Board) -> isize {
        // find if there's a move that can prevent player to win
        let valid_columns = game_board.get_valid_columns();
        for col in valid_columns {
            let mut clone_board = game_board.clone();
            clone_board.place_disc(col, DiscType::Red);
            if clone_board.is_connect4(DiscType::Red) {
                return col as isize
            }
        }

        -1
    }

    // hard algorithm from the following implementation: https://github.com/KeithGalli/Connect4-Python/blob/master/connect4_with_ai.py
    fn evaluate_window(&self, window: &Vec<DiscType>, disc_type: DiscType) -> isize {
        let mut score: isize = 0;

        let mut disc_count = 0;
        let mut empty_count = 0;
        let mut oppo_count = 0;
        for i in 0..window.len() {
            if window[i] == disc_type {
                disc_count += 1;
            } else if window[i] == DiscType::Empty {
                empty_count += 1;
            } else {
                oppo_count += 1;
            }
        }

        if disc_count >= 4 {
            score += 100;
        } else if disc_count == 3 && empty_count == 1 {
            score += 5;
        } else if disc_count == 2 {
            score += 2;
        } 

        if oppo_count == 3 && empty_count == 1 {
            score -= 4;
        }

        score
    }

    fn score_position(&self, game_board: &Board, disc_type: DiscType) -> isize {
        let mut center_count = 0;
        for i in 0..self.board_rows {
            if game_board.board[i][self.board_columns / 2] == disc_type {
                center_count += 1;
            }
        }
        let mut score = center_count * 3;

        for r in 0..self.board_rows {
            let row_array = &game_board.board[r];
            for c in 0..self.board_columns - 3 {
                let mut window: Vec<DiscType> = Vec::new();
                for i in 0..4 {
                    window.push(row_array[c + i]);
                }
                score += self.evaluate_window(&window, disc_type);
            }
        }

        for c in 0..self.board_columns {
            let mut col_array: Vec<DiscType> = Vec::new();
            for i in 0..self.board_rows {
                col_array.push(game_board.board[i][c]);
            }
            for r in 0..self.board_rows - 3 {
                let mut window: Vec<DiscType> = Vec::new();
                for i in 0..4 {
                    window.push(col_array[r + i]);
                }
                score += self.evaluate_window(&window, disc_type);
            }
        }

        for r in 0..self.board_rows - 3 {
            for c in 0..self.board_columns - 3 {
                let mut window: Vec<DiscType> = Vec::new();
                for i in 0..4 {
                    window.push(game_board.board[r + i][c + i]);
                }
                score += self.evaluate_window(&window, disc_type);
            }
        }

        for r in 0..self.board_rows - 3 {
            for c in 0..self.board_columns - 3 {
                let mut window: Vec<DiscType> = Vec::new();
                for i in 0..4 {
                    window.push(game_board.board[r + 3 - i][c + i]);
                }
                score += self.evaluate_window(&window, disc_type);
            }
        }

        score
    }

    fn minmax(&self, board: Board, depth: usize, is_ai: bool) -> (usize, isize) {
        if board.is_connect4(DiscType::Red) {
            return (66, std::isize::MIN)
        } else if board.is_connect4(DiscType::Yellow) {
            return (66, std::isize::MAX)
        } else if board.is_full() {
            return (66, 0)
        } else if depth == 0 {
            return (66, self.score_position(&board, DiscType::Yellow))
        }

        let valid_columns = board.get_valid_columns();
        if is_ai {
            let mut score = std::isize::MIN;
            let mut column: usize = *valid_columns.choose(&mut rand::thread_rng()).unwrap();
            for col in valid_columns {
                let mut clone_board = board.clone();
                clone_board.place_disc(col, DiscType::Yellow);
                let new_score = self.minmax(clone_board, depth - 1, !is_ai).1;
                if new_score > score {
                    score = new_score;
                    column = col;
                }
                let alpha = std::cmp::max(self.alpha, score);
                if alpha >= self.beta {
                    break;
                }
            }
            return (column, score)
        } else {
            let mut score = std::isize::MAX;
            let mut column: usize = *valid_columns.choose(&mut rand::thread_rng()).unwrap();
            for col in valid_columns {
                let mut clone_board = board.clone();
                clone_board.place_disc(col, DiscType::Red);
                let new_score = self.minmax(clone_board, depth - 1, !is_ai).1;
                if new_score < score {
                    score = new_score;
                    column = col;
                }
                let beta = std::cmp::min(self.beta, score);
                if self.alpha >= beta {
                    break;
                }
            }
            return (column, score)
        }
    }
}

pub struct TootOttoAI {
    board_rows: usize,
    board_columns: usize,
    difficulty: Difficulty
}

impl TootOttoAI {
    pub fn new(board_rows: usize, board_columns: usize, difficulty: Difficulty) -> Self {
        Self {
            board_rows,
            board_columns,
            difficulty
        }
    }

    fn random_gen(&self, game_board: Board) -> (usize, DiscType) {
        let mut rng = thread_rng();
        let valid_columns = game_board.get_valid_columns();
        let column = *valid_columns.choose(&mut rng).unwrap();
        let disc_type = {
            if rng.gen_range(0, 2) == 0 {
                DiscType::T
            } else {
                DiscType::O
            }
        };
        (column, disc_type)
    }

    pub fn find_best_move(&self, game: BoardGame) -> (usize, DiscType) {
        match self.difficulty {
            Difficulty::Easy => {
                // pure random
                return self.random_gen(game.game_board.clone())
            },
            Difficulty::Medium => {
                // find if there's a winning move for AI
                // if not choose randomly
                let (winning_move, disc_type) = self.find_winning_move(game.game_board.clone());
                if winning_move >= 0 {
                    return (winning_move as usize, disc_type)
                } else {
                    return self.random_gen(game.game_board.clone())
                }
            }
            Difficulty::Hard => {
                // find if there's a winning move for AI
                let (winning_move, disc_type) = self.find_winning_move(game.game_board.clone());
                if winning_move >= 0 {
                    return (winning_move as usize, disc_type)
                } else { // find if there's a blocking move for player
                    let (blocking_move, disc_type) = self.find_blocking_move(game.game_board.clone());
                    if blocking_move >= 0 {
                        return (blocking_move as usize, disc_type)
                    } else {
                        return self.random_gen(game.game_board.clone())
                    }
                }
            },
            
        }
    }

    fn find_winning_move(&self, game_board: Board) -> (isize, DiscType) {
        // find if there's a move that causes AI(OTTO) to win
        let valid_columns = game_board.get_valid_columns();
        // first check for 'T'
        for col in valid_columns.clone() {
            let mut clone_board = game_board.clone();
            clone_board.place_disc(col, DiscType::T);
            match clone_board.is_toot_or_otto() {
                GameEvent::IsOTTO => return (col as isize, DiscType::T),
                _ => ()
            }
        }

        // then check for 'O'
        for col in valid_columns.clone() {
            let mut clone_board = game_board.clone();
            clone_board.place_disc(col, DiscType::O);
            match clone_board.is_toot_or_otto() {
                GameEvent::IsOTTO => return (col as isize, DiscType::O),
                _ => ()
            }
        }

        (-1, DiscType::Empty)
    }

    fn find_blocking_move(&self, game_board: Board) -> (isize, DiscType) {
        // find if there's a move that can prevent player(TOOT) to win
        let valid_columns = game_board.get_valid_columns();
        // first check for 'T'
        for col in valid_columns.clone() {
            let mut clone_board = game_board.clone();
            clone_board.place_disc(col, DiscType::T);
            match clone_board.is_toot_or_otto() {
                GameEvent::IsTOOT => return (col as isize, DiscType::O), // place the opposite disc to prevent winning
                _ => ()
            }
        }

        // then check for 'O'
        for col in valid_columns.clone() {
            let mut clone_board = game_board.clone();
            clone_board.place_disc(col, DiscType::O);
            match clone_board.is_toot_or_otto() {
                GameEvent::IsTOOT => return (col as isize, DiscType::T),
                _ => ()
            }
        }

        (-1, DiscType::Empty)
    }
}