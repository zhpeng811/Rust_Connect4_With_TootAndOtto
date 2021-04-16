use yew::prelude::*;
use crate::pages::text_input::TextInput;
use yew_components::Select;
use model::ai::Difficulty;
use crate::components::canvas_model::CanvasModel;

pub enum Msg {
    StartGame,
    EndGame,
    SetPlayer1Name(String),
    SetDifficulty(Difficulty),
    UpdateText(String),
}

pub struct PlayTOOTOTTOWithComputer {
    player1_name: String,
    player2_name: String,
    game_running: bool,
    disable_button: bool,
    display_board: String,
    text: String,
    difficulty: Difficulty,
    link: ComponentLink<Self>,
    update_text: Callback<InputData>
}

impl Component for PlayTOOTOTTOWithComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            player1_name: String::from(""),
            player2_name: String::from("Computer"),
            game_running: false,
            disable_button: false,
            display_board: String::from("none"),
            difficulty: Difficulty::Easy,
            text: String::from("T"), // default as T
            update_text: link.callback(|input_data: InputData| Msg::UpdateText(input_data.value.to_string())),
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
            Msg::SetDifficulty(difficulty) => {
                self.difficulty = difficulty;
            }
            Msg::UpdateText(text) => {
                self.text = text.clone();
            }
            Msg::EndGame => {
                self.game_running = false;
                self.disable_button = false;
                self.display_board = String::from("none");
            }
        }
        true
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
                    <Select // list of props: https://docs.rs/yew-components/0.2.0/yew_components/struct.Select.html
                        <Difficulty>
                        selected = Some(self.difficulty)
                        options = {Difficulty::to_vec()}
                        disabled = {self.disable_button}
                        on_change = self.link.callback(|dif: Difficulty| Msg::SetDifficulty(dif))/>
                    <button 
                        onclick = self.link.callback(|_| Msg::StartGame)
                        disabled = {self.disable_button}
                    >
                    {"Start Game"}
                    </button>
                </div>
                <div style=format!("display: {}", self.display_board)>
                    <br/>
                    <h4>{format!("New Game: {} Vs {}", self.player1_name, self.player2_name)}</h4>
                    <small>{format!("(Winning Combination: {} - ", self.player1_name)} <b>{"TOOT"}</b> {format!("   and    {} - ", self.player2_name)} <b>{"OTTO)"}</b></small>
                    <br/>
                    {"Select a Disc Type:  "}
                    <input type = "radio" name = "choice" value = "T" checked = {self.text.eq("T")} oninput = &self.update_text/> <label for="T"> {"T"} </label>
                    {"\u{00a0}\u{00a0}\u{00a0}\u{00a0}"} // add some spaces between the two inputs
                    <input type = "radio" name = "choice" value = "O" checked = {self.text.eq("O")} oninput = &self.update_text/> <label for="O"> {"O"} </label>
                    <br/>
                    <CanvasModel:
                        canvas_id = "toototto_computer"
                        player1 = self.player1_name.clone()
                        player2 = self.player2_name.clone()
                        board_rows = 6
                        board_columns = 7
                        text = self.text.clone()
                        difficulty = self.difficulty
                        game_done_cbk=self.link.callback(|_| Msg::EndGame)/>
                </div>
            </div>
                
        }
    }
}
