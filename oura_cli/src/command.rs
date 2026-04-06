use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use eyre::{Result, bail, eyre};
use oura_core::OuraClient;
use oura_core::ports::sleep::SleepPort;

use crate::Commands;
use crate::display::{Display, OutputMode};

#[derive(Debug)]
pub struct DateRange {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

impl DateRange {
    pub fn parse(start: Option<String>, end: Option<String>) -> Result<Self> {
        let fmt = "%Y-%m-%d";
        let start = start
            .map(|s| {
                NaiveDate::parse_from_str(&s, fmt)
                    .map_err(|_| eyre!("invalid start_date format '{s}', expected YYYY-MM-DD"))
            })
            .transpose()?;
        let end = end
            .map(|s| {
                NaiveDate::parse_from_str(&s, fmt)
                    .map_err(|_| eyre!("invalid end_date format '{s}', expected YYYY-MM-DD"))
            })
            .transpose()?;
        if let (Some(s), Some(e)) = (start, end)
            && s > e
        {
            bail!("start_date {s} is after end_date {e}");
        }
        Ok(Self { start, end })
    }
}

#[async_trait]
pub trait Execute: Send + Sync {
    async fn execute(&self) -> Result<()>;
}

// ── Commands ──────────────────────────────────────────────────────────────────

pub struct SleepCommand {
    pub date_range: DateRange,
    pub display: Display,
    pub adapter: Arc<dyn SleepPort>,
}

pub struct ActivityCommand {
    pub date_range: DateRange,
    pub display: Display,
    pub client: Arc<OuraClient>,
}

pub struct ReadinessCommand {
    pub date_range: DateRange,
    pub display: Display,
    pub client: Arc<OuraClient>,
}

pub struct StressCommand {
    pub date_range: DateRange,
    pub display: Display,
    pub client: Arc<OuraClient>,
}

pub struct HeartrateCommand {
    pub date_range: DateRange,
    pub display: Display,
    pub client: Arc<OuraClient>,
}

// ── Execute impls ─────────────────────────────────────────────────────────────

#[async_trait]
impl Execute for SleepCommand {
    async fn execute(&self) -> Result<()> {
        tracing::info!(command = "sleep", "executing command");
        let summaries = self
            .adapter
            .daily_summary(self.date_range.start, self.date_range.end)
            .await?;
        if summaries.is_empty() {
            println!("No sleep data found.");
        } else {
            tracing::info!(count = summaries.len(), "fetched records");
            self.display.show(&summaries);
        }
        Ok(())
    }
}

#[async_trait]
impl Execute for ActivityCommand {
    async fn execute(&self) -> Result<()> {
        tracing::info!(command = "activity", "executing command");
        let start = self
            .date_range
            .start
            .map(|d| d.format("%Y-%m-%d").to_string());
        let end = self
            .date_range
            .end
            .map(|d| d.format("%Y-%m-%d").to_string());
        match self
            .client
            .get_daily_activity(start.as_deref(), end.as_deref())
            .await?
        {
            None => println!("No activity data found."),
            Some(resp) => {
                tracing::info!(count = resp.data.len(), "fetched records");
                self.display.show(&resp);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Execute for ReadinessCommand {
    async fn execute(&self) -> Result<()> {
        tracing::info!(command = "readiness", "executing command");
        let start = self
            .date_range
            .start
            .map(|d| d.format("%Y-%m-%d").to_string());
        let end = self
            .date_range
            .end
            .map(|d| d.format("%Y-%m-%d").to_string());
        match self
            .client
            .get_daily_readiness(start.as_deref(), end.as_deref())
            .await?
        {
            None => println!("No readiness data found."),
            Some(resp) => {
                tracing::info!(count = resp.data.len(), "fetched records");
                self.display.show(&resp);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Execute for StressCommand {
    async fn execute(&self) -> Result<()> {
        tracing::info!(command = "stress", "executing command");
        let start = self
            .date_range
            .start
            .map(|d| d.format("%Y-%m-%d").to_string());
        let end = self
            .date_range
            .end
            .map(|d| d.format("%Y-%m-%d").to_string());
        match self
            .client
            .get_daily_stress(start.as_deref(), end.as_deref())
            .await?
        {
            None => println!("No stress data found."),
            Some(resp) => {
                tracing::info!(count = resp.data.len(), "fetched records");
                self.display.show(&resp);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Execute for HeartrateCommand {
    async fn execute(&self) -> Result<()> {
        tracing::info!(command = "heartrate", "executing command");
        let start = self
            .date_range
            .start
            .map(|d| d.format("%Y-%m-%d").to_string());
        let end = self
            .date_range
            .end
            .map(|d| d.format("%Y-%m-%d").to_string());
        match self
            .client
            .get_heartrate(start.as_deref(), end.as_deref())
            .await?
        {
            None => println!("No heart rate data found."),
            Some(resp) => {
                tracing::info!(count = resp.data.len(), "fetched records");
                self.display.show(&resp);
            }
        }
        Ok(())
    }
}

// ── Factory ───────────────────────────────────────────────────────────────────

pub fn from_cli(
    cmd: Commands,
    client: Arc<OuraClient>,
    sleep: Arc<dyn SleepPort>,
) -> Result<Box<dyn Execute>> {
    match cmd {
        Commands::Sleep {
            start_date,
            end_date,
            json,
        } => Ok(Box::new(SleepCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display {
                mode: if json {
                    OutputMode::Json
                } else {
                    OutputMode::Pretty
                },
            },
            adapter: sleep,
        })),
        Commands::Activity {
            start_date,
            end_date,
            json,
        } => Ok(Box::new(ActivityCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display {
                mode: if json {
                    OutputMode::Json
                } else {
                    OutputMode::Pretty
                },
            },
            client,
        })),
        Commands::Readiness {
            start_date,
            end_date,
            json,
        } => Ok(Box::new(ReadinessCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display {
                mode: if json {
                    OutputMode::Json
                } else {
                    OutputMode::Pretty
                },
            },
            client,
        })),
        Commands::Stress {
            start_date,
            end_date,
            json,
        } => Ok(Box::new(StressCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display {
                mode: if json {
                    OutputMode::Json
                } else {
                    OutputMode::Pretty
                },
            },
            client,
        })),
        Commands::Heartrate {
            start_date,
            end_date,
            json,
        } => Ok(Box::new(HeartrateCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display {
                mode: if json {
                    OutputMode::Json
                } else {
                    OutputMode::Pretty
                },
            },
            client,
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_dates() {
        let dr = DateRange::parse(Some("2026-04-01".into()), Some("2026-04-07".into())).unwrap();
        assert_eq!(dr.start.unwrap().to_string(), "2026-04-01");
        assert_eq!(dr.end.unwrap().to_string(), "2026-04-07");
    }

    #[test]
    fn parse_none_dates() {
        let dr = DateRange::parse(None, None).unwrap();
        assert!(dr.start.is_none());
        assert!(dr.end.is_none());
    }

    #[test]
    fn parse_invalid_format_returns_err() {
        let result = DateRange::parse(Some("04-01-2026".into()), None);
        assert!(result.is_err());
    }

    #[test]
    fn parse_start_after_end_returns_err() {
        let result = DateRange::parse(Some("2026-04-10".into()), Some("2026-04-01".into()));
        assert!(result.is_err());
    }

    #[test]
    fn parse_same_start_and_end_is_ok() {
        let result = DateRange::parse(Some("2026-04-05".into()), Some("2026-04-05".into()));
        assert!(result.is_ok());
    }

    #[test]
    fn date_range_returns_err_on_bad_start_date() {
        let result = DateRange::parse(Some("not-a-date".into()), None);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid start_date")
        );
    }
}
