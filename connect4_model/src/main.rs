mod connect4;
use connect4::*;

fn main() {
    let mut game = Connect4::new(6, 7, false);
    loop {
        println!("{}", game.game_board);
        if game.current_player == 1 {
            println!("Player 1's turn, Disc color = Red");
        } else {
            println!("Player 2's turn, Disc color = Yellow");
        }
        println!("please type the column that you wish to insert your piece: ");
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("unable to read number");
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
