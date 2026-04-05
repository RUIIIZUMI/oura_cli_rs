use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SleepContributors {
    pub deep_sleep: Option<i32>,
    pub efficiency: Option<i32>,
    pub latency: Option<i32>,
    pub rem_sleep: Option<i32>,
    pub restfulness: Option<i32>,
    pub timing: Option<i32>,
    pub total_sleep: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct DailySleep {
    pub id: String,
    pub day: String,
    pub score: Option<i32>,
    pub contributors: SleepContributors,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct DailySleepResponse {
    pub data: Vec<DailySleep>,
    pub next_token: Option<String>,
}
