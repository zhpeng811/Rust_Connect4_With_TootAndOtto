#![allow(non_snake_case)]
pub use crate::disc::DiscType;
pub use crate::player::{Player, PlayerType};
pub use crate::board::Board;

pub enum GameEvent {
    PlaceSuccess,
    Player1Win,
    Player2Win,
    Draw,
    IsTOOT,
    IsOTTO,
    Neither,
    PlaceColumnFull,
    UnexpectedErr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameType {
    Connect4,
    TOOTandOTTO
}

impl ToString for GameType {
    fn to_string(&self) -> String {
        match self {
            GameType::Connect4 => String::from("Connect-4"),
            GameType::TOOTandOTTO => String::from("TOOT-OTTO")
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoardGame {
    pub game_board: Board,
    pub player1: Player,
    pub player2: Player,
    pub current_player: usize,
    pub game_type: GameType
}

impl BoardGame {
    pub fn new_connect4(board_rows: usize, board_columns: usize, vs_ai: bool) -> Self {
        let player2_type = if vs_ai {PlayerType::AI} else {PlayerType::Human};

        Self {
            game_board: Board::new(board_rows, board_columns),
            player1: Player::new(PlayerType::Human, DiscType::Red),
            player2: Player::new(player2_type, DiscType::Yellow),
            current_player: 1,
            game_type: GameType::Connect4
        }
    }

    pub fn new_toot_and_otto(board_rows: usize, board_columns: usize, vs_ai: bool) -> Self {
        let player2_type = if vs_ai {PlayerType::AI} else {PlayerType::Human};

        Self {
            game_board: Board::new(board_rows, board_columns),
            player1: Player::new(PlayerType::Human, DiscType::T), // disc is just a default, can be changed
            player2: Player::new(player2_type, DiscType::O), // disc is just a default, can be changed
            current_player: 1,
            game_type: GameType::TOOTandOTTO
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

    // for TOOT and OTTO
    pub fn change_disc_type(&mut self, disc_type: DiscType) {
        let player: &mut Player;

        if (self.game_type == GameType::Connect4) {
            // not allowed to chagne disc type for connect 4
            // there should not be a GUI that allows this
            return
        }

        // due to GUI issues, changing a disc type will change for both players
        self.player1.disc_type = disc_type;
        self.player2.disc_type = disc_type;
    }

    pub fn place_disc(&mut self, column: usize) -> GameEvent {
        self.game_board.place_disc(column, self.get_current_disc_type())
    }

    pub fn check(&mut self) -> GameEvent {
        if self.game_type == GameType::Connect4 && self.game_board.is_connect4(self.get_current_disc_type()) {
            if self.current_player == 1 {
                return GameEvent::Player1Win
            } else {
                return GameEvent::Player2Win
            }
        } else if self.game_type == GameType::TOOTandOTTO {
            let event = self.game_board.is_toot_or_otto();
            match event {
                GameEvent::IsTOOT => return GameEvent::Player1Win,
                GameEvent::IsOTTO => return GameEvent::Player2Win,
                _ => {}
            }
        }

        if self.game_board.is_full() {
            return GameEvent::Draw
        } else {
            self.switch_turn();
            return GameEvent::PlaceSuccess
        }
    }
}
