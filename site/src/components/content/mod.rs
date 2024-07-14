// content.rs

// dependencies
use crate::domain::NASAData;
use gloo_net::http::Request;
use url::Url;
use yew::{classes, function_component, html, use_effect_with, use_state, Html};

const NASA_API_KEY: &str = "lsULnkmChaJlS3fZO85M3cnGA8TFCAm2peEfd9QS";

// the content component, renders select data returned from the NASA APOD API
#[function_component]
pub fn Content() -> Html {
    let key = NASA_API_KEY;
    let api_key = ["apod?api_key=", key].concat();
    let api_url = Url::parse("https://api.nasa.gov/planetary/").expect("Failed to parse the url to fetch data from.");
    let api_url = api_url.join(&api_key).expect("Failed to join URL");
    let empty_data = NASAData::default();
    let fetched_data = use_state(|| empty_data);
    {
        let fetched_data = fetched_data.clone();
        use_effect_with((), move |_| {
            let fetched_data = fetched_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_nasa_data: NASAData = Request::get(api_url.as_ref())
                    .send()
                    .await
                    .expect("Unable to fetch data from NASA API")
                    .json()
                    .await
                    .expect("Data received from API is not valid.");
                fetched_data.set(fetched_nasa_data);
            });
            || ()
        });
    }

    html! {
        <main>
            <section>
                <h3>{ "Date: " } {&fetched_data.date}</h3>
                <h3>{ "Title: " } {&fetched_data.title}</h3>
                <h3>{ "Explanation: " } </h3>
                <p> {&fetched_data.explanation} </p>
                if &fetched_data.media_type == "image"  {
                    if let Some(hdurl) = fetched_data.hdurl.clone() {
                        <h3>{ "Image: " }</h3>
                        <img src={hdurl} class={classes!("img-fluid")} alt={"NASA Astronomy Photo of the Day "} />
                    } else {
                        <h3>{ "Image: " }</h3>
                        <img src={fetched_data.url.clone()} class={classes!("img-fluid")} alt={"NASA Astronomy Photo of the Day "} />
                    }
                } else {
                    <h3>{ "Video: "}</h3>
                    <iframe width="960" height="540" src={fetched_data.url.clone()}></iframe>
                }
                if let Some(copyright) = &fetched_data.copyright {
                    <h3>{ "Image by: "} {&copyright}</h3>
                } else {
                    <p>{ "Today's image or video has no attributed copyright data. Copyright may embedded in a watermark."}</p>
                }
            </section>
        </main>
    }
}
