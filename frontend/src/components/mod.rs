// src/components/app.rs

// dependencies
use crate::components::content::Content;
use crate::components::footer::Footer;
use crate::components::header::Header;
use chrono::{Datelike, Local};
use yew::{classes, function_component, html, Html};

// bring modules into scope
mod content;
mod footer;
mod header;

// the main app component, passes year prop to the footer
#[function_component]
pub fn App() -> Html {
    let year = Local::now().year();
    html! {
       <div class={classes!("container")}>
           <Header />
           <br />
           <Content />
           <Footer year={year} />
       </div>
    }
}
