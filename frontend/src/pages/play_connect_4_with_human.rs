use yew::prelude::*;
use crate::pages::text_input::TextInput;

pub enum Msg {
    StartGame,
    SetPlayer1Name(String),
    SetPlayer2Name(String),
}

pub struct PlayConnect4WithHuman {
    player1_name: String,
    player2_name: String,
    link: ComponentLink<Self>,
}

impl Component for PlayConnect4WithHuman {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            player1_name: String::from(""),
            player2_name: String::from(""),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartGame => {
                log::info!("Player 1 name: {}", self.player1_name);
                log::info!("Player 2 name: {}", self.player2_name);
                true
            }
            Msg::SetPlayer1Name(name) => {
                self.player1_name = name;
                false
            }
            Msg::SetPlayer2Name(name) => {
                self.player2_name = name;
                false
            }
        }
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
                        oninput = self.link.callback(Msg::SetPlayer1Name)
                    />
                    <TextInput 
                        value = self.player2_name.clone()
                        oninput = self.link.callback(Msg::SetPlayer2Name)
                    />
                    <button onclick= self.link.callback(|_| Msg::StartGame)>{"Start Game"}</button>
                </div>
                
                <canvas id="gameboard" height="480" width="640"></canvas>
            </div>
                
        }
    }
}
