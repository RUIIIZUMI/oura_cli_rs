# Command Dispatch Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor `main.rs` command dispatch into `command.rs` with per-command structs, shared `DateRange`/`Display` types, and `Result<Option<T>>` client return types.

**Architecture:** Each command becomes an independent struct implementing `Execute` via `async_trait`. `DateRange` centralises date validation. `Display` owns the output mode decision (pretty vs JSON). The client returns `Option` instead of an empty vec, making "no data" explicit in the type system.

**Tech Stack:** Rust 2024, `async-trait 0.1`, `chrono 0.4` (NaiveDate), `clap 4`, `eyre`, `serde_json`

---

## File Map

| Action | Path | Responsibility |
|--------|------|----------------|
| Modify | `Cargo.toml` | Add `async-trait`, `chrono` to workspace deps |
| Modify | `oura_cli/Cargo.toml` | Pull in `async-trait`, `chrono` |
| Modify | `crates/core/src/client.rs` | Change 5 public methods to `Result<Option<XxxResponse>>` |
| Modify | `oura_cli/src/display.rs` | Add `OutputMode`, `Display`, `Displayable` trait; convert `print_*` fns to trait impls |
| Create | `oura_cli/src/command.rs` | `DateRange`, `Execute` trait, 5 command structs, `from_cli` dispatcher |
| Modify | `oura_cli/src/main.rs` | Remove `match` block; call `command::from_cli` + `.execute` |

---

## Task 1: Add workspace dependencies

**Files:**
- Modify: `Cargo.toml`
- Modify: `oura_cli/Cargo.toml`

- [ ] **Step 1: Add async-trait and chrono to workspace dependencies**

In `Cargo.toml`, add to `[workspace.dependencies]`:

```toml
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
```

- [ ] **Step 2: Add them to oura_cli/Cargo.toml**

In `oura_cli/Cargo.toml`, add to `[dependencies]`:

```toml
async-trait.workspace = true
chrono.workspace = true
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo check
```

Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml oura_cli/Cargo.toml Cargo.lock
git commit -m "chore: add async-trait and chrono workspace dependencies"
```

---

## Task 2: Update client.rs — return `Result<Option<T>>`

**Files:**
- Modify: `crates/core/src/client.rs`

- [ ] **Step 1: Write failing test for empty-data → None behaviour**

Add to the bottom of `crates/core/src/client.rs`:

```rust
#[cfg(test)]
mod tests {
    use crate::models::{DailySleepResponse, DailySleep, SleepContributors};

    fn empty_sleep_response() -> DailySleepResponse {
        DailySleepResponse { data: vec![], next_token: None }
    }

    fn nonempty_sleep_response() -> DailySleepResponse {
        DailySleepResponse {
            data: vec![DailySleep {
                id: "x".into(),
                day: "2026-04-01".into(),
                score: Some(80),
                contributors: SleepContributors {
                    deep_sleep: None, efficiency: None, latency: None,
                    rem_sleep: None, restfulness: None, timing: None, total_sleep: None,
                },
                timestamp: "2026-04-01T06:00:00".into(),
            }],
            next_token: None,
        }
    }

    #[test]
    fn empty_response_becomes_none() {
        let resp = empty_sleep_response();
        let result: Option<DailySleepResponse> = if resp.data.is_empty() { None } else { Some(resp) };
        assert!(result.is_none());
    }

