#![allow(dead_code)]

use rand::prelude::*;
use crate::game::{BoardGame, GameEvent};
use crate::board::{Board};
use crate::disc::{DiscType};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Insane,
}

// needed to <Select> component display
impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Difficulty::Easy => String::from("Easy"),
            Difficulty::Medium => String::from("Medium"),
            Difficulty::Hard => String::from("Hard"),
            Difficulty::Insane => String::from("Insane")
        }
    }
}

impl Difficulty {
    pub fn to_vec() -> Vec<Difficulty> {
        vec![Difficulty::Easy, Difficulty::Medium, Difficulty::Hard, Difficulty::Insane]
    }
}

pub struct Connect4AI {
    board_rows: usize,
    board_columns: usize,
    max_depth: usize,
    difficulty: Difficulty,
    alpha: isize,
    beta: isize,
    score_board: Vec<Vec<i64>>
}

impl Connect4AI {
    pub fn new(board_rows: usize, board_columns: usize, difficulty: Difficulty) -> Self {
        let mut map: Vec<Vec<i64>> = vec![vec![0; board_columns]; board_rows];
        Self {
            board_rows,
            board_columns,
            max_depth: 6,
            difficulty,
            alpha: std::isize::MIN,
            beta: std::isize::MAX,
            score_board: map,
        }
    }

    fn random_gen(&self, game_board: Board) -> usize {
        let valid_columns = game_board.get_valid_columns();
        match valid_columns.choose(&mut thread_rng()) {
            Some(column) => return *column,
            None => return 0 // not gonna be used
        }
    }

    fn convert_board(&mut self, board: Board) {
        for y in 0..self.board_rows {
            for x in 0..self.board_columns {
                self.score_board[y][x] = if board.board[y][x] == DiscType::Red {
                    1
                } else if board.board[y][x] == DiscType::Yellow {
                    -1
                } else {
                    0
                }
            }
        }
    }

