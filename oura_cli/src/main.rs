use clap::{Parser, Subcommand};
use eyre::{bail, Result};
use oura_core::client::OuraClient;
use oura_core::models::{
    DailyActivity, DailyReadiness, DailyStress, DailyStressSummary, HeartRate,
};

#[derive(Parser)]
#[command(name = "oura", about = "Oura API CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

    let token = match std::env::var("OURA_RING_API_KEY") {
        Ok(t) => t,
        Err(_) => bail!("OURA_RING_API_KEY environment variable is not set"),
    };

    let client = OuraClient::new(token);

    match cli.command {
        Commands::Sleep { start_date, end_date, json } => {
            let resp = client
                .get_daily_sleep(start_date.as_deref(), end_date.as_deref())
                .await?;
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No sleep data found.");
                return Ok(());
            }
            print_score_chart("Sleep Score", resp.data.iter().map(|e| (e.day.get(5..).unwrap_or(&e.day).to_string(), e.score.unwrap_or(0))));
        }

        Commands::Activity { start_date, end_date, json } => {
            let resp = client
                .get_daily_activity(start_date.as_deref(), end_date.as_deref())
                .await?;
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No activity data found.");
                return Ok(());
            }
            print_score_chart("Activity Score", resp.data.iter().map(|e| (e.day.get(5..).unwrap_or(&e.day).to_string(), e.score.unwrap_or(0))));
            print_activity_extras(&resp.data);
        }

        Commands::Readiness { start_date, end_date, json } => {
            let resp = client
                .get_daily_readiness(start_date.as_deref(), end_date.as_deref())
                .await?;
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No readiness data found.");
                return Ok(());
            }
            print_score_chart("Readiness Score", resp.data.iter().map(|e| (e.day.get(5..).unwrap_or(&e.day).to_string(), e.score.unwrap_or(0))));
            print_readiness_extras(&resp.data);
        }

        Commands::Stress { start_date, end_date, json } => {
            let resp = client
                .get_daily_stress(start_date.as_deref(), end_date.as_deref())
                .await?;
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No stress data found.");
                return Ok(());
            }
            print_stress_table(&resp.data);
        }

        Commands::Heartrate { start_date, end_date, json } => {
            let resp = client
                .get_heartrate(start_date.as_deref(), end_date.as_deref())
                .await?;
            if json {
                println!("{}", serde_json::to_string_pretty(&resp)?);
                return Ok(());
            }
            if resp.data.is_empty() {
                println!("No heart rate data found.");
                return Ok(());
            }
            print_heartrate_table(&resp.data);
        }
    }

    Ok(())
}

fn print_score_chart(title: &str, entries: impl Iterator<Item = (String, i32)>) {
    println!("{title}");
    println!("{}", "─".repeat(54));
    for (day, score) in entries {
        let bar_len = (score as usize / 2).min(50);
        let bar = "█".repeat(bar_len);
        let color = match score {
            80.. => "\x1b[32m",
            60.. => "\x1b[33m",
            _ => "\x1b[31m",
        };
        println!(
            "{day} │ {color}{bar}{}\x1b[0m {score}",
            " ".repeat(50 - bar_len),
        );
    }
    println!("{}", "─".repeat(54));
}

fn print_activity_extras(entries: &[DailyActivity]) {
    println!();
    println!("{:<8} {:>8} {:>8}", "Date", "Steps", "Cal");
    println!("{}", "─".repeat(28));
    for e in entries {
        println!("{:<8} {:>8} {:>8}", e.day.get(5..).unwrap_or(&e.day), e.steps, e.active_calories);
    }
}

fn print_readiness_extras(entries: &[DailyReadiness]) {
    println!();
    println!("{:<8} {:>12} {:>12}", "Date", "Temp Dev", "HRV Bal");
    println!("{}", "─".repeat(36));
    for e in entries {
        let temp = e.temperature_deviation.map_or("-".to_string(), |t| format!("{t:+.2}°C"));
        let hrv = e.contributors.hrv_balance.map_or("-".to_string(), |h| h.to_string());
        println!("{:<8} {:>12} {:>12}", e.day.get(5..).unwrap_or(&e.day), temp, hrv);
    }
}

fn print_stress_table(entries: &[DailyStress]) {
    println!("Stress Level");
    println!("{}", "─".repeat(54));
    for e in entries {
        let summary = match &e.day_summary {
            Some(DailyStressSummary::Restored) => "\x1b[32mrestored\x1b[0m",
            Some(DailyStressSummary::Normal) => "\x1b[33mnormal\x1b[0m",
            Some(DailyStressSummary::Stressful) => "\x1b[31mstressful\x1b[0m",
            Some(DailyStressSummary::Unknown) | None => "-",
        };
        let stress_min = e.stress_high.unwrap_or(0) / 60;
        let recovery_min = e.recovery_high.unwrap_or(0) / 60;
        println!(
            "{} │ {}  stress={}m  recovery={}m",
            e.day.get(5..).unwrap_or(&e.day), summary, stress_min, recovery_min
        );
    }
    println!("{}", "─".repeat(54));
}

fn print_heartrate_table(entries: &[HeartRate]) {
    println!("Heart Rate (bpm)");
    println!("{}", "─".repeat(54));
    // Show at most last 20 entries to avoid flooding the terminal
    let display = if entries.len() > 20 { &entries[entries.len() - 20..] } else { entries };
    for e in display {
        // timestamp is "2026-04-05T02:00:00" — show date+time portion
        let ts = e.timestamp.get(..16).unwrap_or(&e.timestamp);
        println!("{ts} │ {} bpm", e.bpm);
    }
    if entries.len() > 20 {
        println!("  (showing last 20 of {} samples)", entries.len());
    }
    println!("{}", "─".repeat(54));
}