    #[test]
    fn nonempty_response_becomes_some() {
        let resp = nonempty_sleep_response();
        let result: Option<DailySleepResponse> = if resp.data.is_empty() { None } else { Some(resp) };
        assert!(result.is_some());
    }
}
```

- [ ] **Step 2: Run tests to see them pass (they test the inline logic; the real test is the signature change)**

```bash
cargo test -p oura_core
```

Expected: all tests pass (the new tests validate the branching logic).

- [ ] **Step 3: Change return types of the 5 public methods**

Replace the entire `impl OuraClient` block's public methods in `crates/core/src/client.rs`. Each method gets the pattern `Ok(if resp.data.is_empty() { None } else { Some(resp) })`:

```rust
pub async fn get_daily_sleep(
    &self,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Option<DailySleepResponse>> {
    let url = build_url("daily_sleep", start_date, end_date, "start_date", "end_date");
    let resp: DailySleepResponse = self.get(&url).await?;
    Ok(if resp.data.is_empty() { None } else { Some(resp) })
}

pub async fn get_daily_activity(
    &self,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Option<DailyActivityResponse>> {
    let url = build_url("daily_activity", start_date, end_date, "start_date", "end_date");
    let resp: DailyActivityResponse = self.get(&url).await?;
    Ok(if resp.data.is_empty() { None } else { Some(resp) })
}

pub async fn get_daily_readiness(
    &self,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Option<DailyReadinessResponse>> {
    let url = build_url("daily_readiness", start_date, end_date, "start_date", "end_date");
    let resp: DailyReadinessResponse = self.get(&url).await?;
    Ok(if resp.data.is_empty() { None } else { Some(resp) })
}

pub async fn get_daily_stress(
    &self,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Option<DailyStressResponse>> {
    let url = build_url("daily_stress", start_date, end_date, "start_date", "end_date");
    let resp: DailyStressResponse = self.get(&url).await?;
    Ok(if resp.data.is_empty() { None } else { Some(resp) })
}

pub async fn get_heartrate(
    &self,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Option<HeartRateResponse>> {
    let start_dt = start_date.map(|d| {
        if d.contains('T') { d.to_string() } else { format!("{d}T00:00:00") }
    });
    let end_dt = end_date.map(|d| {
        if d.contains('T') { d.to_string() } else { format!("{d}T23:59:59") }
    });
    let url = build_url(
        "heartrate",
        start_dt.as_deref(),
        end_dt.as_deref(),
        "start_datetime",
        "end_datetime",
    );
    let resp: HeartRateResponse = self.get(&url).await?;
    Ok(if resp.data.is_empty() { None } else { Some(resp) })
}
```

- [ ] **Step 4: Verify oura_core still compiles**

```bash
cargo check -p oura_core
```

Expected: no errors. (`oura_cli` will fail — that's expected until Task 6.)

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/client.rs
git commit -m "feat(core): client methods return Result<Option<T>>, None on empty data"
```

---

## Task 3: Rewrite display.rs

**Files:**
- Modify: `oura_cli/src/display.rs`

- [ ] **Step 1: Write a failing test for Display::show in JSON mode**

Add a `#[cfg(test)]` block at the end of `oura_cli/src/display.rs` (before writing the actual new code, add the module skeleton so the test can reference the types):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use oura_core::models::{DailySleepResponse};

    #[test]
    fn show_json_serializes_response() {
        let resp = DailySleepResponse { data: vec![], next_token: None };
        // Verify serialization doesn't panic and produces valid JSON
        let json = serde_json::to_string_pretty(&resp).unwrap();
        assert!(json.contains("\"data\""));
    }

    #[test]
    fn output_mode_default_is_pretty() {
        let d = Display { mode: OutputMode::Pretty };
        assert!(matches!(d.mode, OutputMode::Pretty));
    }
}
```

- [ ] **Step 2: Run tests to confirm they fail (types not defined yet)**

```bash
cargo test -p oura_cli 2>&1 | head -20
```

Expected: compile error — `Display`, `OutputMode` not found.

- [ ] **Step 3: Replace display.rs with new implementation**

Overwrite `oura_cli/src/display.rs` entirely:

```rust
use oura_core::models::{
    DailyActivity, DailyActivityResponse, DailyReadiness, DailyReadinessResponse,
    DailyStress, DailyStressSummary, DailyStressResponse, HeartRate, HeartRateResponse,
    DailySleepResponse,
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
                (e.day.get(5..).unwrap_or(&e.day).to_string(), e.score.unwrap_or(0))
            }),
        );
    }
}

