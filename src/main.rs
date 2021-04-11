#![recursion_limit="2048"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};

// mod components;
mod pages;
use pages::{
    connect_4_app::Home, 
    how_to_connect_4::HowToConnect4,
    play_connect_4_with_human::PlayConnect4WithHuman,
    how_to_toot::HowToToot, 
    game_history::GameHistory,
    page_not_found::PageNotFound,
};
mod switch;
use switch::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    link: ComponentLink<Self>,
    navbar_active: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            navbar_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_nav() }

                <div class="w3-main" style="margin-left:390px;margin-right:40px">
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                        })
                    />
                </div>
            </>
        }
    }
}
impl Model {
    fn view_nav(&self) -> Html {
        let Self {
            ref link,
            navbar_active,
            ..
        } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <div class="nav">
                <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold"
                    id="mySidenav"><br/>
                    <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%">{"Close Menu"}</a>
                    <div class="w3-container">
                    <h3 class="w3-padding-64"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
                    </div>
                    <AppAnchor classes="w3-padding w3-hover-white" route=AppRoute::HowToConnect4>
                        {"How to Play Connect4"}
                    </AppAnchor>
                    <AppAnchor classes="w3-padding w3-hover-white" route=AppRoute::PlayConnect4WithHuman>
                        {"Play Connect4 With Computer"}
                    </AppAnchor>
                    <a href="#/Connect4Human" class="w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</a>
                    <br/>
                    <AppAnchor classes="w3-padding w3-hover-white" route=AppRoute::HowToToot>
                        {"How to Play TOOT-OTTO"}
                    </AppAnchor>
                    <a href="#/TootOttoComputer" class="w3-padding w3-hover-white">{"Play Toot-Otto With Computer"}</a>
                    <a href="#/TootOttoHuman" class="w3-padding w3-hover-white">{"Play Toot-Otto With Another Human"}</a>
                    <br/>
                    <AppAnchor classes="w3-padding w3-hover-white" route=AppRoute::GameHistory>
                        {"View Game History"}
                    </AppAnchor>
                    <a href="#/Scores" class="w3-padding w3-hover-white">{"Score Board"}</a>
                </nav>

                // <!-- Top menu on small screens -->
                <header class="w3-container w3-top w3-hide-large w3-red w3-xlarge w3-padding">
                    <a href="javascript:void(0)" class="w3-btn w3-red w3-border w3-border-white w3-margin-right">{"&#9776;"}</a>
                    <span>{"Connect 4 with MEAN"}</span>
                </header>

                // <!-- Overlay effect when opening sidenav on small screens -->
                <div class="w3-overlay w3-hide-large" style="cursor:pointer" title="close side menu" id="myOverlay"></div>
            </div>
        }
    }

    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::HowToConnect4 => {
                html! { <HowToConnect4 /> }
            }
            AppRoute::PlayConnect4WithHuman => {
                html! { <PlayConnect4WithHuman/> }
            }
            AppRoute::HowToToot => {
                html! { <HowToToot /> }
            }
            AppRoute::GameHistory => {
                html! { <GameHistory /> }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <PageNotFound route=route /> }
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
