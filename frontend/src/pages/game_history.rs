// code skeleton from yew documentation: https://yew.rs/docs/en/next/concepts/services/fetch

use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryInfo {
    pub game_type: String,
    pub player1: String,
    pub player2: String,
    pub winner: String,
    pub difficulty: String,
    pub time_played: String,
}

pub struct GameHistory {
    link: ComponentLink<Self>,
    history: Vec<HistoryInfo>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    FetchSuccess(Vec<HistoryInfo>),
    FetchDataError,
    FetchFailed,
}

impl GameHistory {
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

    fn view_history(&self) -> Html {
        if self.history.len() > 0 {
            html! {
                { self.history.iter().enumerate().map(|(i, history)| {
                    html! {
                        <tr>
                            <td> { i + 1 } </td>
                            <td> { history.game_type.clone() } </td>
                            <td> { history.player1.clone() } </td>
                            <td> { history.player2.clone() } </td>
                            <td> { history.difficulty.clone() } </td>
                            <td> { history.winner.clone()} </td>
                            <td> { history.time_played.clone() } </td>
                        </tr>
                    }
                }).collect::<Html>() }
            }
        } else {
            html! {
                <tr>
                    <td colspan="7"> {"Failed to get history or no game history exist"} </td>
                </tr>
            }
        }
    }
}

impl Component for GameHistory {
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
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round" />
                
                <table>
                    <tr>
                        <th>{"Game ID"}</th>
                        <th>{"Game Type"}</th>
                        <th>{"Player1"}</th>
                        <th>{"Player2"}</th>
                        <th>{"Difficulty"}</th>
                        <th>{"Winner"}</th>
                        <th>{"When Played (UTC Time)"}</th>
                    </tr>

                    { self.view_history() }
                
                </table>
            </div>
        }
    }
}
