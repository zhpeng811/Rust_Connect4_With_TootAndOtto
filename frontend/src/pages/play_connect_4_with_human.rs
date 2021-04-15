use yew::prelude::*;
use crate::pages::text_input::TextInput;
use crate::components::canvas_model::CanvasModel;
use crate::components::ai_difficulty::{Difficulty};

pub enum Msg {
    StartGame,
    EndGame,
    SetPlayer1Name(String),
    SetPlayer2Name(String),
}

pub struct PlayConnect4WithHuman {
    player1_name: String,
    player2_name: String,
    game_running: bool,
    disable_button: bool,
    display_board: String,
    link: ComponentLink<Self>,
}

impl Component for PlayConnect4WithHuman {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            player1_name: String::from(""),
            player2_name: String::from(""),
            game_running: false,
            disable_button: false,
            display_board: String::from("none"),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartGame => {
                self.game_running = true;
                self.disable_button = true;
                self.display_board = String::from("block");
            }
            Msg::SetPlayer1Name(name) => {
                self.player1_name = name;
            }
            Msg::SetPlayer2Name(name) => {
                self.player2_name = name;
            }
            Msg::EndGame => {
                self.game_running = false;
                self.disable_button = false;
                self.display_board = String::from("none");
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>

                <div class="col-md-offset-3 col-md-8">
                    <TextInput 
                        value = self.player1_name.clone()
                        placeholder = "Player 1's Name"
                        oninput = self.link.callback(Msg::SetPlayer1Name)
                        disabled = {self.disable_button}
                    />
                    <TextInput 
                        value = self.player2_name.clone()
                        placeholder = "Player 2's Name"
                        oninput = self.link.callback(Msg::SetPlayer2Name)
                        disabled = {self.disable_button}
                    />
                    <button
                        id = "start-button"
                        onclick= self.link.callback(|_| Msg::StartGame)
                        disabled = {self.disable_button}
                    >
                        {"Start Game"}
                    </button>
                </div>
                <div style=format!("display: {}", self.display_board)>
                    <br/>
                    <h4>{format!("New Game: {} Vs {}", self.player1_name, self.player2_name)}</h4>
                    <small>{format!("(Disc Colors: {} - ", self.player1_name)} <b>{"Red"}</b> {format!("   and    {} - ", self.player2_name)} <b>{"Yellow)"}</b></small>
                    <br/>
                    <CanvasModel:
                        canvas_id = "connect4_human"
                        player1 = self.player1_name.clone()
                        player2 = self.player2_name.clone()
                        board_rows = 6
                        board_columns = 7
                        text = String::from("")
                        difficulty = Difficulty::Easy
                        game_done_cbk=self.link.callback(|_| Msg::EndGame)/>
                </div>
            </div>
        }
    }
}
