use eyre::Result;
use reqwest::Client;

use crate::models::{
    DailyActivityResponse, DailyReadinessResponse, DailySleepResponse, DailyStressResponse,
    HeartRateResponse,
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

    pub async fn get_daily_sleep(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Option<DailySleepResponse>> {
        let url = build_url(
            "daily_sleep",
            start_date,
            end_date,
            "start_date",
            "end_date",
        );
        let resp: DailySleepResponse = self.get(&url).await?;
        Ok(if resp.data.is_empty() {
            None
        } else {
            Some(resp)
        })
    }

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
        // heartrate endpoint uses datetime format; append T00:00:00 if only a date is given
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

#[cfg(test)]
mod tests {
    use crate::models::{DailySleep, DailySleepResponse, SleepContributors};

    fn empty_sleep_response() -> DailySleepResponse {
        DailySleepResponse {
            data: vec![],
            next_token: None,
        }
    }

    fn nonempty_sleep_response() -> DailySleepResponse {
        DailySleepResponse {
            data: vec![DailySleep {
                id: "x".into(),
                day: "2026-04-01".into(),
                score: Some(80),
                contributors: SleepContributors {
                    deep_sleep: None,
                    efficiency: None,
                    latency: None,
                    rem_sleep: None,
                    restfulness: None,
                    timing: None,
                    total_sleep: None,
                },
                timestamp: "2026-04-01T06:00:00".into(),
            }],
            next_token: None,
        }
    }

    #[test]
    fn empty_response_becomes_none() {
        let resp = empty_sleep_response();
        let result: Option<DailySleepResponse> = if resp.data.is_empty() {
            None
        } else {
            Some(resp)
        };
        assert!(result.is_none());
    }

    #[test]
    fn nonempty_response_becomes_some() {
        let resp = nonempty_sleep_response();
        let result: Option<DailySleepResponse> = if resp.data.is_empty() {
            None
        } else {
            Some(resp)
        };
        assert!(result.is_some());
    }
}
