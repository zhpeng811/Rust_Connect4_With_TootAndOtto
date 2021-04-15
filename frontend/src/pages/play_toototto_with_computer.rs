use yew::prelude::*;
use crate::pages::text_input::TextInput;

pub enum Msg {
    StartGame,
    SetPlayer1Name(String)
}

pub struct PlayTOOTOTTOWithComputer {
    player1_name: String,
    player2_name: String,
    link: ComponentLink<Self>,
    disable_button: bool
}

impl Component for PlayTOOTOTTOWithComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            player1_name: String::from(""),
            player2_name: String::from("Computer"),
            link,
            disable_button: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartGame => {
                self.disable_button = true;
                log::info!("Player 1 name: {}", self.player1_name);
                log::info!("Player 2 name: {}", self.player2_name);
                true
            }
            Msg::SetPlayer1Name(name) => {
                self.player1_name = name;
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
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>

                <div class="col-md-offset-3 col-md-8">
                    <TextInput 
                        value = self.player1_name.clone()
                        placeholder = "Player's Name"
                        oninput = self.link.callback(Msg::SetPlayer1Name)
                        disabled = {self.disable_button}
                    />
                    <button 
                        onclick = self.link.callback(|_| Msg::StartGame)
                        disabled = {self.disable_button}
                    >
                    {"Start Game"}
                    </button>
                </div>
                
                <canvas id="gameboard" height="480" width="640"></canvas>
            </div>
                
        }
    }
}
