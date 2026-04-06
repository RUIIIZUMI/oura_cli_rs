mod command;
mod display;
mod util;

use clap::{Parser, Subcommand};
use eyre::{Result, bail};
use oura_core::client::OuraClient;

#[derive(Parser)]
#[command(name = "oura", about = "Oura API CLI")]
struct Cli {
    /// Enable debug logging
    #[arg(long, global = true)]
    debug: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch daily sleep score
    Sleep {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch daily activity score
    Activity {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch daily readiness score
    Readiness {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch daily stress level
    Stress {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch heart rate samples
    Heartrate {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    util::init_tracing(cli.debug);

    tracing::info!("oura cli started");

    let token = match std::env::var("OURA_RING_API_KEY") {
        Ok(t) => t,
        Err(_) => bail!("OURA_RING_API_KEY environment variable is not set"),
    };

    let client = OuraClient::new(token);

    let cmd = command::from_cli(cli.command)?;
    cmd.execute(&client).await
}
