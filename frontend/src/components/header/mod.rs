// src/components/header.rs

// dependencies
use yew::{function_component, html, Html};

// the site header component
#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
            <h1> { "NASA Imagery Viewer" }</h1>
            <h2> { "...a new photo or video every day"}</h2>
        </header>
    }
}
