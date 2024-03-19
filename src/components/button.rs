// src/components/button.rs

use yew::prelude::*;
use crate::utils::click_message::format_click_message;


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    // This callback now sends an i32 representing the click count
    pub onclick: Callback<i32>,
}
// struct for holding number of button clicks
pub struct Button {
    clicks: i32,
}


pub enum Msg {
    Clicked,
}

impl Component for Button {
    type Message = Msg;
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            clicks: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                self.clicks += 1;
                ctx.props().onclick.emit(self.clicks);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Before click, show "Click here"
        // after one click show "Clicked 1 time"
        // after subsequet clicks show "Clicked Self.clicks times"
        // store the labe in label variable
        let label: String = format_click_message(self.clicks);

        html! {
            <button onclick={ctx.link().callback(|_| Msg::Clicked)}>
                {label}
            </button>
        }
    }
}
