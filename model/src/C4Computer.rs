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
