use serde::{Deserialize, Serialize};

// ── daily_sleep ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub(crate) struct ApiSleepContributors {
    pub deep_sleep: Option<i32>,
    pub efficiency: Option<i32>,
    pub latency: Option<i32>,
    pub rem_sleep: Option<i32>,
    pub restfulness: Option<i32>,
    pub timing: Option<i32>,
    pub total_sleep: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiDailySleep {
    pub day: String,
    pub score: Option<i32>,
    pub contributors: ApiSleepContributors,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiDailySleepResponse {
    pub data: Vec<ApiDailySleep>,
    #[allow(dead_code)]
    pub next_token: Option<String>,
}

// ── sleep (sessions) ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub(crate) struct ApiSleep {
    pub id: String,
    pub day: String,
    pub bedtime_start: String,
    pub bedtime_end: String,
    #[serde(rename = "type")]
    pub session_type: String,
    pub total_sleep_duration: Option<i32>,
    pub light_sleep_duration: Option<i32>,
    pub deep_sleep_duration: Option<i32>,
    pub rem_sleep_duration: Option<i32>,
    pub awake_time: Option<i32>,
    pub average_heart_rate: Option<f64>,
    pub average_hrv: Option<f64>,
    pub lowest_heart_rate: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiSleepResponse {
    pub data: Vec<ApiSleep>,
    #[allow(dead_code)]
    pub next_token: Option<String>,
}

// ── sleep_time ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ApiOptimalBedtime {
    pub start_offset: Option<i32>,
    pub end_offset: Option<i32>,
    pub day_tz: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiSleepTime {
    pub day: String,
    pub optimal_bedtime: Option<ApiOptimalBedtime>,
    pub recommendation: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiSleepTimeResponse {
    pub data: Vec<ApiSleepTime>,
    #[allow(dead_code)]
    pub next_token: Option<String>,
}
