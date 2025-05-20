use anyhow::Result;
use clap::Parser;
use std::env;

mod app;
mod pubsub;
mod ui;

use app::App;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Project ID to filter resources
    #[clap(short, long)]
    project_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let project_id = if let Some(ref id) = cli.project_id {
        id
    } else {
        return Err(anyhow::anyhow!("Project ID is required"));
    };

    env::set_var("PUBSUB_EMULATOR_HOST", "localhost:8065");

    let pubsub_client = pubsub::PubSubClient::new(&project_id).await?;

    let mut app = App::new(project_id, pubsub_client)?;
    app.run().await?;

    for log in &app.debug_logs {
        println!("{}", log);
    }

    Ok(())
}