impl Displayable for DailyActivityResponse {
    fn display_pretty(&self) {
        print_score_chart(
            "Activity Score",
            self.data.iter().map(|e| {
                (e.day.get(5..).unwrap_or(&e.day).to_string(), e.score.unwrap_or(0))
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
                (e.day.get(5..).unwrap_or(&e.day).to_string(), e.score.unwrap_or(0))
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
        let resp = DailySleepResponse { data: vec![], next_token: None };
        let json = serde_json::to_string_pretty(&resp).unwrap();
        assert!(json.contains("\"data\""));
    }

    #[test]
    fn output_mode_default_is_pretty() {
        let d = Display { mode: OutputMode::Pretty };
        assert!(matches!(d.mode, OutputMode::Pretty));
    }
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test -p oura_cli
```

Expected: `show_json_serializes_response` and `output_mode_default_is_pretty` pass.

- [ ] **Step 5: Commit**

```bash
git add oura_cli/src/display.rs
git commit -m "feat(cli): add OutputMode/Display/Displayable, convert print_* to trait impls"
```

---

## Task 4: Create command.rs — DateRange

**Files:**
- Create: `oura_cli/src/command.rs`

- [ ] **Step 1: Write failing tests for DateRange::parse**

Create `oura_cli/src/command.rs` with just the tests first:

```rust
use chrono::NaiveDate;
use eyre::{Result, bail, eyre};

pub struct DateRange {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

impl DateRange {
    pub fn parse(start: Option<String>, end: Option<String>) -> Result<Self> {
        todo!()
    }

    pub fn as_start_str(&self) -> Option<String> {
        self.start.map(|d| d.format("%Y-%m-%d").to_string())
    }

    pub fn as_end_str(&self) -> Option<String> {
        self.end.map(|d| d.format("%Y-%m-%d").to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_dates() {
        let dr = DateRange::parse(
            Some("2026-04-01".into()),
            Some("2026-04-07".into()),
        ).unwrap();
        assert_eq!(dr.as_start_str(), Some("2026-04-01".into()));
        assert_eq!(dr.as_end_str(), Some("2026-04-07".into()));
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
        let result = DateRange::parse(
            Some("2026-04-10".into()),
            Some("2026-04-01".into()),
        );
        assert!(result.is_err());
    }

    #[test]
    fn parse_same_start_and_end_is_ok() {
        let result = DateRange::parse(
            Some("2026-04-05".into()),
            Some("2026-04-05".into()),
        );
        assert!(result.is_ok());
    }
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cargo test -p oura_cli date_range 2>&1 | tail -10
```

Expected: all 5 tests fail with `not yet implemented` (todo! panics).

- [ ] **Step 3: Implement DateRange::parse**

Replace the `todo!()` in `DateRange::parse`:

```rust
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
    if let (Some(s), Some(e)) = (start, end) {
        if s > e {
            bail!("start_date {s} is after end_date {e}");
        }
    }
    Ok(Self { start, end })
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cargo test -p oura_cli date_range
```

Expected: all 5 tests pass.

- [ ] **Step 5: Commit**

```bash
git add oura_cli/src/command.rs
git commit -m "feat(cli): add DateRange with YYYY-MM-DD format and range validation"
```

---

## Task 5: Add Execute trait, command structs, and from_cli

**Files:**
- Modify: `oura_cli/src/command.rs`

- [ ] **Step 1: Write failing test for from_cli error on bad date**

Add to the `tests` module in `command.rs`:

```rust
    #[test]
    fn from_cli_returns_err_on_bad_start_date() {
        use clap::Parser;
        // Parse a Cli struct directly to get a Commands value
        let result = DateRange::parse(Some("not-a-date".into()), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid start_date"));
    }
```

- [ ] **Step 2: Run test to confirm it passes (DateRange::parse already works)**

```bash
cargo test -p oura_cli from_cli_returns_err_on_bad_start_date
```

Expected: passes.

- [ ] **Step 3: Add Execute trait, command structs, and from_cli to command.rs**

Append to `oura_cli/src/command.rs` (after the `DateRange` impl block, before the `#[cfg(test)]` block):

```rust
use async_trait::async_trait;
use oura_core::client::OuraClient;
use crate::display::{Display, OutputMode};

#[async_trait]
pub trait Execute: Send + Sync {
    async fn execute(&self, client: &OuraClient) -> Result<()>;
}

pub struct SleepCommand     { pub date_range: DateRange, pub display: Display }
pub struct ActivityCommand  { pub date_range: DateRange, pub display: Display }
pub struct ReadinessCommand { pub date_range: DateRange, pub display: Display }
pub struct StressCommand    { pub date_range: DateRange, pub display: Display }
pub struct HeartrateCommand { pub date_range: DateRange, pub display: Display }

#[async_trait]
impl Execute for SleepCommand {
    async fn execute(&self, client: &OuraClient) -> Result<()> {
        tracing::info!(command = "sleep", "executing command");
        match client.get_daily_sleep(
            self.date_range.as_start_str().as_deref(),
            self.date_range.as_end_str().as_deref(),
        ).await? {
            None => println!("No sleep data found."),
            Some(resp) => {
                tracing::info!(count = resp.data.len(), "fetched records");
                self.display.show(&resp);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Execute for ActivityCommand {
    async fn execute(&self, client: &OuraClient) -> Result<()> {
        tracing::info!(command = "activity", "executing command");
        match client.get_daily_activity(
            self.date_range.as_start_str().as_deref(),
            self.date_range.as_end_str().as_deref(),
        ).await? {
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
    async fn execute(&self, client: &OuraClient) -> Result<()> {
        tracing::info!(command = "readiness", "executing command");
        match client.get_daily_readiness(
            self.date_range.as_start_str().as_deref(),
            self.date_range.as_end_str().as_deref(),
        ).await? {
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
    async fn execute(&self, client: &OuraClient) -> Result<()> {
        tracing::info!(command = "stress", "executing command");
        match client.get_daily_stress(
            self.date_range.as_start_str().as_deref(),
            self.date_range.as_end_str().as_deref(),
        ).await? {
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
    async fn execute(&self, client: &OuraClient) -> Result<()> {
        tracing::info!(command = "heartrate", "executing command");
        match client.get_heartrate(
            self.date_range.as_start_str().as_deref(),
            self.date_range.as_end_str().as_deref(),
        ).await? {
            None => println!("No heart rate data found."),
            Some(resp) => {
                tracing::info!(count = resp.data.len(), "fetched records");
                self.display.show(&resp);
            }
        }
        Ok(())
    }
}
```

Note: `DisplayMode` is not a type — the import should be `use crate::display::{Display, OutputMode};`. Fix that import line to:

```rust
use crate::display::{Display, OutputMode};
```

- [ ] **Step 4: Add from_cli dispatcher**

Append to `command.rs` (still before `#[cfg(test)]`):

```rust
use crate::Commands;

pub fn from_cli(cmd: Commands) -> Result<Box<dyn Execute>> {
    match cmd {
        Commands::Sleep { start_date, end_date, json } => Ok(Box::new(SleepCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display { mode: if json { OutputMode::Json } else { OutputMode::Pretty } },
        })),
        Commands::Activity { start_date, end_date, json } => Ok(Box::new(ActivityCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display { mode: if json { OutputMode::Json } else { OutputMode::Pretty } },
        })),
        Commands::Readiness { start_date, end_date, json } => Ok(Box::new(ReadinessCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display { mode: if json { OutputMode::Json } else { OutputMode::Pretty } },
        })),
        Commands::Stress { start_date, end_date, json } => Ok(Box::new(StressCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display { mode: if json { OutputMode::Json } else { OutputMode::Pretty } },
        })),
        Commands::Heartrate { start_date, end_date, json } => Ok(Box::new(HeartrateCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display { mode: if json { OutputMode::Json } else { OutputMode::Pretty } },
        })),
    }
}
```

- [ ] **Step 5: Verify oura_cli compiles (main.rs still uses old code, that's ok)**

```bash
cargo check -p oura_cli 2>&1 | grep "^error" | head -20
```

Expected: errors only from `main.rs` (unused imports, unresolved match), not from `command.rs` itself.

- [ ] **Step 6: Commit**

```bash
git add oura_cli/src/command.rs
git commit -m "feat(cli): add Execute trait, command structs, and from_cli dispatcher"
```

---

## Task 6: Simplify main.rs

**Files:**
- Modify: `oura_cli/src/main.rs`

- [ ] **Step 1: Replace main.rs**

Overwrite `oura_cli/src/main.rs` entirely:

```rust
mod command;
mod display;
mod util;

use clap::{Parser, Subcommand};
use eyre::{Result, bail};
use oura_core::client::OuraClient;

#[derive(Parser)]
#[command(name = "oura", about = "Oura API CLI")]
struct Cli {
    /// Enable debug logging
    #[arg(long, global = true)]
    debug: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch daily sleep score
    Sleep {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch daily activity score
    Activity {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch daily readiness score
    Readiness {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch daily stress level
    Stress {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch heart rate samples
    Heartrate {
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
        /// Output raw JSON
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    util::init_tracing(cli.debug);

    tracing::info!("oura cli started");

    let token = match std::env::var("OURA_RING_API_KEY") {
        Ok(t) => t,
        Err(_) => bail!("OURA_RING_API_KEY environment variable is not set"),
    };

    let client = OuraClient::new(token);

    let cmd = command::from_cli(cli.command)?;
    cmd.execute(&client).await
}
```

Note: `Commands` is now `pub` so `command.rs` can import it via `use crate::Commands`.

- [ ] **Step 2: Build the full workspace**

```bash
cargo build
```

Expected: clean build with no errors.

- [ ] **Step 3: Run all tests**

```bash
cargo test
```

Expected: all tests pass.

- [ ] **Step 4: Commit**

```bash
git add oura_cli/src/main.rs
git commit -m "refactor(cli): simplify main.rs — dispatch via command::from_cli"
```

---

## Task 7: Run verify

- [ ] **Step 1: Run mise verify**

```bash
mise run verify
```

Expected: all checks pass (build, clippy, test).

- [ ] **Step 2: Commit if verify added any auto-fixes**

Only commit if `git diff` shows changes from verify tooling.
