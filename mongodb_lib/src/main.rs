#![feature(proc_macro_hygiene, decl_macro)] // needed for rocket crate to use get() and post() macro

#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use mongodb::{Client, Collection, error::Error};
use rocket::config::{Config, Environment};

mod database {
    use mongodb::{Client, Collection, error::Error, results::InsertOneResult};
    use bson::{doc, Bson};
    use chrono::Utc;
    use rocket::State;
    use rocket_contrib::json::Json;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HistoryInfo {
        game_type: String,
        player1: String,
        player2: String,
        winner: String,
        time_played: String,
    }

    #[get("/history")]
    pub fn get_histories(collection: State<Collection>) -> Json<Vec<HistoryInfo>> {
        let mut history_info: Vec<HistoryInfo> = Vec::new();
        
        match collection.find(None, None) {
            Ok(cursor) => {
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
                        Err(_) => eprintln!("result is not a document")
                    }
                }
            },
            Err(_) => eprintln!("collection find failed")
        }

        Json(history_info)
    }

    #[post("/history", format="json", data="<history>")]
    pub fn insert_history(collection: State<Collection>, history: Json<HistoryInfo>) {
        if let Ok(Bson::Document(document)) = bson::to_bson(&history.into_inner()) {
            collection.insert_one(document, None);
        }
    }
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("target/deploy/index.html")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("target/deploy/").join(file)).ok()
}

fn connect_to_collection() -> Result<Collection, Error> {
    let collection = Client::with_uri_str("mongodb://localhost:27017/")?
            .database("Connect4DB")
            .collection("games");

    Ok(collection)
}

fn main() {
    let result = connect_to_collection();

    match result {
        Ok(collection) => {
            let config = Config::build(Environment::Staging)
                .address("127.0.0.1")
                .port(8000)
                .finalize()
                .unwrap();
            
            let rocket = rocket::custom(config).manage(collection).mount("/", routes![
                index,
                files,
                database::get_histories,
                database::insert_history
            ]);
            rocket.launch();
        },
        Err(err) => eprintln!("Failed to start backend: {}", err),
    }
}