    pub fn find_best_move(&mut self, game: BoardGame) -> usize {
        self.convert_board(game.game_board.clone());
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
            Difficulty::Insane => {
                let val_choice = self.max_state(-1, &self.score_board, 0, -100000000007, 100000000007);
                let val = val_choice.0;
                let choice = val_choice.1;
                if choice < 0 || choice as usize > self.board_columns {
                    return self.random_gen(game.game_board.clone());
                }
                return choice as usize;
            }
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

    pub fn value(
        &self,
        ai_move_value: i64,
        state: &Vec<Vec<i64>>,
        depth: i64,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, i64) {
        let val = self.check_state(state);
        let max_depth = match self.difficulty {
            Easy => 1,
            Medium => 3,
            Hard => 5,
            Insane => 5,
        };
        if depth >= max_depth {
            // if slow (or memory consumption is high), lower the value
            let mut ret_val = 0;

            // if win, value = +inf
            let win_val = val.0;
            let chain_val = val.1 * ai_move_value;
            ret_val = chain_val;

            // If it lead to winning, then do it
            if win_val == 4 * ai_move_value {
                // AI win, AI wants to win of course
                ret_val = 999999;
            } else if win_val == 4 * ai_move_value * -1 {
                // AI lose, AI hates losing
                ret_val = 999999 * -1;
            }
            ret_val -= depth * depth;

            return (ret_val, -1);
        }

        let win = val.0;
        // if already won, then return the value right away
        if win == 4 * ai_move_value {
            // AI win, AI wants to win of course
            return (999999 - depth * depth, -1);
        }
        if win == 4 * ai_move_value * -1 {
            // AI lose, AI hates losing
            return (999999 * -1 - depth * depth, -1);
        }

        if depth % 2 == 0 {
            return self.min_state(ai_move_value, state, depth + 1, alpha, beta);
        }
        return self.max_state(ai_move_value, state, depth + 1, alpha, beta);
    }

    pub fn max_state(
        &self,
        ai_move_value: i64,
        state: &Vec<Vec<i64>>,
        depth: i64,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, i64) {
        let mut v = -100000000007;
        let mut new_move: i64 = -1;
        let mut move_queue = Vec::new();

        for j in 0..self.board_columns {
            let temp_state = self.fill_map(state, j, ai_move_value);
            if temp_state[0][0] != 999 {
                let temp_val = self.value(ai_move_value, &temp_state, depth, alpha, beta);
                if temp_val.0 > v {
                    v = temp_val.0;
                    new_move = j as i64;
                    move_queue = Vec::new();
                    move_queue.push(j);
                } else if temp_val.0 == v {
                    move_queue.push(j);
                }

                // alpha-beta pruning
                if v > beta {
                    new_move = self.choose(&move_queue);
                    return (v, new_move);
                }
                alpha = std::cmp::max(alpha, v);
            }
        }
        new_move = self.choose(&move_queue);
        println!("max state moves: {:?}", move_queue);
        return (v, new_move);
    }

    pub fn min_state(
        &self,
        ai_move_value: i64,
        state: &Vec<Vec<i64>>,
        depth: i64,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, i64) {
        let mut v = 100000000007;
        let mut new_move: i64 = -1;
        let mut move_queue = Vec::new();

        for j in 0..7 {
            let temp_state = self.fill_map(state, j, ai_move_value * -1);
            if temp_state[0][0] != 999 {
                let temp_val = self.value(ai_move_value, &temp_state, depth, alpha, beta);
                if temp_val.0 < v {
                    v = temp_val.0;
                    new_move = j as i64;
                    move_queue = Vec::new();
                    move_queue.push(j);
                } else if temp_val.0 == v {
                    move_queue.push(j);
                }

                // alpha-beta pruning
                if v < alpha {
                    new_move = self.choose(&move_queue);
                    return (v, new_move);
                }
                beta = std::cmp::min(beta, v);
            }
        }
        new_move = self.choose(&move_queue);

        return (v, new_move);
    }

    pub fn fill_map(&self, new_state: &Vec<Vec<i64>>, column: usize, value: i64) -> Vec<Vec<i64>> {
        let mut temp_map = new_state.clone();
        if temp_map[0][column] != 0 || column > 6 {
            temp_map[0][0] = 999; // error code
        }

        let mut done = false;
        let mut row = 0;

        for i in 0..5 {
            if temp_map[i + 1][column] != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = 5;
        }
        temp_map[row][column] = value;
        return temp_map;
    }

    pub fn get_random_val(&self, val: usize) -> usize {
        let mut rng = rand::thread_rng();
        let base: f64 = rng.gen();
        let max_val = val as f64;

        return (base * max_val).floor() as usize;
    }

    pub fn choose(&self, choice: &Vec<usize>) -> i64 {
        let index = self.get_random_val(choice.len());
        return choice[index] as i64;
    }

    pub fn check_state(&self, state: &Vec<Vec<i64>>) -> (i64, i64) {
        let mut win_val = 0;
        let mut chain_val = 0;
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (0, 0, 0, 0);
        for i in 0..6 {
            for j in 0..7 {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {
                    if j + k < 7 {
                        temp_r += state[i][j + k];
                    }

                    if i + k < 6 {
                        temp_b += state[i + k][j];
                    }

                    if i + k < 6 && j + k < 7 {
                        temp_br += state[i + k][j + k];
                    }

                    if i >= k && j + k < 7 {
                        temp_tr += state[i - k][j + k];
                    }
                }
                chain_val += temp_r * temp_r * temp_r;
                chain_val += temp_b * temp_b * temp_b;
                chain_val += temp_br * temp_br * temp_br;
                chain_val += temp_tr * temp_tr * temp_tr;

                if temp_r.abs() == 4 {
                    win_val = temp_r;
                } else if temp_b.abs() == 4 {
                    win_val = temp_b;
                } else if temp_br.abs() == 4 {
                    win_val = temp_br;
                } else if temp_tr.abs() == 4 {
                    win_val = temp_tr;
                }
            }
        }

        return (win_val, chain_val);
    }
}

pub struct TootOttoAI {
    board_rows: usize,
    board_columns: usize,
    difficulty: Difficulty,
    score_board: Vec<Vec<i64>>
}

impl TootOttoAI {
    pub fn new(board_rows: usize, board_columns: usize, difficulty: Difficulty) -> Self {
        let mut map: Vec<Vec<i64>> = vec![vec![0; board_columns]; board_rows];

        Self {
            board_rows,
            board_columns,
            difficulty,
            score_board: map,
        }
    }

