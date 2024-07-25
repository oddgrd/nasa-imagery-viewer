// src/lib/app.rs

// dependencies
use crate::views::Root;
use yew::{function_component, html, Html};

// the main app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Root />
        </div>
    }
}