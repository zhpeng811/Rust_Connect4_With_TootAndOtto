#![feature(proc_macro_hygiene, decl_macro)] // needed for rocket crate to use get() and post() macro

#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use mongodb::{Client, Collection, error::Error};
use rocket::config::{Config, Environment};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod database {
    use mongodb::Collection;
    use bson::{doc, Bson};
    use chrono::Utc;
    use rocket::State;
    use rocket_contrib::json::Json;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HistoryInfo {
        pub game_type: String,
        pub player1: String,
        pub player2: String,
        pub winner: String,
        pub difficulty: String,
        pub time_played: String,
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
                            let difficulty = String::from(document.get("Difficulty").and_then(Bson::as_str).unwrap_or("N/A"));
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
                                difficulty,
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
        let body = history.into_inner();

        // the time_played field from HistoryInfo is ignored here
        // front end can pass any string since it won't be used
        let doc = doc! {
            "gameType": body.game_type,
            "Player1Name": body.player1,
            "Player2Name": body.player2,
            "WinnerName": body.winner,
            "Difficulty": body.difficulty,
            "GameDate": Utc::now()
        };
        let _ = collection.insert_one(doc, None);
    }

    #[delete("/history")]
    pub fn delete_history(collection: State<Collection>) {
        collection.drop(None);
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

// using rocket_cors crate example: https://github.com/lawliet89/rocket_cors/blob/master/examples/fairing.rs
fn main() {
    // This will allow the frontend to make HTTP GET and POST requests
    // otherwise the request is rejected due to CORS error
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap();

    let result = connect_to_collection();

    match result {
        Ok(collection) => {
            // making the address just "localhost" doesn't work for frontend
            let config = Config::build(Environment::Staging)
                .address("127.0.0.1")
                .port(8000)
                .finalize()
                .unwrap();
            
            let rocket = rocket::custom(config).manage(collection)
                .mount("/", routes![
                    index,
                    files,
                    database::get_histories,
                    database::insert_history,
                    database::delete_history,
                ])
                .attach(cors);
            rocket.launch();
        },
        Err(err) => eprintln!("Failed to start backend: {}", err),
    }
}
