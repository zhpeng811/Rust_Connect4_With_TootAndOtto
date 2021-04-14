use crate game::{GameEvent, BoardGame};
use rand::prelude::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Easy => String::from("Easy"),
            Medium => String::from("Medium"),
            Hard => String::from("Hard"),
        }
    }
}

pub struct AIConfig {
    carlo_iter: isize,
    minmax_depth: isize,
}

pub const EASY_AI: AIConfig = AIConfig {
    carlo_iter: 5,
    minmax_depth: 2,
};

pub const MID_AI: AIConfig = AIConfig {
    carlo_iter: 1000,
    minmax_depth: 4,
};

pub const HARD_AI: AIConfig = AIConfig {
    carlo_iter: 4000,
    minmax_depth: 6,
};


fn mc_search(game: &mut BoardGame, ai: AIConfig) -> isize {
    let mut score = 0;
    (0..ai.carlo_iter).for_each(|_| {
        let mut finished = false;
        while !finished {
            let cols = game.get_col();
            let col = random::<usize>() % cols;
            let result = game.place_disc(col);
            if result == GameEvent::Player1Win(row) || result == GameEvent::Player2Win(row){
                score += 1;
                finished = true;
            } else if result == GameEvent::Draw(row) {
                finished = true;
            }
        }
        game.board.clear_board();
    };
    score
}

static mut COUNT: isize = 0;
fn minmax(game: &mut BoardGame, depth: usize) {
    unsafe {
        COUNT += 1;
    }
    if depth == 0 {
        return 0;
    }
    let is_max = game.get_turn() % 2 == 0;

    let minmax: fn(isize, isize) -> isize = if is_max { std::cmp::max } else { std::cmp::min };

    let mut score = if is_max {
        std::isize::MIN
    } else {
        std::isize::MAX
    };

    let cols = game.get_col();
    for i in 0..cols {
        game.place_disc(i);
        score = minmax(score, minmax_search(game, depth - 1));
        game.board.clear_board();
    }
    score
}


