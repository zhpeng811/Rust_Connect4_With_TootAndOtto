use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryInfo {
    pub game_type: String,
    pub player1: String,
    pub player2: String,
    pub winner: String,
    pub difficulty: String,
    pub time_played: String,
}
