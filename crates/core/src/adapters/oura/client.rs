use eyre::Result;
use reqwest::Client;

use super::models::{ApiDailySleepResponse, ApiSleepResponse, ApiSleepTimeResponse};
use crate::models::{
    DailyActivityResponse, DailyReadinessResponse, DailyStressResponse, HeartRateResponse,
};

const BASE_URL: &str = "https://api.ouraring.com";

pub struct OuraClient {
    client: Client,
    token: String,
}

impl OuraClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    // ── Sleep ─────────────────────────────────────────────────────────────────

    pub(crate) async fn get_daily_sleep(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<ApiDailySleepResponse> {
        let url = build_url(
            "daily_sleep",
            start_date,
            end_date,
            "start_date",
            "end_date",
        );
        self.get(&url).await
    }

    pub(crate) async fn get_sleep(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<ApiSleepResponse> {
        let url = build_url("sleep", start_date, end_date, "start_date", "end_date");
        self.get(&url).await
    }

    pub(crate) async fn get_sleep_time(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<ApiSleepTimeResponse> {
        let url = build_url("sleep_time", start_date, end_date, "start_date", "end_date");
        self.get(&url).await
    }

    // ── Non-sleep (still used directly by CLI commands) ───────────────────────

    pub async fn get_daily_activity(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Option<DailyActivityResponse>> {
        let url = build_url(
            "daily_activity",
            start_date,
            end_date,
            "start_date",
            "end_date",
        );
        let resp: DailyActivityResponse = self.get(&url).await?;
        Ok(if resp.data.is_empty() {
            None
        } else {
            Some(resp)
        })
    }

    pub async fn get_daily_readiness(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Option<DailyReadinessResponse>> {
        let url = build_url(
            "daily_readiness",
            start_date,
            end_date,
            "start_date",
            "end_date",
        );
        let resp: DailyReadinessResponse = self.get(&url).await?;
        Ok(if resp.data.is_empty() {
            None
        } else {
            Some(resp)
        })
    }

    pub async fn get_daily_stress(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Option<DailyStressResponse>> {
        let url = build_url(
            "daily_stress",
            start_date,
            end_date,
            "start_date",
            "end_date",
        );
        let resp: DailyStressResponse = self.get(&url).await?;
        Ok(if resp.data.is_empty() {
            None
        } else {
            Some(resp)
        })
    }

    pub async fn get_heartrate(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Option<HeartRateResponse>> {
        let start_dt = start_date.map(|d| {
            if d.contains('T') {
                d.to_string()
            } else {
                format!("{d}T00:00:00")
            }
        });
        let end_dt = end_date.map(|d| {
            if d.contains('T') {
                d.to_string()
            } else {
                format!("{d}T23:59:59")
            }
        });
        let url = build_url(
            "heartrate",
            start_dt.as_deref(),
            end_dt.as_deref(),
            "start_datetime",
            "end_datetime",
        );
        let resp: HeartRateResponse = self.get(&url).await?;
        Ok(if resp.data.is_empty() {
            None
        } else {
            Some(resp)
        })
    }

    // ── Internal ──────────────────────────────────────────────────────────────

    async fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        tracing::debug!(url = %url, "sending request");
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?;
        tracing::debug!(status = %resp.status(), "received response");
        Ok(resp.json::<T>().await?)
    }
}

fn build_url(
    endpoint: &str,
    start: Option<&str>,
    end: Option<&str>,
    start_key: &str,
    end_key: &str,
) -> String {
    let mut params = vec![];
    if let Some(s) = start {
        params.push(format!("{start_key}={s}"));
    }
    if let Some(e) = end {
        params.push(format!("{end_key}={e}"));
    }
    let base = format!("{BASE_URL}/v2/usercollection/{endpoint}");
    if params.is_empty() {
        base
    } else {
        format!("{}?{}", base, params.join("&"))
    }
}
