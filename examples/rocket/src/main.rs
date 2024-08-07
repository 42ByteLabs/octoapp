#[macro_use]
extern crate rocket;

use anyhow::Result;
use rocket::{http::Status, State};

// Import the prelude module to get all the necessary imports
use octoapp::prelude::*;

/// The webhook route for GitHub events
///
/// `WebHook` acts similar to `Json<T>` in Rocket, but it deserializes the
/// incoming JSON payload into a `WebHook<T>` instance.
///
/// You can either use `WebHook<Event>` to get all events, or use a specific
/// event type like `WebHook<PingEvent>` to only get ping events.
#[post("/", data = "<event>")]
async fn webhook(state: &State<OctoAppState>, event: WebHook<Event>) -> (Status, String) {
    // Get the Octocrab instance from the state
    let _octocrab = state.config.octocrab();

    match event.into_inner() {
        // Handle the ping event (just return "pong")
        Event::Ping(_) => {
            println!("Received ping event");
            (Status::Ok, "pong".to_string())
        }
        // Handle the push event
        //
        // https://docs.github.com/en/webhooks/webhook-events-and-payloads#push
        Event::Push(push) => {
            println!("Received push event: {:?}", push);
            (Status::Ok, "Received push event".to_string())
        }
        _ => {
            println!("Received an unknown event");
            (Status::BadRequest, "Received an unknown event".to_string())
        }
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv()?;

    // Load the configuration (from environment variables)
    let config = OctoAppConfig::init().build()?;
    println!("{}", config);

    // Create the application state (OctoAppState).
    // This is to manage the configuration and other shared state.
    let octostate = OctoAppState::new(config);

    // Build the Rocket instance
    let rocket = rocket::build()
        // Attach the OctoAppState to the Rocket instance
        .manage(octostate)
        // Mount the /github route
        .mount("/github", routes![webhook]);
    // Launch the Rocket instance
    rocket.launch().await?;

    Ok(())
}
