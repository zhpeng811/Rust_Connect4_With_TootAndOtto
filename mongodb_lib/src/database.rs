use mongodb::{Client, Database, error::Error, results::InsertOneResult};
use bson::{doc, Bson};
use chrono::Utc;

#[derive(Debug)]
pub struct HistoryInfo {
    game_type: String,
    player1: String,
    player2: String,
    winner: String,
    time_played: String,
}

fn connect_to_db() -> Database {
    let result = Client::with_uri_str("mongodb://localhost:27017/");
    match result {
        Ok(client) => {
            client.database("Connect4DB")
        },
        Err(err) => panic!("Cannot connect to mongodb: {}", err)
    }
}

fn add_game_history(game_type: String, player1: String, player2: String, winner: String) -> Result<InsertOneResult, Error> {
    let db = connect_to_db();
    let collection = db.collection("games");
    let doc = doc! {
        "gameType": game_type,
        "Player1Name": player1,
        "Player2Name": player2,
        "WinnerName": winner,
        "GameDate": Utc::now()
    };

    collection.insert_one(doc, None)
}

fn get_histories() -> Result<Vec<HistoryInfo>, Error> {
    let db = connect_to_db();
    let collection = db.collection("games");
    let cursor = collection.find(None, None)?;
    
    let mut history_info: Vec<HistoryInfo> = Vec::new();
    for result in cursor {
        match result {
            Ok(document) => {
                let game_type = String::from(document.get("gameType").and_then(Bson::as_str).unwrap_or(""));
                let player1 = String::from(document.get("Player1Name").and_then(Bson::as_str).unwrap_or(""));
                let player2 = String::from(document.get("Player2Name").and_then(Bson::as_str).unwrap_or(""));
                let winner = String::from(document.get("WinnerName").and_then(Bson::as_str).unwrap_or(""));
                let mut time_played: String = "unknown".to_string();
                if let Some(date_time) = document.get("GameDate").and_then(Bson::as_utc_date_time) {
                    let time_rfc2822 = date_time.to_rfc2822();
                    let len = time_rfc2822.len();
                    // remove the "+0000" part from the String
                    // see https://docs.rs/chrono/0.4.5/chrono/struct.DateTime.html#method.to_rfc2822 for more detail
                    time_played = time_rfc2822[..len - 6].to_string();
                }
                history_info.push(HistoryInfo {
                    game_type,
                    player1,
                    player2,
                    winner,
                    time_played
                });
            },
            Err(_) => {}
        }
    }

    Ok(history_info)
}