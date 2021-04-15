mod disc;
mod board;
mod player;
mod game;
mod ai;
use game::*;
use ai::*;

fn read_input() -> usize {
    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("unable to read input");

        match line.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("invalid input, try again")
        }
    }
}

fn play_connect4_with_computer(row: usize, column: usize) {
    let mut game = BoardGame::new_connect4(row, column, true);
    let mut ai = ai::Connect4AI::new(row, column, ai::Difficulty::Medium);
    let mut column_to_place: usize = 0;
    loop {
        println!("{}", game.game_board);
        if game.current_player == 1 {
            println!("Player 1's turn, current disc: {}", game.player1.disc_type);
            println!("please type the column that you wish to insert your piece: ");
            
            column_to_place = read_input();
            if column_to_place >= column {
                println!("index out of bound, try again");
                continue;
            }
        } else {
            println!("Computer's turn, computer disc: {}", game.player2.disc_type);
            let mut clone_game = game.clone();
            column_to_place = ai.findMove(clone_game);
            println!("Computer choose to place at column: {}", column_to_place);
        }

        match game.place_disc(column_to_place) {
            GameEvent::PlaceSuccess => {
                match game.check() {
                    GameEvent::Draw => {
                        println!("Game Draw!");
                        break;
                    }
                    GameEvent::Player1Win => {
                        println!("Player 1 Win!");
                        break;
                    },
                    GameEvent::Player2Win => {
                        println!("Player 2 Win!");
                        break;
                    },
                    GameEvent::PlaceSuccess => (),
                    _ => println!("unexpected")
                }
            },
            GameEvent::PlaceColumnFull => println!("column is full, try again"),
            _ => println!("unexpected error, try again"),
        }
    }
}

fn play_connect4_with_human(row: usize, column: usize) {
    let mut game = BoardGame::new_connect4(6, 7, false);
    loop {
        println!("{}", game.game_board);
        if game.current_player == 1 {
            println!("Player 1's turn, current disc: {}", game.player1.disc_type);
        } else {
            println!("Player 2's turn, current disc: {}", game.player2.disc_type);
        }
        println!("please type the column that you wish to insert your piece: ");

        let column_to_place = read_input();
        match game.place_disc(column_to_place) {
            GameEvent::PlaceSuccess => {
                match game.check() {
                    GameEvent::Draw => {
                        println!("Game Draw!");
                        break;
                    }
                    GameEvent::Player1Win => {
                        println!("Player 1 Win!");
                        break;
                    },
                    GameEvent::Player2Win => {
                        println!("Player 2 Win!");
                        break;
                    },
                    GameEvent::PlaceSuccess => (),
                    _ => println!("unexpected")
                }
            },
            GameEvent::PlaceColumnFull => println!("column is full, try again"),
            _ => println!("unexpected error, try again"),
        }
    }
}

fn play_toototto_with_human(row: usize, column: usize) {
    let mut game = BoardGame::new_toot_and_otto(6, 7, false);
    loop {
        println!("{}", game.game_board);
        if game.current_player == 1 {
            println!("Player 1's turn, current disc: {}", game.player1.disc_type);
        } else {
            println!("Player 2's turn, current disc: {}", game.player2.disc_type);
        }
        println!("please type the column that you wish to insert your piece: ");
        println!("type '421' to switch your disc to 'T'"); 
        println!("type '466' to switch your disc to 'O'"); 
    
        let column_to_place = read_input();
        if column_to_place == 421 {
            game.change_disc_type(DiscType::T);
            continue;
        } else if column_to_place == 466 {
            game.change_disc_type(DiscType::O);
            continue;
        }

        match game.place_disc(column_to_place) {
            GameEvent::PlaceSuccess => {
                match game.check() {
                    GameEvent::Draw => {
                        println!("Game Draw!");
                        break;
                    }
                    GameEvent::Player1Win => {
                        println!("Player 1 Win!");
                        break;
                    },
                    GameEvent::Player2Win => {
                        println!("Player 2 Win!");
                        break;
                    },
                    GameEvent::PlaceSuccess => (),
                    _ => println!("unexpected")
                }
            },
            GameEvent::PlaceColumnFull => println!("column is full, try again"),
            _ => println!("unexpected error, try again"),
        }
    }
}

fn main() {
    loop {
        println!("pick a game: ");
        println!("1: Connect 4 With Computer");
        println!("2: Connect 4 With Human");
        println!("3: TOOT and OTTO with Computer");
        println!("4: TOOT and OTTO with Human");
        println!("5: quit");
        let input = read_input();
        match input {
            1 => play_connect4_with_computer(6, 7),
            2 => play_connect4_with_human(6, 7),
            3 => continue,
            4 => play_toototto_with_human(6, 7),
            5 => break,
            _ => {
                println!("invalid input");
                continue;
            }
        }
    }
}