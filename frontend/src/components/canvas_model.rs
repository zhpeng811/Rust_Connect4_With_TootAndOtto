use anyhow::Error;
use stdweb::{
    web::{
        FillRule, 
        document, 
        window, 
        CanvasRenderingContext2d, 
        event::ClickEvent,
        html_element::CanvasElement
    },
    traits::*,
    unstable::TryInto,
};
use yew::{
    prelude::*,
    format::Json,
    services::fetch::{
        FetchService, 
        FetchTask, 
        Request, 
        Response
    },
};

use model::ai::{Difficulty, Connect4AI, TootOttoAI};
use model::game::*;

use crate::pages::game_history::HistoryInfo;

pub struct CanvasModel {
    props: Props,
    canvas_id: String,
    vs_ai: bool,
    game_type: GameType,
    board_rows: usize,
    board_columns: usize,
    text: String,
    canvas: Option<CanvasElement>,
    canvas_render_context: Option<CanvasRenderingContext2d>,
    cbk: Callback<ClickEvent>,
    animate_cbk: Callback<(usize, usize, usize, bool)>,
    game: BoardGame,
    won: bool,
    paused: bool,
    reject_click: bool,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<CanvasModel>
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub board_rows: Option<usize>,
    pub board_columns: Option<usize>,
    pub text: Option<String>,
    pub difficulty: Difficulty,
    pub canvas_id: Option<String>,
    pub game_done_cbk: Callback<i64>,
}

pub enum Message {
    Click(ClickEvent),
    AnimateCallback((usize, usize, usize, bool)),
    PostSuccess(String),
    PostFailedWithErr(Error),
    PostFailed
}

impl CanvasModel {
    pub fn reset(&mut self) {
        self.game = {
            if self.game_type == GameType::Connect4 {
                BoardGame::new_connect4(self.board_rows, self.board_columns, self.vs_ai)
            } else {
                BoardGame::new_toot_and_otto(self.board_rows, self.board_columns, self.vs_ai)
            }
        };
        self.paused = false;
        self.won = false;
        self.reject_click = false;
        self.clear();
        self.draw_mask();
    }

    // Same for both Connect 4 and TOOT-and-OTTO
    pub fn draw_circle(&self, x: u32, y: u32, fill: &str, stroke: &str, text: &str) {
        let context = self.canvas_render_context.as_ref().unwrap();
        context.save();
        context.set_fill_style_color(&fill);
        context.set_stroke_style_color(&stroke);
        context.begin_path();
        context.arc(x as f64, y as f64, 25.0, 0.0, 2.0 * 3.1415926, false);
        context.fill(FillRule::NonZero);
        context.restore();
        context.set_font("bold 30px serif");
        context.restore();
        context.fill_text(text, x as f64 - 12.0, y as f64 + 12.0, None);
    }

    // Same for both Connect 4 and TOOT-and-OTTO
    pub fn draw_mask(&self) {
        let context = self.canvas_render_context.as_ref().unwrap();
        context.save();
        context.set_fill_style_color("#00bfff");
        context.begin_path();
        for y in 0..self.board_rows {
            for x in 0..self.board_columns {
                context.arc(
                    (75 * x + 100) as f64,
                    (75 * y + 50) as f64,
                    25.0,
                    0.0,
                    2.0 * 3.1415926,
                    false,
                );
                context.rect(
                    (75 * x + 150) as f64,
                    (75 * y) as f64,
                    -100.0,
                    100.0,
                );
            }
        }
        context.fill(FillRule::NonZero);
        context.restore();
    }

    // Different for Connect 4 and TOOT-and-OTTO
    pub fn draw(&self) {
        for x in 0..self.board_columns {
            for y in 0..self.board_rows {
                let mut text = "";
                let mut fg_color = "transparent";
                let board = self.game.game_board.board.clone();

                if self.game_type == GameType::Connect4 {
                    if board[y][x] == DiscType::Red {
                        fg_color = "#ff4136";
                    } else if board[y][x] == DiscType::Yellow {
                        fg_color = "#ffa500";
                    }
                } else if self.game_type == GameType::TOOTandOTTO {
                    if board[y][x] == DiscType::T {
                        text = "T";
                        fg_color = "#99ffcc";
                    } else if board[y][x] == DiscType::O {
                        text = "O";
                        fg_color = "#99ffcc";
                    }
                }

                self.draw_circle(
                    (75 * x + 100) as u32,
                    (75 * y + 50) as u32,
                    &fg_color,
                    "black",
                    text,
                );
            }
        }
    }

