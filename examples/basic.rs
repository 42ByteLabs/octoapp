use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    // dotenvy::dotenv()?;

    // Load the configuration (from environment variables)
    let config = octoapp::OctoAppConfig::init().app_id(12345).build()?;
    println!("{}", config);

    let octocrab = config.octocrab();
    if let Ok(client) = octocrab {
        client.installation(octocrab::models::InstallationId(config.app_id() as u64));
        println!("{:?}", client);

        let org = client.orgs("42ByteLabs").get().await?;
        println!("{:?}", org);

        let repos = client
            .issues("42ByteLabs", "octoapp")
            .list()
            .creator("GeekMasher")
            .send()
            .await?;

        println!("Reposiotories: {}", repos.total_count.unwrap_or_default());
    }

    Ok(())
}
