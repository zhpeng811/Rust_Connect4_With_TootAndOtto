// OLD, DO NOT USE

// #![allow(non_snake_case)]
// pub use crate::disc::DiscType;
// pub use crate::player::{Player, PlayerType};
// pub use crate::board::Board;
// pub use crate::game::{GameEvent, Game};

// pub struct Connect4 {
//     pub game_board: Board,
//     pub player1: Player,
//     pub player2: Player,
//     pub current_player: usize
// }

// impl Connect4 {
//     fn new(board_rows: usize, board_columns: usize, vs_ai: bool) -> Self {
//         let player2_type = if vs_ai {PlayerType::AI} else {PlayerType::Human};

//         Self {
//             game_board: Board::new(board_rows, board_columns),
//             player1: Player::new(PlayerType::Human, DiscType::Red),
//             player2: Player::new(player2_type, DiscType::Yellow),
//             current_player: 1
//         }
//     }

//     fn switch_turn(&mut self) {
//         if self.current_player == 1 {
//             self.current_player = 2
//         } else {
//             self.current_player = 1
//         }
//     }

//     fn get_current_disc_type(&self) -> DiscType {
//         if self.current_player == 1 {
//             self.player1.disc_type
//         } else {
//             self.player2.disc_type
//         }
//     }

//     pub fn place_disc(&mut self, column: usize) -> GameEvent {
//         let game_event = self.game_board.place_disc(column, self.get_current_disc_type());
//         match game_event {
//             GameEvent::PlaceSuccess(row) => {
//                 self.check(row)
//             },
//             _ => return game_event
//         }
//     }

//     fn check(&mut self, row: usize) -> GameEvent {
//         if self.game_board.is_full() {
//             return GameEvent::Draw(row)
//         } else if self.game_board.is_connect4(self.get_current_disc_type()) {
//             if self.current_player == 1 {
//                 return GameEvent::Player1Win(row)
//             } else {
//                 return GameEvent::Player2Win(row)
//             }
//         } else {
//             self.switch_turn();
//             return GameEvent::PlaceSuccess(row)
//         }
//     }
// }