    // Same for both Connect 4 and TOOT-and-OTTO
    pub fn clear(&self) {
        let context = self.canvas_render_context.as_ref().unwrap();
        let canvas = self.canvas.as_ref().unwrap();
        context.clear_rect(
            0.0,
            0.0,
            canvas.width() as f64,
            canvas.height() as f64,
        );
    }

    // Same for both Connect 4 and TOOT-and-OTTO
    pub fn on_region(&self, coord: f64, x: f64, radius: f64) -> bool {
        return (coord - x) * (coord - x) <= radius * radius;
    }

    // Same for both Connect 4 and TOOT-and-OTTO
    pub fn check(&mut self) {
        match self.game.check() {
            GameEvent::Player1Win => {
                self.record_match(1);
            },
            GameEvent::Player2Win => {
                self.record_match(2);
            },
            GameEvent::Draw => {
                self.record_match(0);
            },
            _ => ()
        }
    }

    // Mostly the same for both games, some minor difference
    pub fn animate(&mut self, column: usize, row: usize, cur_pos: usize, mode: bool) {
        // log::info!("animating {}", row);
        let mut fg_color = "transparent";
        if self.game.current_player == 1 && self.game_type == GameType::Connect4 {
            fg_color = "#ff4136";
        } else if self.game.current_player == 2 && self.game_type == GameType::Connect4 {
            fg_color = "#ffa500";
        } else if self.game_type == GameType::TOOTandOTTO {
            fg_color = "#99ffcc";
        }

        if row * 75 >= cur_pos {
            self.clear();
            self.draw();
            self.draw_circle(
                (75 * column + 100) as u32,
                (cur_pos + 50) as u32,
                &fg_color,
                "black",
                &self.text[..],
            );
            self.draw_mask();

            let cloned = self.animate_cbk.clone();
            window().request_animation_frame(move |_| cloned.emit((column, row, cur_pos + 25, mode)));
        } else {
            self.draw();
            self.check();
            if self.vs_ai && self.game_type == GameType::Connect4 && self.game.current_player == 2 {
                let mut connect4_ai = Connect4AI::new(self.board_rows, self.board_columns, self.props.difficulty);
                let best_move = connect4_ai.find_best_move(self.game.clone());
                log::info!("Computer Choose to place at column {}", best_move);
                self.paused = false;
                self.action(best_move, true);
            } else if self.vs_ai && self.game_type == GameType::TOOTandOTTO && self.game.current_player == 2 {
                let toototto_ai = TootOttoAI::new(self.board_rows, self.board_columns, self.props.difficulty);
                let (best_move, disc_type) = toototto_ai.find_best_move(self.game.clone());

                let current_disc_type = self.game.get_current_disc_type(); // record the current disc type
                self.game.change_disc_type(disc_type); // set disc type to whatever the AI return
                log::info!("Computer Choose to place at column {}", best_move);
                self.paused = false;
                self.action(best_move, true); // place the piece
                self.game.change_disc_type(current_disc_type); // reset the disc type back to original
            } else {
                self.reject_click = false;
            }
        }
    }

    pub fn action(&mut self, column: usize, mode: bool) -> i64 {
        if self.paused || self.won {
            return 0;
        }

        match self.game.place_disc(column) {
            GameEvent::PlaceSuccess(row) => {
                self.animate(column, row, 0, mode);
                self.paused = true;
                return 1
            },
            _ => return 0
        }
    }

    pub fn record_match(&mut self, winner: usize) {
        self.paused = true;
        self.won = true;
        self.reject_click = false;
        let msg: String;
        if winner == 1 {
            msg = format!("{} wins", self.props.player1.as_ref().unwrap());
        } else if winner == 2 {
            msg = format!("{} wins", self.props.player2.as_ref().unwrap());
        } else {
            msg = "It's a draw".to_string();
        }

        let to_print = format!("{} - Click on game board to reset", msg);

        let context = self.canvas_render_context.as_ref().unwrap();
        context.save();
        context.set_font("14pt sans-serif");
        context.set_fill_style_color("#111");
        context.fill_text(&to_print, 150.0, 20.0, None);

        let difficulty = if self.vs_ai {self.props.difficulty.to_string()} else {"N/A".to_string()};

        let history = HistoryInfo {
            game_type: self.game_type.to_string(),
            player1: self.props.player1.as_ref().unwrap().clone(),
            player2: self.props.player2.as_ref().unwrap().clone(),
            winner: if winner == 1 {
                self.props.player1.as_ref().unwrap().clone()
            } else if winner == 2 {
                self.props.player2.as_ref().unwrap().clone()
            } else {
                String::from("Draw")
            },
            difficulty: difficulty,
            time_played: "".to_string(), // doesn't matter here, backend will use the current time
        };

        // create callback for POST request to backend
        let callback = self.link.callback(move |response: Response<Result<String, Error>>| {
            let (parts, body) = response.into_parts();
            if parts.status.is_success() {
                match body {
                    Ok(msg) => Message::PostSuccess(msg),
                    Err(err) => Message::PostFailedWithErr(err),
                }
            } else {
                Message::PostFailed
            }
        });

        // create the POST request
        let request = Request::post("http://127.0.0.1:8000/history")
            .header("Content-Type", "application/json")
            .body(Json(&history))
            .unwrap();

        // send the POST request
        self.fetch_task = FetchService::fetch(request, callback).ok();

        context.restore();
    }
}

