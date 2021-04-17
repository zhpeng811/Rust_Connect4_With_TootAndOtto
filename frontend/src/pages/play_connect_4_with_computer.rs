use yew::prelude::*;
use yew_components::Select;
use model::ai::Difficulty;
use model::board_size::BoardSize;
use crate::components::canvas_model::CanvasModel;
use crate::components::text_input::TextInput;
use crate::components::alert::alert;

pub enum Msg {
    StartGame,
    EndGame,
    SetPlayer1Name(String),
    SetDifficulty(Difficulty),
    SetBoardSize(BoardSize)
}

pub struct PlayConnect4WithComputer {
    player1_name: String,
    player2_name: String,
    game_running: bool,
    disable_button: bool,
    display_board: String,
    board_size: BoardSize,
    difficulty: Difficulty,
    link: ComponentLink<Self>,
}

impl Component for PlayConnect4WithComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            player1_name: String::from(""),
            player2_name: String::from("Computer"),
            game_running: false,
            disable_button: false,
            display_board: String::from("none"),
            board_size: BoardSize::SevenBySix,
            difficulty: Difficulty::Easy,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartGame => { 
                if self.player1_name.len() == 0 {
                    alert("Player name field cannot be empty");
                    return false
                }
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
            Msg::SetBoardSize(board_size) => {
                self.board_size = board_size;
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
                    {"\u{00a0}\u{00a0}\u{00a0}\u{00a0}"} // add some spaces between the elements
                    <Select // list of props: https://docs.rs/yew-components/0.2.0/yew_components/struct.Select.html
                        <Difficulty>
                        selected = Some(self.difficulty)
                        options = {Difficulty::to_vec()}
                        disabled = {self.disable_button}
                        on_change = self.link.callback(|dif: Difficulty| Msg::SetDifficulty(dif))
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
                    <small>{format!("(Disc Colors: {} - ", self.player1_name)} <b>{"Red"}</b> {format!("   and    {} - ", self.player2_name)} <b>{"Yellow)"}</b></small>
                    <br/>
                    <CanvasModel:
                        canvas_id = "connect4_computer"
                        player1 = self.player1_name.clone()
                        player2 = self.player2_name.clone()
                        board_rows = self.board_size.get_row()
                        board_columns = self.board_size.get_column()
                        text = String::from("")
                        difficulty = self.difficulty
                        game_done_cbk=self.link.callback(|_| Msg::EndGame)/>
                </div>
            </div>
                
        }
    }
}
