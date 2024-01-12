// footer.rs

// dependencies
use crate::domain::Props;
use yew::{function_component, html, Html};

// the footer component
#[function_component]
pub fn Footer(props: &Props) -> Html {
    html! {
        <footer>
            <section>
                <br />
                <p>{ "\u{00A9} " } {props.year} { " Jeffery D Mitchell | All Rights Reserved | Site created in WebAssembly with " }<a href="https://yew.rs"> { "Yew" }</a></p>
                <p> {" Site hosting courtesy of: " }<a href="https://shuttle.rs">{ "shuttle.rs" }</a></p>
            </section>
        </footer>
    }
}
