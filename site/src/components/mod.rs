// src/components/app.rs

// dependencies
use crate::components::content::Content;
use crate::components::footer::Footer;
use crate::components::header::Header;
use yew::{classes, function_component, html, Html};

// bring modules into scope
mod content;
mod footer;
mod header;

// the main app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
       <div class={classes!("container")}>
           <Header />
           <br />
           <Content />
           <Footer />
       </div>
    }
}
