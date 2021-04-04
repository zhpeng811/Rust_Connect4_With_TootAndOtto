mod disc;
mod board;
mod player;
mod game;
// mod connect4;
use game::*;

fn main() {
    let mut game = BoardGame::new_toot_and_otto(6, 7, false);
    loop {
        println!("{}", game.game_board);
        if game.current_player == 1 {
            println!("Player 1's turn, current disc: {}", game.player1.disc_type);
        } else {
            println!("Player 2's turn, current disc: {}", game.player2.disc_type);
        }
        println!("please type the column that you wish to insert your piece: ");
        println!("type 'C' to switch your disc");
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("unable to read input");
        if (line.eq("C\n")) {
            game.switch_disc_type();
            continue;
        }
        match line.trim().parse::<usize>() {
            Ok(column) => {
                match game.place_disc(column) {
                    GameEvent::PlaceSuccess(row) => {},
                    GameEvent::Draw(row) => {
                        println!("Game Draw!");
                        break;
                    }
                    GameEvent::Player1Win(row) => {
                        println!("Player 1 Win!");
                        break;
                    },
                    GameEvent::Player2Win(row) => {
                        println!("Player 2 Win!");
                        break;
                    },
                    _ => println!("column is full, try again")
                }
            }
            Err(_) => println!("invalid input, try again")
        }
    }
}
