mod disc;
mod board;
mod player;
mod game;
mod C4Computer;
use game::*;
use C4Computer::*;

fn play_connect4_with_computer(row: usize, column: usize) {
    let mut game = BoardGame::new_connect4(row, column, false);
    let mut ai = C4Computer::Connect4AI::new(row, column, C4Computer::Difficulty::Medium);
    let mut column_to_place = 0;
    loop {
        println!("{}", game.game_board);
        if 1 + game.get_turns() % 2 == 1{
            println!("Player 1's turn, current disc: {}", game.player1.disc_type);
            println!("please type the column that you wish to insert your piece: ");
            let mut line = String::new();
            std::io::stdin()
                .read_line(&mut line)
                .expect("unable to read input");
            match line.trim().parse::<usize>() {
                Ok(col) => {
                    if col >= column {
                        println!("index out of bound, try again");
                        continue;
                    }
                    column_to_place = col;
                }
                Err(_) => println!("invalid input, try again")
            }
        } else {
            println!("Computer's turn, computer disc: {}", game.player2.disc_type);
            let mut clone_game = game.clone();
            // let (column, score) = ai.negamax(&mut clone_game, 0, true);
            let column = ai.findMove(&mut clone_game);
            println!("here");
            println!("Computer choose to place at column: {}", column);
            column_to_place = column as usize;
        }

        match game.place_disc(column_to_place) {
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
}

fn play_connect4_with_human() {
    let mut game = BoardGame::new_connect4(6, 7, false);
    loop {
        println!("{}", game.game_board);
        if game.current_player == 1 {
            println!("Player 1's turn, current disc: {}", game.player1.disc_type);
        } else {
            println!("Player 2's turn, current disc: {}", game.player2.disc_type);
        }
        println!("please type the column that you wish to insert your piece: ");
        // if game_type.eq("2\n") {
        //     println!("type 'C' to switch your disc"); 
        // }
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("unable to read input");
        // if game_type.eq("2\n") && line.eq("C\n") {
        //     game.switch_disc_type();
        //     continue;
        // }
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

fn main() {
    let mut game: BoardGame;
    loop {
        println!("pick a game: ");
        println!("1: Connect 4");
        println!("2: TOOT and OTTO");
        println!("3: quit");
        let mut game_type = String::new();
        std::io::stdin()
            .read_line(&mut game_type)
            .expect("unable to read input");
        if game_type.eq("1\n") {
            play_connect4_with_computer(6, 7);
        } else if game_type.eq("2\n") {
            play_connect4_with_human();
            // game = BoardGame::new_toot_and_otto(6, 7, false);
        } else if game_type.eq("3\n") {
            break;
        } else {
            println!("invalid input");
            continue;
        }
    }
}
