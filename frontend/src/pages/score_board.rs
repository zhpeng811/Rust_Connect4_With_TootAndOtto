use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::Json;
use std::collections::HashMap;
use crate::types::HistoryInfo;

// this Struct and its impelementation is very similar to GameHistory in game_history.rs
// only the stats functions and htmls are different 
pub struct ScoreBoard {
    link: ComponentLink<Self>,
    history: Vec<HistoryInfo>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    FetchSuccess(Vec<HistoryInfo>),
    FetchDataError,
    FetchFailed,
}

impl ScoreBoard {
    fn fetch(&mut self) -> FetchTask {
        let callback = self.link.callback(
            |response: Response<Json<Result<Vec<HistoryInfo>, anyhow::Error>>>| {
                let (parts, Json(body)) = response.into_parts();
                if parts.status.is_success() {
                    if let Ok(data) = body {
                        Msg::FetchSuccess(data)
                    } else {
                        // fetched data contains error
                        Msg::FetchDataError
                    }
                } else {
                    // fetch failed
                    Msg::FetchFailed
                }
            }
        );

        let request = Request::get("http://127.0.0.1:8000/history").body(yew::format::Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }

    fn games_won_by_computer_stat(&self) -> Html {
        if self.history.len() > 0 {
            html! {
                <tr>
                    <td> { self.history.len() } </td> // Total Games Played
                    <td> { self.history.iter().filter(|history| history.player2.eq("Computer")).count() } </td> // Games Againest Computer
                    <td> { self.history.iter().filter(|history| history.winner.eq("Computer")).count() }</td> // Games Computer Won

                </tr>
            }
        } else {
            html! {
                <tr>
                    <td colspan="3"> {"Failed to get history or no game history exist"} </td>
                </tr>
            }
        }
    }

    fn detail_won_by_computer_stat(&self) -> Html {
        if self.history.len() > 0 {
            html! {
                { self.history.iter().filter(|history| history.winner.eq("Computer")).enumerate().map(|(i, history)| {
                    html! {
                        <tr>
                            <td> { i + 1 } </td> // Sl. No.
                            <td> { history.game_type.clone() } </td> // Game Type
                            <td> { history.winner.clone() } </td> // Winner
                            <td> { history.player1.clone() } </td> // Played Against
                            <td> { history.time_played.clone()} </td> // When Played
  
                        </tr>
                    }
                }).collect::<Html>() }
            }
        } else {
            html! {
                <tr>
                    <td colspan="5"> {"Failed to get history or no game history exist"} </td>
                </tr>
            }
        }
    }

    fn champion_games_stat(&self, filter_cond: String) -> Html {
        if self.history.len() > 0 {
            let mut map = HashMap::new();
            self.history.iter().filter(|history| 
                if filter_cond.len() == 0 {
                    return true
                } else {
                    return history.game_type.eq(&filter_cond)
                }
            ).for_each(|history| {
                *map.entry(history.winner.clone()).or_insert(0) += 1;
            });
            
            // sorting from https://stackoverflow.com/a/63966206
            let mut hash_vec: Vec<(&String, &u32)> = map.iter().collect();
            hash_vec.sort_by(|a, b| b.1.cmp(a.1));

            html! {
                { hash_vec.iter().enumerate().map(|(i, (winner, num_wins))| {
                    html! {
                        <tr>
                            <td> { i + 1 } </td> // Sl. No.
                            <td> { winner } </td> // Winner or draw
                            <td> { num_wins } </td> // No. of Wins
                        </tr>
                    }
                }).collect::<Html>() }
            }
        } else {
            html! {
                <tr>
                    <td colspan="3"> {"Failed to get history or no game history exist"} </td>
                </tr>
            }
        }
    }
}

impl Component for ScoreBoard {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut this = Self {
            link,
            history: Vec::new(),
            fetch_task: None
        };
        // store the task so it isn't cancelled immediately
        this.fetch_task = Some(this.fetch());
        this
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSuccess(data) => {
                self.history = data;
                self.fetch_task = None;
            }
            Msg::FetchFailed => log::info!("fetching history failed"),
            Msg::FetchDataError => log::info!("fetched data contains error"),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round" />
                <div> 
                    <h4> {"Games Won by Computer"} </h4> 
                </div>

                <table>
                    <tr>
                        <th> {"Total Games Played"} </th>
                        <th> {"Games Against Computer"} </th>
                        <th> {"Games Computer Won"} </th>
                    </tr>

                    { self.games_won_by_computer_stat() } 
                </table>
        
                <br />
        
                <div> 
                    <h4> {"Details of Games Won by Computer"} </h4> 
                </div>

                <table>
                    <tr>
                        <th> {"Sl. No."} </th>
                        <th> {"Game Type"} </th>
                        <th> {"Winner"} </th>
                        <th> {"Played Against"} </th>
                        <th> {"When Played (UTC Time)"} </th>
                    </tr>
                    
                    { self.detail_won_by_computer_stat() }
                </table>
        
                <br />

                <div>
                    <h4> {"Champion players for Connect-4"} </h4>
                </div>

                <table>
                    <tr>
                        <th> {"Sl. No."} </th>
                        <th> {"Player Name"} </th>
                        <th> {"No. of Wins"} </th>
                    </tr>
                    
                    { self.champion_games_stat(String::from("Connect-4")) }
                </table>

                <br />

                <div>
                    <h4> {"Champion players for TOOT-and-OTTO"} </h4>
                </div>

                <table>
                    <tr>
                        <th> {"Sl. No."} </th>
                        <th> {"Player Name"} </th>
                        <th> {"No. of Wins"} </th>
                    </tr>
                    
                    { self.champion_games_stat(String::from("TOOT-OTTO")) }
                </table>

                <br />

                <div>
                    <h4> {"Champion players for All Games"} </h4>
                </div>

                <table>
                    <tr>
                        <th> {"Sl. No."} </th>
                        <th> {"Player Name"} </th>
                        <th> {"No. of Wins"} </th>
                    </tr>
                    
                    { self.champion_games_stat(String::from("")) }
                </table>

            </div>
        }
    }
}
