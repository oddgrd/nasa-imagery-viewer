// src/main.rs

// dependencies
use crate::components::App;

// bring modules into scope
mod components;
mod domain;

// main function to render the site
fn main() {
    yew::Renderer::<App>::new().render();
}
