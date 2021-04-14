// code skeleton from https://github.com/yewstack/yew/blob/master/examples/store/src/text_input.rs
// code modified to suit the Connect4 and TOOT and OTTO project 
use yew::prelude::*;

pub enum Msg {
    SetText(String)
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: String,
    pub oninput: Callback<String>,
    pub disabled: bool,
}

pub struct TextInput {
    link: ComponentLink<Self>,
    pub text: String,
    props: Props,
}

impl Component for TextInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: props.value.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetText(text) => {
                self.text = text.clone();
                self.props.oninput.emit(text.clone());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.text = self.props.value.clone();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="text"
                value=&self.text
                oninput=self.link.callback(|e: InputData| Msg::SetText(e.value))
                disabled = self.props.disabled
            />
        }
    }
}