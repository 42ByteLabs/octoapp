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
    let octo = event.octocrab(state).await.unwrap();
    tracing::info!("Octocrab instance: {:?}", octo);

    match event.into_inner() {
        Event::Issues(issues) => {
            tracing::info!("Received an issue event: {:?}", issues.issue.id);

            // Comment on the issue
            octo.issues("42ByteLabs", "octoapp")
                .create_comment(issues.issue.number, "Hello from OctoApp!")
                .await
                .unwrap();

            (Status::Ok, "Received an issue event".to_string())
        }
        _ => {
            tracing::warn!("Received an unknown event");
            (Status::Ok, "Received an unknown event".to_string())
        }
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv()?;

    // Load the configuration (from environment variables)
    let mut config = OctoAppConfig::init().build()?;
    // Install the configuration and fetch all the installations
    // of the GitHub App (if any).
    config.install().await?;

    // This will create an Octocrab instance with the required authentication
    // information. Note, this is done after `.install()` so if an installation
    // is found, the Octocrab instance will be created with the installation token.
    let client = config.octocrab()?;

    let repos = client
        .orgs("42ByteLabs")
        .list_repos()
        .send()
        .await?
        .take_items();
    tracing::info!("Monitoring Repository Count :: {:?}", repos.len());

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
