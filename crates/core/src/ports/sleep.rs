use async_trait::async_trait;
use chrono::NaiveDate;
use eyre::Result;
use serde::{Deserialize, Serialize};

// ── Domain models ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepSummary {
    pub day: NaiveDate,
    pub score: Option<u8>,
    pub contributors: SleepScoreBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepScoreBreakdown {
    pub deep_sleep: Option<u8>,
    pub efficiency: Option<u8>,
    pub latency: Option<u8>,
    pub rem_sleep: Option<u8>,
    pub restfulness: Option<u8>,
    pub timing: Option<u8>,
    pub total_sleep: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepSession {
    pub id: String,
    pub day: NaiveDate,
    pub bedtime_start: String,
    pub bedtime_end: String,
    pub session_type: SleepSessionType,
    pub total_sleep_duration: Option<i32>,
    pub light_sleep_duration: Option<i32>,
    pub deep_sleep_duration: Option<i32>,
    pub rem_sleep_duration: Option<i32>,
    pub awake_time: Option<i32>,
    pub avg_heart_rate: Option<f64>,
    pub avg_hrv: Option<f64>,
    pub lowest_heart_rate: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepSessionType {
    LongSleep,
    ShortSleep,
    Rest,
    Deleted,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepWindow {
    pub day: NaiveDate,
    pub optimal_start_offset: Option<i32>,
    pub optimal_end_offset: Option<i32>,
    pub recommendation: Option<String>,
    pub status: Option<String>,
}

// ── Port trait ────────────────────────────────────────────────────────────────

#[async_trait]
pub trait SleepPort: Send + Sync {
    async fn daily_summary(
        &self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<SleepSummary>>;

    async fn sessions(
        &self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<SleepSession>>;

    async fn sleep_time(
        &self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<SleepWindow>>;
}
