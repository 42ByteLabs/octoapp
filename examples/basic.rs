use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv()?;

    // Load the configuration (from environment variables)
    let config = octoapp::OctoAppConfig::init().build()?;
    println!("{}", config);

    let octocrab = config.octocrab()?;
    octocrab.installation(octocrab::models::InstallationId(config.app_id() as u64));

    println!("{:?}", octocrab);

    let org = octocrab.orgs("42ByteLabs").get().await?;
    println!("{:?}", org);

    Ok(())
}
