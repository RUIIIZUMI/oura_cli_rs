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

    match cli.command {
        Commands::Sleep {
            start_date,
            end_date,
            json,
        } => {
            tracing::info!(command = "sleep", "executing command");
            let resp = client
                .get_daily_sleep(start_date.as_deref(), end_date.as_deref())
                .await?;
            tracing::info!(count = resp.data.len(), "fetched records");
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No sleep data found.");
                return Ok(());
            }
            display::print_score_chart(
                "Sleep Score",
                resp.data.iter().map(|e| {
                    (
                        e.day.get(5..).unwrap_or(&e.day).to_string(),
                        e.score.unwrap_or(0),
                    )
                }),
            );
        }

        Commands::Activity {
            start_date,
            end_date,
            json,
        } => {
            tracing::info!(command = "activity", "executing command");
            let resp = client
                .get_daily_activity(start_date.as_deref(), end_date.as_deref())
                .await?;
            tracing::info!(count = resp.data.len(), "fetched records");
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No activity data found.");
                return Ok(());
            }
            display::print_score_chart(
                "Activity Score",
                resp.data.iter().map(|e| {
                    (
                        e.day.get(5..).unwrap_or(&e.day).to_string(),
                        e.score.unwrap_or(0),
                    )
                }),
            );
            display::print_activity_extras(&resp.data);
        }

        Commands::Readiness {
            start_date,
            end_date,
            json,
        } => {
            tracing::info!(command = "readiness", "executing command");
            let resp = client
                .get_daily_readiness(start_date.as_deref(), end_date.as_deref())
                .await?;
            tracing::info!(count = resp.data.len(), "fetched records");
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No readiness data found.");
                return Ok(());
            }
            display::print_score_chart(
                "Readiness Score",
                resp.data.iter().map(|e| {
                    (
                        e.day.get(5..).unwrap_or(&e.day).to_string(),
                        e.score.unwrap_or(0),
                    )
                }),
            );
            display::print_readiness_extras(&resp.data);
        }

        Commands::Stress {
            start_date,
            end_date,
            json,
        } => {
            tracing::info!(command = "stress", "executing command");
            let resp = client
                .get_daily_stress(start_date.as_deref(), end_date.as_deref())
                .await?;
            tracing::info!(count = resp.data.len(), "fetched records");
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No stress data found.");
                return Ok(());
            }
            display::print_stress_table(&resp.data);
        }

        Commands::Heartrate {
            start_date,
            end_date,
            json,
        } => {
            tracing::info!(command = "heartrate", "executing command");
            let resp = client
                .get_heartrate(start_date.as_deref(), end_date.as_deref())
                .await?;
            tracing::info!(count = resp.data.len(), "fetched records");
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No heart rate data found.");
                return Ok(());
            }
            display::print_heartrate_table(&resp.data);
        }
    }

    Ok(())
}
