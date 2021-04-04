#![allow(non_snake_case)]
pub use crate::disc::DiscType;
pub use crate::player::{Player, PlayerType};
pub use crate::board::Board;
pub use crate::game::GameEvent;

pub struct TOOTandOTTO {
    pub game_board: Board,
    pub toot_player: Player,
    pub otto_player: Player,
    pub current_player: usize
}

impl TOOTandOTTO {
    pub fn new(board_rows: usize, board_columns: usize, vs_ai: bool) -> Self {
        let otto_type = if vs_ai {PlayerType::AI} else {PlayerType::Human};

        Self {
            game_board: Board::new(board_rows, board_columns),
            toot_player: Player::new(PlayerType::Human, DiscType::T), // disc type is just default and will be changable
            otto_player: Player::new(player2_type, DiscType::O),
            current_player: 1
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

    pub fn place_disc(&mut self, column: usize) -> GameEvent {
        let game_event = self.game_board.place_disc(column, self.get_current_disc_type());
        match game_event {
            GameEvent::PlaceSuccess(row) => {
                self.check(row)
            },
            _ => return game_event
        }
    }

    pub fn check(&mut self, row: usize) -> GameEvent {
        if self.game_board.is_full() {
            return GameEvent::Draw(row)
        } else if self.game_board.is_connect4(self.get_current_disc_type()) {
            if self.current_player == 1 {
                return GameEvent::Player1Win(row)
            } else {
                return GameEvent::Player2Win(row)
            }
        } else {
            self.switch_turn();
            return GameEvent::PlaceSuccess(row)
        }
    }
}