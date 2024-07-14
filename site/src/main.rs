// src/main.rs

// dependencies
use crate::components::App;

// bring modules into scope
mod components;
mod domain;

// main function to render the site
fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
