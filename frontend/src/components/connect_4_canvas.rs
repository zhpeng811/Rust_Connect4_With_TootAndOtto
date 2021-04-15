use anyhow::Error;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::Date;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};
use stdweb::web::event::ClickEvent;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{prelude::*, virtual_dom::VNode, Properties};

use super::ai_difficulty::Difficulty;
use model::game::*;

use crate::pages::game_history::HistoryInfo;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub struct CanvasModel {
    props: Props,
    canvas_id: String,
    canvas: Option<CanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    cbk: Callback<ClickEvent>,
    animate_cbk: Callback<(usize, usize, usize, usize, bool)>,
    game: BoardGame,
    won: bool,
    paused: bool,
    reject_click: bool,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<CanvasModel>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub difficulty: Difficulty,
    pub canvas_id: Option<String>,
    pub game_done_cbk: Callback<i64>,
}

pub enum Message {
    Click(ClickEvent),
    AnimateCallback((usize, usize, usize, usize, bool)),
    Ignore,
}

impl CanvasModel {
        pub fn reset(&mut self) {
        self.game = BoardGame::new_connect4(6, 7, self.canvas_id == "connect_computer");
        self.paused = false;
        self.won = false;
        self.reject_click = false;
        self.clear();
        self.draw_mask();
    }

    pub fn draw_circle(&self, x: u32, y: u32, fill: &str, stroke: &str, text: &str) {
        self.ctx.as_ref().unwrap().save();
        self.ctx.as_ref().unwrap().set_fill_style_color(&fill);
        self.ctx.as_ref().unwrap().set_stroke_style_color(&stroke);
        self.ctx.as_ref().unwrap().begin_path();
        self.ctx
            .as_ref()
            .unwrap()
            .arc(x as f64, y as f64, 25.0, 0.0, 2.0 * 3.14159265359, false);
        self.ctx.as_ref().unwrap().fill(FillRule::NonZero);
        self.ctx.as_ref().unwrap().restore();

        let context = self.ctx.as_ref().unwrap();
        context.set_font("bold 30px serif");
        context.restore();
        context.fill_text(text, x as f64 - 12.0, y as f64 + 12.0, None);
    }

    pub fn draw_mask(&self) {
        self.ctx.as_ref().unwrap().save();
        self.ctx.as_ref().unwrap().set_fill_style_color("#00bfff");
        self.ctx.as_ref().unwrap().begin_path();
        for y in 0..6 {
            for x in 0..7 {
                self.ctx.as_ref().unwrap().arc(
                    (75 * x + 100) as f64,
                    (75 * y + 50) as f64,
                    25.0,
                    0.0,
                    2.0 * 3.14159265359,
                    false,
                );
                self.ctx.as_ref().unwrap().rect(
                    (75 * x + 150) as f64,
                    (75 * y) as f64,
                    -100.0,
                    100.0,
                );
            }
        }
        self.ctx.as_ref().unwrap().fill(FillRule::NonZero);
        self.ctx.as_ref().unwrap().restore();
    }

    pub fn draw(&self) {
        for x in 0..7 {
            for y in 0..6 {
                let mut fg_color = "transparent";
                let board = self.game.game_board.board.clone();
                if board[y][x] == DiscType::Red {
                    fg_color = "#ff4136";
                } else if board[y][x] == DiscType::Yellow {
                    fg_color = "#ffff00";
                }
                self.draw_circle(
                    (75 * x + 100) as u32,
                    (75 * y + 50) as u32,
                    &fg_color,
                    "black",
                    "",
                );
            }
        }
    }

    pub fn clear(&self) {
        self.ctx.as_ref().unwrap().clear_rect(
            0.0,
            0.0,
            self.canvas.as_ref().unwrap().width() as f64,
            self.canvas.as_ref().unwrap().height() as f64,
        );
    }

