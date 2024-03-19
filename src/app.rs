// src/app.rs

use yew::prelude::*;

use crate::components::Button;
use crate::utils::click_message::format_click_message;

#[function_component(App)]
pub fn app() -> Html {

    let clicks = use_state(|| 0);

    let onclick = {
        let clicks = clicks.clone();
        Callback::from(move |click_count: i32| {
            clicks.set(click_count);
        })
    };

    let clicks_label = format_click_message(*clicks);
    html! {
        <>
        <div>
            <h1>{"Yew Application"}</h1>
            <Button {onclick} />
            <div>{format!("{}", clicks_label)}</div>
        </div>
        </>
    }
}
