use yew::prelude::*;
use mongodb_lib::get_histories; 

pub struct HistoryInfo {
    game_type: String,
    player1: String,
    player2: String,
    winner: String,
    time_played: String,
}

pub struct GameHistory {
    link: ComponentLink<Self>,
    history: Vec<HistoryInfo>,
}

impl Component for GameHistory {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let history = get_histories();
        Self {
            link,
            history,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round" />
    
	            <div id="game-stream">
	                <table>
			            <tr>
                            <th>{"Game-ID"}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
  			            </tr>

                        { for self.history.iter().enumerate().map(
                            |(i, game)| html! {
                                <tr>
                                <td>{i + 1}</td>
                                <td>{game.game_type.clone()}</td>
                                <td>{game.player1.clone()}</td>
                                <td>{game.player2.clone()}</td>
                                <td>{game.winner.clone()}</td>
                                <td>{game.time_played.clone()}</td>
                                </tr>
                            }
                        )}
		            </table>
			    </div>
		    </div>
        }
    }
}
