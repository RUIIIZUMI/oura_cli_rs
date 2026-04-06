use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use eyre::Result;

use super::client::OuraClient;
use super::models::{ApiDailySleep, ApiSleep, ApiSleepTime};
use crate::ports::sleep::{
    SleepPort, SleepScoreBreakdown, SleepSession, SleepSessionType, SleepSummary, SleepWindow,
};

pub struct OuraSleepAdapter {
    client: Arc<OuraClient>,
}

impl OuraSleepAdapter {
    pub fn new(client: Arc<OuraClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SleepPort for OuraSleepAdapter {
    async fn daily_summary(
        &self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<SleepSummary>> {
        let resp = self
            .client
            .get_daily_sleep(fmt_date(start).as_deref(), fmt_date(end).as_deref())
            .await?;
        Ok(resp.data.into_iter().map(SleepSummary::from).collect())
    }

    async fn sessions(
        &self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<SleepSession>> {
        let resp = self
            .client
            .get_sleep(fmt_date(start).as_deref(), fmt_date(end).as_deref())
            .await?;
        Ok(resp.data.into_iter().map(SleepSession::from).collect())
    }

    async fn sleep_time(
        &self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<SleepWindow>> {
        let resp = self
            .client
            .get_sleep_time(fmt_date(start).as_deref(), fmt_date(end).as_deref())
            .await?;
        Ok(resp.data.into_iter().map(SleepWindow::from).collect())
    }
}

// ── Conversions ───────────────────────────────────────────────────────────────

impl From<ApiDailySleep> for SleepSummary {
    fn from(api: ApiDailySleep) -> Self {
        Self {
            day: parse_date(&api.day),
            score: api.score.map(|s| s.clamp(0, 100) as u8),
            contributors: SleepScoreBreakdown {
                deep_sleep: api.contributors.deep_sleep.map(|v| v.clamp(0, 100) as u8),
                efficiency: api.contributors.efficiency.map(|v| v.clamp(0, 100) as u8),
                latency: api.contributors.latency.map(|v| v.clamp(0, 100) as u8),
                rem_sleep: api.contributors.rem_sleep.map(|v| v.clamp(0, 100) as u8),
                restfulness: api.contributors.restfulness.map(|v| v.clamp(0, 100) as u8),
                timing: api.contributors.timing.map(|v| v.clamp(0, 100) as u8),
                total_sleep: api.contributors.total_sleep.map(|v| v.clamp(0, 100) as u8),
            },
        }
    }
}

impl From<ApiSleep> for SleepSession {
    fn from(api: ApiSleep) -> Self {
        let session_type = match api.session_type.as_str() {
            "long_sleep" => SleepSessionType::LongSleep,
            "short_sleep" => SleepSessionType::ShortSleep,
            "rest" => SleepSessionType::Rest,
            "deleted" => SleepSessionType::Deleted,
            _ => SleepSessionType::Unknown,
        };
        Self {
            id: api.id,
            day: parse_date(&api.day),
            bedtime_start: api.bedtime_start,
            bedtime_end: api.bedtime_end,
            session_type,
            total_sleep_duration: api.total_sleep_duration,
            light_sleep_duration: api.light_sleep_duration,
            deep_sleep_duration: api.deep_sleep_duration,
            rem_sleep_duration: api.rem_sleep_duration,
            awake_time: api.awake_time,
            avg_heart_rate: api.average_heart_rate,
            avg_hrv: api.average_hrv,
            lowest_heart_rate: api.lowest_heart_rate,
        }
    }
}

impl From<ApiSleepTime> for SleepWindow {
    fn from(api: ApiSleepTime) -> Self {
        Self {
            day: parse_date(&api.day),
            optimal_start_offset: api.optimal_bedtime.as_ref().and_then(|b| b.start_offset),
            optimal_end_offset: api.optimal_bedtime.as_ref().and_then(|b| b.end_offset),
            recommendation: api.recommendation,
            status: api.status,
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn fmt_date(d: Option<NaiveDate>) -> Option<String> {
    d.map(|d| d.format("%Y-%m-%d").to_string())
}

fn parse_date(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
}
