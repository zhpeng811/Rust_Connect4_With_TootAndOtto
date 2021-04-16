use yew::prelude::*;
use yew::html::InputData;
use crate::pages::text_input::TextInput;
use crate::components::canvas_model::CanvasModel;
use yew_components::Select;
use crate::components::alert::alert;
use model::ai::Difficulty;
use model::board_size::BoardSize;

pub enum Msg {
    StartGame,
    SetPlayer1Name(String),
    SetPlayer2Name(String),
    UpdateText(String),
    SetBoardSize(BoardSize),
    EndGame
}

pub struct PlayTOOTOTTOWithHuman {
    player1_name: String,
    player2_name: String,
    game_running: bool,
    disable_button: bool,
    display_board: String,
    text: String,
    board_size: BoardSize,
    link: ComponentLink<Self>,
    update_text: Callback<InputData>
}

impl Component for PlayTOOTOTTOWithHuman {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            player1_name: String::from(""),
            player2_name: String::from(""),
            game_running: false,
            disable_button: false,
            text: String::from("T"), // default as T
            board_size: BoardSize::SixByFour,
            display_board: String::from("none"),
            update_text: link.callback(|input_data: InputData| Msg::UpdateText(input_data.value.to_string())),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartGame => {
                if self.player1_name.len() == 0 || self.player2_name.len() == 0 {
                    alert("Player name field cannot be empty");
                    return false
                }
                self.disable_button = true;
                self.game_running = true;
                self.display_board = String::from("block");
            }
            Msg::SetPlayer1Name(name) => {
                self.player1_name = name;
            }
            Msg::SetPlayer2Name(name) => {
                self.player2_name = name;
            }
            Msg::UpdateText(text) => {
                self.text = text.clone();
            }
            Msg::SetBoardSize(board_size) => {
                self.board_size = board_size;
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
                    {"\u{00a0}\u{00a0}\u{00a0}\u{00a0}"} // add some spaces between the elements
                    <TextInput 
                        value = self.player2_name.clone()
                        placeholder = "Player 2's Name"
                        oninput = self.link.callback(Msg::SetPlayer2Name)
                        disabled = {self.disable_button}
                    />
                    {"\u{00a0}\u{00a0}\u{00a0}\u{00a0}"} // add some spaces between the elements
                    <Select 
                        <BoardSize>
                        selected = Some(self.board_size)
                        options = {BoardSize::to_vec()}
                        disabled = {self.disable_button}
                        on_change = self.link.callback(|size: BoardSize| Msg::SetBoardSize(size))
                    />
                    {"\u{00a0}\u{00a0}\u{00a0}\u{00a0}"} // add some spaces between the elements
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
                        canvas_id = "toototto_human"
                        player1 = self.player1_name.clone()
                        player2 = self.player2_name.clone()
                        board_rows = self.board_size.get_row()
                        board_columns = self.board_size.get_column()
                        text = self.text.clone()
                        difficulty = Difficulty::Easy
                        game_done_cbk=self.link.callback(|_| Msg::EndGame)/>
                </div>
            </div>
                
        }
    }
}
