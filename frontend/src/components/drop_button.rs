use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct DropButton {
    link: ComponentLink<DropButton>,
    fetch_task: Option<FetchTask>,
}

pub enum Message {
    PostDrop,
    PostSuccess,
    PostFailed,
}

impl Component for DropButton {
    type Message = Message;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::PostDrop => {
                // create callback for POST request to backend
                let callback = self.link.callback(move |response: Response<Result<String, Error>>| {
                    if response.status().is_success() {
                        Message::PostSuccess
                    } else {
                        Message::PostFailed
                    }
                });

                // create the POST request
                let request = Request::delete("http://127.0.0.1:8000/history")
                    .body(Nothing)
                    .unwrap();

                // send the POST request
                self.fetch_task = FetchService::fetch(request, callback).ok();
            },
            Message::PostSuccess => log::info!("drop collection successful"),
            Message::PostFailed => log::info!("failed to drop collection"),
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Message::PostDrop)>{"Clear Game History"}</button>
        }
    }
}
