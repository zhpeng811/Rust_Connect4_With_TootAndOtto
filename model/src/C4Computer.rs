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

const MINMAX_SHIFT: isize = 14;

pub fn evaluate_board(game: &mut BoardGame, ai: AIConfig) -> (isize, isize) {
    let is_max = game.get_turns() % 2 == 0;
    fn test_move(col: usize, game: &mut BoardGame, ai: AIConfig) -> isize {
        game.place_disc(mov);
        let mut score = minmax_search(game, ai.minmax_depth) << MINMAX_SHIFT;
        if score == 0 {
            score = mc_search(game, ai);
        }
        game.game_board.undo_last();
        score
    }
}

fn mc_search(game: &mut BoardGame, ai: AIConfig) -> isize {
    let mut score = 0;
    (0..ai.carlo_iter).for_each(|_| {
        let mut moves = 0;
        let mut result = GameEvent::Ongoing;
        let mut finished = false;
        while !finished {
            if result == GameEvent::Ongoing {
                let cols = game.get_col();
                let col = random::<usize>() % cols;
                let result = game.place_disc(col);
            } else if result == GameEvent::Player1Win(row) || result == GameEvent::Player2Win(row){
                score += 1;
                finished = true;
            } else if result == GameEvent::Draw(row) {
                finished = true;
            } else if result == GameEvent::PlaceColumnFull{
                moves -= 1;
                result = GameEvent::Ongoing;
            }
        }
        for _ in 0..moves {
            game.game_board.undo_last();
        }
    };
    score
}

static mut COUNT: isize = 0;
fn minmax(game: &mut BoardGame, depth: usize) -> isize {
    unsafe {
        COUNT += 1;
    }
    if depth == 0 {
        return 0;
    }
    let is_max = game.get_turns() % 2 == 0;

    if game.status == GameEvent::Player2Win {
        return -(depth as isize);
    }
    if game.status == GameEvent::Player1Win {
        return depth as isize;
    }


    let minmax: fn(isize, isize) -> isize = if is_max { std::cmp::max } else { std::cmp::min };

    let mut score = if is_max {
        std::isize::MIN
    } else {
        std::isize::MAX
    };

    let cols = game.get_col();
    for i in 0..cols {
        let result = game.place_disc(i);
        if result != GameEvent::PlaceColumnFull {
            score = minmax(score, minmax_search(game, depth - 1));
            game.game_board.undo_last();
        }
    }
    score
}


