use anyhow::Result;
use octoapp::{prelude::*, HyperWebhookHandler, OctoAppConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

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
    println!("Monitoring Repository Count :: {:?}", repos.len());

    // Create the webhook handler
    let handler = HyperWebhookHandler::new(config)
        .path("/github")
        .on_event(|webhook: WebHook<Event>| async move {
            println!("Received webhook event from installation {}", webhook.installation());
            
            match webhook.into_inner() {
                Event::Ping(ping) => {
                    println!("Received ping event: {:?}", ping.hook_id);
                    Ok(())
                }
                Event::Issues(issues) => {
                    println!("Received issue event: {:?}", issues.issue.id);
                    // Here you would typically use the octocrab client
                    // to interact with the GitHub API
                    Ok(())
                }
                Event::PullRequest(pr) => {
                    println!("Received pull request event: {:?}", pr.pull_request.id);
                    Ok(())
                }
                _ => {
                    println!("Received other event type");
                    Ok(())
                }
            }
        });

    println!("Starting hyper webhook server on http://127.0.0.1:4242/github");
    
    // Start the server
    handler.serve("127.0.0.1:4242").await?;

    Ok(())
}
