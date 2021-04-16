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

const ERR_CODE: i64 = 421;
const REWARD: i64 = 999999;

fn fill_map(
        new_state: &Vec<Vec<i64>>, 
        column: usize, 
        value: i64, 
        board_rows: usize,
        board_columns: usize
    ) -> Vec<Vec<i64>> {

    let mut temp_map = new_state.clone();
    if temp_map[0][column] != 0 || column >= board_columns {
        temp_map[0][0] = ERR_CODE; 
    }

    let mut done = false;
    let mut row = 0;

    for i in 0..board_rows - 1 {
        if temp_map[i + 1][column] != 0 {
            done = true;
            row = i;
            break;
        }
    }
    if !done {
        row = board_rows - 1;
    }
    temp_map[row][column] = value;
    return temp_map;
}

fn get_random_index(len: usize) -> usize {
    let mut rng = rand::thread_rng();
    let random: f64 = rng.gen();

    return (random * len as f64).floor() as usize;
}

fn choose<T: Copy>(choice: &Vec<T>) -> T {
    let index = get_random_index(choice.len());
    return choice[index];
}

pub struct Connect4AI {
    board_rows: usize,
    board_columns: usize,
    difficulty: Difficulty,
    score_board: Vec<Vec<i64>>
}

impl Connect4AI {
    pub fn new(board_rows: usize, board_columns: usize, difficulty: Difficulty) -> Self {
        let mut map: Vec<Vec<i64>> = vec![vec![0; board_columns]; board_rows];
        Self {
            board_rows,
            board_columns,
            difficulty,
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
            }
            _ => { // Hard or Insane, use minmax algorithm with alpha-beta pruning
                let choice_val = self.max_state(-1, &self.score_board, 0, -100000000007, 100000000007);
                let (val, choice) = choice_val;
                if choice < 0 || choice as usize > self.board_columns {
                    return self.random_gen(game.game_board.clone());
                }
                return choice as usize;
            }
        }
    }

    fn find_winning_move(&self, game_board: Board) -> isize {
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
            Difficulty::Hard => 3,
            Difficulty::Insane => 4,
            _ => 1,
        };
        if depth >= max_depth { // if slow (or memory consumption is high), lower the value
            let mut ret_value = 0;

            // if win, value = +inf
            let win_val = val.0;
            let chain_val = val.1 * ai_move_value;
            ret_value = chain_val;

            // If it lead to winning, then do it
            if win_val == 4 * ai_move_value { // AI win, AI wants to win of course
                ret_value = REWARD;
            } else if win_val == 4 * ai_move_value * -1 { // AI lose, AI hates losing
                ret_value = REWARD * -1;
            }
            ret_value -= depth * depth;

            return (ret_value, -1);
        }

        let win = val.0;
        // if already won, then return the value right away
        if win == 4 * ai_move_value { // AI win, AI wants to win of course
            return (REWARD - depth * depth, -1);
        }
        if win == 4 * ai_move_value * -1 { // AI lose, AI hates losing
            return (REWARD * -1 - depth * depth, -1);
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
        let mut move_val: i64 = -1;
        let mut move_queue = Vec::new();

        for j in 0..self.board_columns {
            let temp_state = fill_map(state, j, ai_move_value, self.board_rows, self.board_columns);
            if temp_state[0][0] != ERR_CODE {
                let temp_val = self.value(ai_move_value, &temp_state, depth, alpha, beta);
                if temp_val.0 > v {
                    v = temp_val.0;
                    move_val = j as i64;
                    move_queue = Vec::new();
                    move_queue.push(j as i64);
                } else if temp_val.0 == v {
                    move_queue.push(j as i64);
                }

                // alpha-beta pruning
                if v > beta {
                    move_val = choose(&move_queue);
                    return (v, move_val);
                }
                alpha = std::cmp::max(alpha, v);
            }
        }

        move_val = choose(&move_queue);
        return (v, move_val);
    }

    pub fn min_state(&self, ai_move_value: i64, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64) -> (i64, i64) {
        let mut v = 100000000007;
        let mut move_val: i64 = -1;
        let mut move_queue = Vec::new();

        for j in 0..self.board_columns {
            let temp_state = fill_map(state, j, ai_move_value * -1, self.board_rows, self.board_columns);
            if temp_state[0][0] != ERR_CODE {
                let temp_val = self.value(ai_move_value, &temp_state, depth, alpha, beta);
                if temp_val.0 < v {
                    v = temp_val.0;
                    move_val = j as i64;
                    move_queue = Vec::new();
                    move_queue.push(j as i64);
                } else if temp_val.0 == v {
                    move_queue.push(j as i64);
                }

                // alpha-beta pruning
                if v < alpha {
                    move_val = choose(&move_queue);
                    return (v, move_val);
                }
                beta = std::cmp::min(beta, v);
            }
        }
        move_val = choose(&move_queue);

        return (v, move_val);
    }

    pub fn check_state(&self, state: &Vec<Vec<i64>>) -> (i64, i64) {
        let mut win_val = 0;
        let mut chain_val = 0;
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (0, 0, 0, 0);
        for i in 0..self.board_rows {
            for j in 0..self.board_columns {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {
                    if j + k < self.board_columns {
                        temp_r += state[i][j + k];
                    }

                    if i + k < self.board_rows {
                        temp_b += state[i + k][j];
                    }

                    if i + k < self.board_rows && j + k < self.board_columns {
                        temp_br += state[i + k][j + k];
                    }

                    if i >= k && j + k < self.board_columns  {
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
            _ => { // Hard or Insane
                let choice_val = self.max_state(&self.score_board, 0, -100000000007, 100000000007);
                let (val, (column, letter)) = choice_val;
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
        for i in 0..self.board_rows {
            for j in 0..self.board_columns {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {
                    let sign: i64 = if k == 0 || k == 3 { -1 } else { 1 };
                    if j + k < self.board_columns {
                        temp_r += sign * state[i][j + k];
                    }
                    if i + k < self.board_rows {
                        temp_b += sign * state[i + k][j];
                    }
                    if i + k < self.board_rows && j + k < self.board_columns {
                        temp_br += sign * state[i + k][j + k];
                    }

                    if i >= k && j + k < self.board_columns {
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

    pub fn value(&self, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64) -> i64 {
        let val = self.check_state(state);
        // depth is a big lower as the AI is slow
        let max_depth = match self.difficulty {
            Difficulty::Hard => 2,
            Difficulty::Insane => 3,
            _ => 1,
        };
        if depth >= max_depth { // if slow (or memory consumption is high), lower the value
            let mut ret_val = 0;

            // if win, value = +inf
            let (win_val, chain_val) = val; 
            ret_val = chain_val;

            // If it lead to winning, then do it
            if win_val == 4 { // AI win, AI wants to win of course
                ret_val = REWARD;
            } else if win_val == 4 { // AI lose, AI hates losing
                ret_val = REWARD * -1;
            }
            ret_val -= depth * depth;

            return ret_val;
        }

        let win = val.0;
        // if already won, then return the value right away
        if win == 4 { // AI win, AI wants to win of course
            return (REWARD - depth * depth);
        }
        if win == -4 {
            // AI lose, AI hates losing
            return (-1 * REWARD - depth * depth);
        }

        if depth % 2 == 0 {
            return self.min_state(state, depth + 1, alpha, beta).0;
        } else {
            return self.max_state(state, depth + 1, alpha, beta).0;
        }
    }

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
            for j in 0..self.board_columns {
                let move_value = if *letter == 'T' { 1 } else { -1 };
                let temp_state = fill_map(state, j, move_value, self.board_rows, self.board_columns);
                if temp_state[0][0] != ERR_CODE {
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
                        new_move = choose(&move_queue);
                        return (v, new_move);
                    }
                    alpha = std::cmp::max(alpha, v);
                }
            }
        }
        new_move = choose(&move_queue);

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
            for j in 0..self.board_columns {
                let move_value = if *letter == 'T' { 1 } else { -1 };
                let temp_state = fill_map(state, j, move_value, self.board_rows, self.board_columns);
                if temp_state[0][0] != ERR_CODE {
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
                        new_move = choose(&move_queue);
                        return (v, new_move);
                    }
                    beta = std::cmp::min(beta, v);
                }
            }
        }
        new_move = choose(&move_queue);

        return (v, new_move);
    }

}
