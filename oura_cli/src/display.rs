use oura_core::models::{
    DailyActivity, DailyActivityResponse, DailyReadiness, DailyReadinessResponse,
    DailySleepResponse, DailyStress, DailyStressResponse, DailyStressSummary, HeartRate,
    HeartRateResponse,
};

pub enum OutputMode {
    Pretty,
    Json,
}

pub struct Display {
    pub mode: OutputMode,
}

pub trait Displayable: serde::Serialize {
    fn display_pretty(&self);
}

impl Display {
    pub fn show<T: Displayable>(&self, data: &T) {
        match self.mode {
            OutputMode::Json => {
                println!("{}", serde_json::to_string_pretty(data).unwrap())
            }
            OutputMode::Pretty => data.display_pretty(),
        }
    }
}

// ── Displayable impls ─────────────────────────────────────────────────────────

impl Displayable for DailySleepResponse {
    fn display_pretty(&self) {
        print_score_chart(
            "Sleep Score",
            self.data.iter().map(|e| {
                (
                    e.day.get(5..).unwrap_or(&e.day).to_string(),
                    e.score.unwrap_or(0),
                )
            }),
        );
    }
}

impl Displayable for DailyActivityResponse {
    fn display_pretty(&self) {
        print_score_chart(
            "Activity Score",
            self.data.iter().map(|e| {
                (
                    e.day.get(5..).unwrap_or(&e.day).to_string(),
                    e.score.unwrap_or(0),
                )
            }),
        );
        print_activity_extras(&self.data);
    }
}

impl Displayable for DailyReadinessResponse {
    fn display_pretty(&self) {
        print_score_chart(
            "Readiness Score",
            self.data.iter().map(|e| {
                (
                    e.day.get(5..).unwrap_or(&e.day).to_string(),
                    e.score.unwrap_or(0),
                )
            }),
        );
        print_readiness_extras(&self.data);
    }
}

impl Displayable for DailyStressResponse {
    fn display_pretty(&self) {
        print_stress_table(&self.data);
    }
}

impl Displayable for HeartRateResponse {
    fn display_pretty(&self) {
        print_heartrate_table(&self.data);
    }
}

// ── Private helpers ───────────────────────────────────────────────────────────

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
        println!(
            "{:<8} {:>8} {:>8}",
            e.day.get(5..).unwrap_or(&e.day),
            e.steps,
            e.active_calories
        );
    }
}

fn print_readiness_extras(entries: &[DailyReadiness]) {
    println!();
    println!("{:<8} {:>12} {:>12}", "Date", "Temp Dev", "HRV Bal");
    println!("{}", "─".repeat(36));
    for e in entries {
        let temp = e
            .temperature_deviation
            .map_or("-".to_string(), |t| format!("{t:+.2}°C"));
        let hrv = e
            .contributors
            .hrv_balance
            .map_or("-".to_string(), |h| h.to_string());
        println!(
            "{:<8} {:>12} {:>12}",
            e.day.get(5..).unwrap_or(&e.day),
            temp,
            hrv
        );
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
            e.day.get(5..).unwrap_or(&e.day),
            summary,
            stress_min,
            recovery_min
        );
    }
    println!("{}", "─".repeat(54));
}

fn print_heartrate_table(entries: &[HeartRate]) {
    println!("Heart Rate (bpm)");
    println!("{}", "─".repeat(54));
    let display = if entries.len() > 20 {
        &entries[entries.len() - 20..]
    } else {
        entries
    };
    for e in display {
        let ts = e.timestamp.get(..16).unwrap_or(&e.timestamp);
        println!("{ts} │ {} bpm", e.bpm);
    }
    if entries.len() > 20 {
        println!("  (showing last 20 of {} samples)", entries.len());
    }
    println!("{}", "─".repeat(54));
}

#[cfg(test)]
mod tests {
    use super::*;
    use oura_core::models::DailySleepResponse;

    #[test]
    fn show_json_serializes_response() {
        let resp = DailySleepResponse {
            data: vec![],
            next_token: None,
        };
        let json = serde_json::to_string_pretty(&resp).unwrap();
        assert!(json.contains("\"data\""));
    }

    #[test]
    fn output_mode_default_is_pretty() {
        let d = Display {
            mode: OutputMode::Pretty,
        };
        assert!(matches!(d.mode, OutputMode::Pretty));
    }
}
