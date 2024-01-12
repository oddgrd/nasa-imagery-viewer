// src/main.rs

// dependencies
use rocket::{Build, Rocket};
use rocket::fs::FileServer;

// function to create rocket instance
fn create() -> Rocket<Build> {
    rocket::build().mount("/", FileServer::from("static"))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = create();

    Ok(rocket.into())
}
