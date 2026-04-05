use eyre::Result;
use reqwest::Client;

use crate::models::DailySleepResponse;

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
    ) -> Result<DailySleepResponse> {
        let mut params = vec![];
        if let Some(start) = start_date {
            params.push(format!("start_date={start}"));
        }
        if let Some(end) = end_date {
            params.push(format!("end_date={end}"));
        }
        let mut url = format!("{BASE_URL}/v2/usercollection/daily_sleep");
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json::<DailySleepResponse>().await?)
    }
}