    pub fn on_region(&self, coord: f64, x: f64, radius: f64) -> bool {
        return ((coord - x) * (coord - x) <= radius * radius);
    }

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
            _ => return
        }
    }

    pub fn animate(
        &mut self,
        column: usize,
        current_player: usize,
        to_row: usize,
        cur_pos: usize,
        mode: bool,
    ) {
        log::info!("animating {}", to_row);
        let mut fg_color = "transparent";
        if current_player == 1 {
            fg_color = "#ff4136";
        } else if current_player == 2 {
            fg_color = "#ffff00";
        }

        if to_row * 75 >= cur_pos {
            self.clear();
            self.draw();
            self.draw_circle(
                (75 * column + 100) as u32,
                (cur_pos + 50) as u32,
                &fg_color,
                "black",
                "",
            );
            self.draw_mask();

            let cloned = self.animate_cbk.clone();
            window().request_animation_frame(enclose!((cloned) move |_| {
                cloned.emit((column, current_player, to_row, cur_pos+25, mode));
            }));
        } else {
            self.draw();
            self.check();
            self.reject_click = false;
        }
    }

    pub fn action(&mut self, column: usize, mode: bool) -> i64 {
        if self.paused || self.won {
            return 0;
        }

        match self.game.place_disc(column) {
            GameEvent::PlaceSuccess(row) => {
                self.animate(column, self.game.current_player, row, 0, mode);
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
        let mut msg = String::new();
        if winner == 1 {
            msg = format!("{} wins", self.props.player1.as_ref().unwrap());
        } else if winner == 2 {
            msg = format!("{} wins", self.props.player2.as_ref().unwrap());
        } else {
            msg = "It's a draw".to_string();
        }

        let to_print = format!("{} - Click on game board to reset", msg);

        self.ctx.as_ref().unwrap().save();
        self.ctx.as_ref().unwrap().set_font("14pt sans-serif");
        self.ctx.as_ref().unwrap().set_fill_style_color("#111");
        self.ctx
            .as_ref()
            .unwrap()
            .fill_text(&to_print, 150.0, 20.0, None);

        let game = HistoryInfo {
            game_type: String::from("Connect-4"),
            player1: self.props.player1.as_ref().unwrap().clone(),
            player2: self.props.player2.as_ref().unwrap().clone(),
            winner: if winner == 1 {
                self.props.player1.as_ref().unwrap().clone()
            } else if winner == 2 {
                self.props.player2.as_ref().unwrap().clone()
            } else {
                String::from("Draw")
            },
            time_played: Date::from_time(Date::now()).to_string(),
        };

        // construct callback
        let callback = self
            .link
            .callback(move |response: Response<Result<String, Error>>| {
                Message::Ignore
            });

        // construct request
        let request = Request::post("/games")
            .header("Content-Type", "application/json")
            .body(Json(&game))
            .unwrap();

        // send the request
        self.fetch_task = FetchService::fetch(request, callback).ok();

        self.ctx.as_ref().unwrap().restore();
    }
}

impl Component for CanvasModel {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let canvas_id = props.canvas_id.clone().unwrap();
        let game = BoardGame::new_connect4(6, 7, canvas_id == "connect_computer".to_string());

        Self {
            props,
            canvas_id,
            canvas: None,
            ctx: None,
            cbk: link.callback(|e: ClickEvent| Message::Click(e)),
            animate_cbk: link
                .callback(|e: (usize, usize, usize, usize, bool)| Message::AnimateCallback(e)),
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

                for j in 0..7 {
                    if self.on_region(x, (75 * j + 100) as f64, 25 as f64) {
                        self.paused = false;
                        log::info!("clicked column {}", j);
                        let valid = self.action(j, false);
                        if valid == 1 {
                            self.reject_click = true;
                        };

                        break;
                    }
                }
            }
            Message::AnimateCallback((a, b, c, d, e)) => {
                self.animate(a, b, c, d, e);
            }
            Message::Ignore => {}
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
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
            self.ctx = Some(context(self.canvas_id.as_str()));

            let ctx = self.ctx.as_ref().unwrap();
            let cloned_cbk = self.cbk.clone();

            self.canvas.as_ref().unwrap().add_event_listener(enclose!(
                (ctx) move | event: ClickEvent | {
                    cloned_cbk.emit(event);
                }
            ));

            // clears and draws mask
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
