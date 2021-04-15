#![allow(non_snake_case)]
pub use crate::disc::DiscType;
pub use crate::player::{Player, PlayerType};
pub use crate::board::Board;

#[derive(Clone)]
pub enum GameEvent {
    PlaceSuccess(usize),
    Player1Win(usize),
    Player2Win(usize),
    Draw(usize),
    IsTOOT,
    IsOTTO,
    Neither,
    PlaceColumnFull,
    UnexpectedErr,
    Ongoing
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameType {
    Connect4,
    TOOTandOTTO
}

#[derive(Clone)]
pub struct BoardGame {
    pub game_board: Board,
    pub player1: Player,
    pub player2: Player,
    pub current_player: usize,
    pub game_type: GameType,
    pub turns: usize,
    pub status: GameEvent
}

impl BoardGame {
    pub fn new_connect4(board_rows: usize, board_columns: usize, vs_ai: bool) -> Self {
        let player2_type = if vs_ai {PlayerType::AI} else {PlayerType::Human};

        Self {
            game_board: Board::new(board_rows, board_columns),
            player1: Player::new(PlayerType::Human, DiscType::Red),
            player2: Player::new(player2_type, DiscType::Yellow),
            current_player: 1,
            game_type: GameType::Connect4,
            turns: 0,
            status: GameEvent::Ongoing
        }
    }

    pub fn new_toot_and_otto(board_rows: usize, board_columns: usize, vs_ai: bool) -> Self {
        let player2_type = if vs_ai {PlayerType::AI} else {PlayerType::Human};

        Self {
            game_board: Board::new(board_rows, board_columns),
            player1: Player::new(PlayerType::Human, DiscType::T), // disc is just a default, can be changed
            player2: Player::new(player2_type, DiscType::O), // disc is just a default, can be changed
            current_player: 1,
            game_type: GameType::TOOTandOTTO,
            turns: 0,
            status: GameEvent::Ongoing
        }
    }

    
    // pub fn get_board_rows(&self) -> usize {
    //     self.game_board.get_num_rows()    
    // }

    // pub fn get_board_columns(&self) -> usize{
    //     self.game_board.get_num_columns()
    // }

    pub fn get_turns(&self) -> usize {
        self.turns
    }

    pub fn get_current_disc_type(&self) -> DiscType {
        let current_player = 1 + self.turns % 2;
        if current_player == 1 {
            self.player1.disc_type
        } else {
            self.player2.disc_type
        }
    }

    pub fn get_other_disc_type(&self) -> DiscType {
        let current_player = 1 + self.turns % 2;
        if current_player == 1 {
            self.player2.disc_type
        } else {
            self.player1.disc_type
        }
    }


    // for TOOT and OTTO
    pub fn switch_disc_type(&mut self) {
        let player: &mut Player;

        let current_player = 1 + self.turns % 2;
        if current_player == 1 {
            player = &mut self.player1;
        } else {
            player = &mut self.player2;
        }

        if player.disc_type == DiscType::T {
            (*player).disc_type = DiscType::O;
        } else {
            (*player).disc_type = DiscType::T;
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

    fn check(&mut self, row: usize) -> GameEvent {
        if self.game_type == GameType::Connect4 && self.game_board.is_connect4(self.get_current_disc_type()) {
            let current_player = 1 + self.turns % 2;
            if current_player == 1 {
                return GameEvent::Player1Win(row)
            } else {
                return GameEvent::Player2Win(row)
            }
        } else if self.game_type == GameType::TOOTandOTTO {
            let event = self.game_board.is_toot_or_otto();
            match event {
                GameEvent::IsTOOT => return GameEvent::Player1Win(row),
                GameEvent::IsOTTO => return GameEvent::Player2Win(row),
                _ => ()
            }
        }

        if self.game_board.is_full() {
            return GameEvent::Draw(row)
        } else {
            self.turns += 1;
            return GameEvent::PlaceSuccess(row)
        }
    }
}