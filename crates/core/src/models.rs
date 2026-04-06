use serde::{Deserialize, Serialize};

// ── Activity ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityContributors {
    pub meet_daily_targets: Option<i32>,
    pub move_every_hour: Option<i32>,
    pub recovery_time: Option<i32>,
    pub stay_active: Option<i32>,
    pub training_frequency: Option<i32>,
    pub training_volume: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleModel {
    pub interval: f64,
    pub items: Vec<Option<f64>>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyActivity {
    pub id: String,
    pub class_5_min: Option<String>,
    pub score: Option<i32>,
    pub active_calories: i32,
    pub average_met_minutes: f64,
    pub contributors: ActivityContributors,
    pub equivalent_walking_distance: i32,
    pub high_activity_met_minutes: i32,
    pub high_activity_time: i32,
    pub inactivity_alerts: i32,
    pub low_activity_met_minutes: i32,
    pub low_activity_time: i32,
    pub medium_activity_met_minutes: i32,
    pub medium_activity_time: i32,
    pub met: SampleModel,
    pub meters_to_target: Option<i32>,
    pub non_wear_time: i32,
    pub resting_time: i32,
    pub sedentary_met_minutes: i32,
    pub sedentary_time: i32,
    pub steps: i32,
    pub target_calories: Option<i32>,
    pub target_meters: Option<i32>,
    pub total_calories: i32,
    pub day: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyActivityResponse {
    pub data: Vec<DailyActivity>,
    pub next_token: Option<String>,
}

// ── Readiness ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadinessContributors {
    pub activity_balance: Option<i32>,
    pub body_temperature: Option<i32>,
    pub hrv_balance: Option<i32>,
    pub previous_day_activity: Option<i32>,
    pub previous_night: Option<i32>,
    pub recovery_index: Option<i32>,
    pub resting_heart_rate: Option<i32>,
    pub sleep_balance: Option<i32>,
    pub sleep_regularity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyReadiness {
    pub id: String,
    pub contributors: ReadinessContributors,
    pub day: String,
    pub score: Option<i32>,
    pub temperature_deviation: Option<f64>,
    pub temperature_trend_deviation: Option<f64>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyReadinessResponse {
    pub data: Vec<DailyReadiness>,
    pub next_token: Option<String>,
}

// ── Stress ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DailyStressSummary {
    Restored,
    Normal,
    Stressful,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyStress {
    pub id: String,
    pub day: String,
    pub stress_high: Option<i32>,
    pub recovery_high: Option<i32>,
    pub day_summary: Option<DailyStressSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyStressResponse {
    pub data: Vec<DailyStress>,
    pub next_token: Option<String>,
}

// ── Heart Rate ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HeartRateSource {
    Awake,
    Rest,
    Sleep,
    Session,
    Live,
    Workout,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartRate {
    pub bpm: i32,
    pub source: HeartRateSource,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartRateResponse {
    pub data: Vec<HeartRate>,
    pub next_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_activity_deserialize() {
        let json = r#"{
            "data": [{
                "id": "abc123",
                "class_5_min": null,
                "score": 72,
                "active_calories": 450,
                "average_met_minutes": 1.8,
                "contributors": {
                    "meet_daily_targets": 80,
                    "move_every_hour": 60,
                    "recovery_time": 70,
                    "stay_active": 75,
                    "training_frequency": 65,
                    "training_volume": 55
                },
                "equivalent_walking_distance": 6500,
                "high_activity_met_minutes": 30,
                "high_activity_time": 1800,
                "inactivity_alerts": 2,
                "low_activity_met_minutes": 120,
                "low_activity_time": 7200,
                "medium_activity_met_minutes": 60,
                "medium_activity_time": 3600,
                "met": {"interval": 300.0, "items": [1.2, null, 2.1], "timestamp": "2026-04-05T06:00:00"},
                "meters_to_target": 1500,
                "non_wear_time": 3600,
                "resting_time": 28800,
                "sedentary_met_minutes": 200,
                "sedentary_time": 12000,
                "steps": 8500,
                "target_calories": 600,
                "target_meters": 8000,
                "total_calories": 2200,
                "day": "2026-04-05",
                "timestamp": "2026-04-05T06:00:00"
            }],
            "next_token": null
        }"#;
        let resp: DailyActivityResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data[0].day, "2026-04-05");
        assert_eq!(resp.data[0].score, Some(72));
        assert_eq!(resp.data[0].steps, 8500);
    }

    #[test]
    fn test_daily_readiness_deserialize() {
        let json = r#"{
            "data": [{
                "id": "def456",
                "contributors": {
                    "activity_balance": 75,
                    "body_temperature": 80,
                    "hrv_balance": 70,
                    "previous_day_activity": 65,
                    "previous_night": 85,
                    "recovery_index": 72,
                    "resting_heart_rate": 88,
                    "sleep_balance": 78,
                    "sleep_regularity": 82
                },
                "day": "2026-04-05",
                "score": 80,
                "temperature_deviation": 0.1,
                "temperature_trend_deviation": -0.05,
                "timestamp": "2026-04-05T06:00:00"
            }],
            "next_token": null
        }"#;
        let resp: DailyReadinessResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data[0].day, "2026-04-05");
        assert_eq!(resp.data[0].score, Some(80));
    }

    #[test]
    fn test_daily_stress_deserialize() {
        let json = r#"{
            "data": [{
                "id": "ghi789",
                "day": "2026-04-05",
                "stress_high": 3600,
                "recovery_high": 7200,
                "day_summary": "normal"
            }],
            "next_token": null
        }"#;
        let resp: DailyStressResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data[0].day, "2026-04-05");
        assert_eq!(resp.data[0].day_summary, Some(DailyStressSummary::Normal));
    }

    #[test]
    fn test_heartrate_deserialize() {
        let json = r#"{
            "data": [
                {"bpm": 62, "source": "rest", "timestamp": "2026-04-05T02:00:00"},
                {"bpm": 58, "source": "sleep", "timestamp": "2026-04-05T03:00:00"}
            ],
            "next_token": null
        }"#;
        let resp: HeartRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data[0].bpm, 62);
        assert_eq!(resp.data[1].bpm, 58);
    }
}
