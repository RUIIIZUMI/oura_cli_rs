use clap::{Parser, Subcommand};
use eyre::{bail, Result};
use oura_core::client::OuraClient;

#[derive(Parser)]
#[command(name = "oura", about = "Oura API CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch daily sleep data
    Sleep {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let token = match std::env::var("OURA_RING_API_KEY") {
        Ok(t) => t,
        Err(_) => bail!("OURA_RING_API_KEY environment variable is not set"),
    };

    let client = OuraClient::new(token);

    match cli.command {
        Commands::Sleep {
            start_date,
            end_date,
        } => {
            let resp = client
                .get_daily_sleep(start_date.as_deref(), end_date.as_deref())
                .await?;

            if resp.data.is_empty() {
                println!("No sleep data found for the given range.");
                return Ok(());
            }

            println!("Sleep Score");
            println!("{}", "─".repeat(50));
            for entry in &resp.data {
                let score = entry.score.unwrap_or(0);
                let bar_len = score as usize / 2;
                let bar = "█".repeat(bar_len);
                let color = match score {
                    80.. => "\x1b[32m",
                    60.. => "\x1b[33m",
                    _ => "\x1b[31m",
                };
                println!(
                    "{} │ {}{}{}\x1b[0m {}",
                    &entry.day[5..],
                    color,
                    bar,
                    " ".repeat(50 - bar_len),
                    score,
                );
            }
            println!("{}", "─".repeat(50));
        }
    }

    Ok(())
}
