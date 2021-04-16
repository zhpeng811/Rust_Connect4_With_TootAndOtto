mod disc;
mod board;
mod player;
mod game;
mod ai;
use game::*;

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
    let mut ai = ai::Connect4AI::new(row, column, ai::Difficulty::Insane);
    loop {
        println!("{}", game.game_board);
        let column_to_place: usize;
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
            let clone_game = game.clone();
            column_to_place = ai.find_best_move(clone_game);
            println!("Computer choose to place at column: {}", column_to_place);
        }

        match game.place_disc(column_to_place) {
            GameEvent::PlaceSuccess(_) => {
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
                    GameEvent::Ongoing => (),
                    _ => println!("unexpected")
                }
            },
            GameEvent::PlaceColumnFull => println!("column is full, try again"),
            _ => println!("unexpected error, try again"),
        }
    }
}

fn play_connect4_with_human(row: usize, column: usize) {
    let mut game = BoardGame::new_connect4(row, column, false);
    loop {
        println!("{}", game.game_board);
        if game.current_player == 1 {
            println!("Player 1's turn, current disc: {}", game.player1.disc_type);
        } else {
            println!("Player 2's turn, current disc: {}", game.player2.disc_type);
        }
        println!("please type the column that you wish to insert your piece: ");

        let column_to_place = read_input();
        if column_to_place >= column {
            println!("index out of bound, try again");
            continue;
        }
        match game.place_disc(column_to_place) {
            GameEvent::PlaceSuccess(_) => {
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
                    GameEvent::Ongoing => (),
                    _ => println!("unexpected")
                }
            },
            GameEvent::PlaceColumnFull => println!("column is full, try again"),
            _ => println!("unexpected error, try again"),
        }
    }
}

fn play_toototto_with_computer(row: usize, column: usize) {
    let mut game = BoardGame::new_toot_and_otto(row, column, false);
    let ai = ai::TootOttoAI::new(row, column, ai::Difficulty::Hard);

    loop {
        println!("{}", game.game_board);
        let column_to_place: usize;
        let current_disc_type = game.get_current_disc_type();

        if game.current_player == 1 {
            println!("Player 1's turn, current disc: {}", game.player1.disc_type);
            println!("please type the column that you wish to insert your piece: ");
            println!("type '421' to switch your disc to 'T'"); 
            println!("type '466' to switch your disc to 'O'"); 
        
            column_to_place = read_input();
            if column_to_place == 421 {
                game.change_disc_type(DiscType::T);
                continue;
            } else if column_to_place == 466 {
                game.change_disc_type(DiscType::O);
                continue;
            } else if column_to_place >= column {
                println!("index out of bound, try again");
                continue;
            }
        } else {
            println!("Computer's turn, computer disc: {}", game.player2.disc_type);
            let clone_game = game.clone();
            let (best_move, disc_type) = ai.find_best_move(clone_game);
            column_to_place = best_move;
            println!("Computer choose to place at column: {}", column_to_place);

            // change the disc type
            game.change_disc_type(disc_type);
        }

        match game.place_disc(column_to_place) {
            // reset the disc type that might be changed by computer
            GameEvent::PlaceSuccess(_) => {
                game.change_disc_type(current_disc_type);
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
                    GameEvent::Ongoing => (),
                    _ => println!("unexpected")
                }
            },
            GameEvent::PlaceColumnFull => println!("column is full, try again"),
            _ => println!("unexpected error, try again"),
        }
    }
}

fn play_toototto_with_human(row: usize, column: usize) {
    let mut game = BoardGame::new_toot_and_otto(row, column, false);
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
        } else if column_to_place >= column {
            println!("index out of bound, try again");
            continue;
        }

        match game.place_disc(column_to_place) {
            GameEvent::PlaceSuccess(_) => {
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
                    GameEvent::Ongoing => (),
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
            3 => play_toototto_with_computer(6, 7),
            4 => play_toototto_with_human(6, 7),
            5 => break,
            _ => {
                println!("invalid input");
                continue;
            }
        }
    }
}