    fn random_gen(&self, game_board: Board) -> (usize, DiscType) {
        let mut rng = thread_rng();
        let valid_columns = game_board.get_valid_columns();
        match valid_columns.choose(&mut rng) {
            Some(column) => {
                let disc_type = {
                    if rng.gen_range(0, 2) == 0 {
                        DiscType::T
                    } else {
                        DiscType::O
                    }
                };
                (*column, disc_type)
            },
            None => return (0, DiscType::Empty), // not gonna be used
        }
    }

    fn convert_board(&mut self, board: Board) {
        for y in 0..self.board_rows {
            for x in 0..self.board_columns {
                self.score_board[y][x] = if board.board[y][x] == DiscType::T {
                    1
                } else if board.board[y][x] == DiscType::O {
                    -1
                } else {
                    0
                }
            }
        }
    }

    pub fn find_best_move(&mut self, game: BoardGame) -> (usize, DiscType) {
        self.convert_board(game.game_board.clone());
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
            Difficulty::Insane => {
                let val_choice = self.max_state(&self.score_board, 0, -100000000007, 100000000007);
                let (val, (column, letter)) = val_choice;
                if column < 0 || column as usize > self.board_columns {
                    return self.random_gen(game.game_board.clone());
                }
                let ret_let = if letter == 'T' { DiscType::T } else { DiscType::O };
                return (column as usize, ret_let);
            }
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

    pub fn check_state(&self, state: &Vec<Vec<i64>>) -> (i64, i64) {
        let mut win_val = 0;
        let mut chain_val = 0;
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (0, 0, 0, 0);
        for i in 0..6 {
            for j in 0..7 {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {
                    // TODO may need to be flipped
                    let sign: i64 = if k == 0 || k == 3 { -1 } else { 1 };
                    if j + k < 7 {
                        temp_r += sign * state[i][j + k];
                    }
                    if i + k < 6 {
                        temp_b += sign * state[i + k][j];
                    }
                    if i + k < 6 && j + k < 7 {
                        temp_br += sign * state[i + k][j + k];
                    }

                    if i >= k && j + k < 7 {
                        temp_tr += sign * state[i - k][j + k];
                    }
                }
                chain_val += temp_r * temp_r * temp_r;
                chain_val += temp_b * temp_b * temp_b;
                chain_val += temp_br * temp_br * temp_br;
                chain_val += temp_tr * temp_tr * temp_tr;

                if temp_r.abs() == 4 {
                    win_val = temp_r;
                } else if temp_b.abs() == 4 {
                    win_val = temp_b;
                } else if temp_br.abs() == 4 {
                    win_val = temp_br;
                } else if temp_tr.abs() == 4 {
                    win_val = temp_tr;
                }
            }
        }
        return (win_val, chain_val);
    }

    pub fn value(&self, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64) -> (i64) {
        let val = self.check_state(state);
        let max_depth = match self.difficulty {
            Easy => 1,
            Medium => 3,
            Hard => 5,
            Insane => 5,
        };
        if depth >= max_depth {
            // if slow (or memory consumption is high), lower the value
            let mut ret_val = 0;

            // if win, value = +inf
            let win_val = val.0;
            let chain_val = val.1;
            ret_val = chain_val;

            // If it lead to winning, then do it
            if win_val == 4 {
                // AI win, AI wants to win of course
                ret_val = 999999;
            } else if win_val == 4 {
                // AI lose, AI hates losing
                ret_val = 999999 * -1;
            }
            ret_val -= depth * depth;

            return (ret_val);
        }

        let win = val.0;
        // if already won, then return the value right away
        if win == 4 {
            // AI win, AI wants to win of course
            return (999999 - depth * depth);
        }
        if win == -4 {
            // AI lose, AI hates losing
            return (-999999 - depth * depth);
        }

        if depth % 2 == 0 {
            return self.min_state(state, depth + 1, alpha, beta).0;
        } else {
            return self.max_state(state, depth + 1, alpha, beta).0;
        }
    }

    // TODO - add letter to AI search
    pub fn max_state(
        &self,
        state: &Vec<Vec<i64>>,
        depth: i64,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, (i64, char)) {
        let mut v = -100000000007;
        let mut new_move: (i64, char) = (-1, ' ');
        let mut move_queue = Vec::new();

        for letter in &['T', 'O'] {
            for j in 0..7 {
                let move_value = if *letter == 'T' { 1 } else { -1 };
                let temp_state = self.fill_map(state, j, move_value);
                if temp_state[0][0] != 999 {
                    let temp_val = self.value(&temp_state, depth, alpha, beta);
                    if temp_val > v {
                        v = temp_val;
                        new_move = (j as i64, *letter);
                        move_queue = Vec::new();
                        move_queue.push((j as i64, *letter));
                    } else if temp_val == v {
                        move_queue.push(((j as i64, *letter)));
                    }

                    // alpha-beta pruning
                    if v > beta {
                        new_move = self.choose(&move_queue);
                        return (v, new_move);
                    }
                    alpha = std::cmp::max(alpha, v);
                }
            }
        }
        new_move = self.choose(&move_queue);

        return (v, new_move);
    }

    // TODO - add letter choice to search
    pub fn min_state(
        &self,
        state: &Vec<Vec<i64>>,
        depth: i64,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, (i64, char)) {
        let mut v = 100000000007;
        let mut new_move: (i64, char) = (-1, ' ');
        let mut move_queue = Vec::new();

        for letter in &['T', 'O'] {
            for j in 0..7 {
                let move_value = if *letter == 'T' { 1 } else { -1 };
                let temp_state = self.fill_map(state, j, move_value);
                if temp_state[0][0] != 999 {
                    let temp_val = self.value(&temp_state, depth, alpha, beta);
                    if temp_val < v {
                        v = temp_val;
                        new_move = (j as i64, *letter);
                        move_queue = Vec::new();
                        move_queue.push((j as i64, *letter));
                    } else if temp_val == v {
                        move_queue.push((j as i64, *letter));
                    }

                    // alpha-beta pruning
                    if v < alpha {
                        new_move = self.choose(&move_queue);
                        return (v, new_move);
                    }
                    beta = std::cmp::min(beta, v);
                }
            }
        }
        new_move = self.choose(&move_queue);

        return (v, new_move);
    }
    
    pub fn get_random_val(&self, val: usize) -> usize {
        let mut rng = rand::thread_rng();
        let base: f64 = rng.gen();
        let max_val = val as f64;

        return (base * max_val).floor() as usize;
    }

    pub fn choose<T: Copy>(&self, choice: &Vec<T>) -> T {
        let index = self.get_random_val(choice.len());
        return choice[index];
    }

    pub fn fill_map(&self, new_state: &Vec<Vec<i64>>, column: usize, value: i64) -> Vec<Vec<i64>> {
        let mut temp_map = new_state.clone();
        if temp_map[0][column] != 0 || column > 6 {
            temp_map[0][0] = 999; // error code
        }

        let mut done = false;
        let mut row = 0;

        for i in 0..5 {
            if temp_map[i + 1][column] != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = 5;
        }

        temp_map[row][column] = value;
        return temp_map;
    }
}
