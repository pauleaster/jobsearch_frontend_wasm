
// src/main.rs
use yew::Renderer;

mod components;
mod utils;
use crate::app::App;

mod app;

fn main() {
    Renderer::<App>::new().render();
}