impl Component for CanvasModel {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let canvas_id = props.canvas_id.clone().unwrap();
        let board_rows = props.board_rows.unwrap();
        let board_columns = props.board_columns.unwrap();
        let vs_ai: bool;
        let game_type: GameType;
        if canvas_id.eq("connect4_computer") {
            vs_ai = true;
            game_type = GameType::Connect4;
        } else if canvas_id.eq("connect4_human") {
            vs_ai = false;
            game_type = GameType::Connect4;
        } else if canvas_id.eq("toototto_computer") {
            vs_ai = true;
            game_type = GameType::TOOTandOTTO;
        } else {
            vs_ai = false;
            game_type = GameType::TOOTandOTTO;
        }

        let game = {
            if game_type == GameType::Connect4 {
                log::info!("creating game for connect4");
                BoardGame::new_connect4(board_rows, board_columns, vs_ai)
            } else {
                log::info!("creating game for TOOT and OTTO");
                BoardGame::new_toot_and_otto(board_rows, board_columns, vs_ai)
            }
        };

        Self {
            props,
            canvas_id,
            vs_ai,
            game_type,
            board_rows,
            board_columns,
            text: String::from(""),
            canvas: None,
            canvas_render_context: None,
            cbk: link.callback(|e: ClickEvent| Message::Click(e)),
            animate_cbk: link.callback(|e: (usize, usize, usize, bool)| Message::AnimateCallback(e)),
            game,
            paused: false,
            won: false,
            reject_click: false,
            fetch_task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Click(e) => {
                if self.reject_click {
                    return false;
                }

                if self.won {
                    self.reset();
                    self.props.game_done_cbk.emit(0);
                    return true;
                }

                let rect = self.canvas.as_ref().unwrap().get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.get_left();

                for j in 0..self.board_columns {
                    if self.on_region(x, (75 * j + 100) as f64, 25 as f64) {
                        self.paused = false;
                        let valid = self.action(j, false);
                        if valid == 1 {
                            self.reject_click = true;
                        };

                        break;
                    }
                }
            }
            Message::AnimateCallback((column, row, cur_pos, mode)) => {
                self.animate(column, row, cur_pos, mode);
            }
            Message::PostSuccess(msg) => log::info!("game history successfully recorded: {}", msg),
            Message::PostFailedWithErr(err) => log::info!("failed to record game history with err: {}", err),
            Message::PostFailed => log::info!("failed to record game history")
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        // Only TOOT-and-OTTO is allowed to change disc-type / text
        if self.game_type == GameType::TOOTandOTTO {
            self.text = self.props.text.as_ref().unwrap().clone(); 
            if self.text.eq("T") {
                self.game.change_disc_type(DiscType::T);
            } else if self.text.eq("O") {
                self.game.change_disc_type(DiscType::O);
            } else {
                log::info!("unknown disc type, should never happen");
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <canvas id={&self.canvas_id} height="480" width="640"></canvas>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.canvas = Some(canvas(self.canvas_id.as_str()));
            self.canvas_render_context = Some(context(self.canvas_id.as_str()));

            let cloned_cbk = self.cbk.clone();

            self.canvas.as_ref().unwrap().add_event_listener(
                move |event: ClickEvent| cloned_cbk.emit(event)
            );

            self.reset();
        }
    }
}

#[inline(always)]
fn canvas(id: &str) -> CanvasElement {
    document()
        .query_selector(&format!("#{}", id))
        .unwrap()
        .expect(&format!("Failed to select canvas id #{}", id))
        .try_into()
        .unwrap()
}

#[inline(always)]
fn context(id: &str) -> CanvasRenderingContext2d {
    canvas(id).get_context().unwrap()
}
