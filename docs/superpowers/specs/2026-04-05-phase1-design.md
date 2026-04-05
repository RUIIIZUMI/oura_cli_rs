# Phase 1 Design: oura_cli_rs

**Date:** 2026-04-05  
**Status:** Approved

---

## Goal

Implement a production-ready personal CLI for the Oura API that covers the 5 core daily metrics, supports both human-readable and JSON output, and ships cross-platform binaries via GitHub Releases.

---

## CLI Command Structure

```
oura <command> [options]

Commands:
  sleep       Daily sleep score & contributors
  activity    Daily activity score & metrics
  readiness   Daily readiness score
  stress      Daily stress level
  heartrate   Heart rate samples

Global options per command:
  --start-date  YYYY-MM-DD
  --end-date    YYYY-MM-DD
  --json        Output raw JSON instead of pretty print
```

All commands read `OURA_RING_API_KEY` from the environment (or `.env` file via dotenvy).

---

## Code Architecture

### `oura_core` crate (library)

**`client.rs`** — Add 4 new methods to `OuraClient`:
- `get_daily_activity(start_date, end_date) -> Result<DailyActivityResponse>`
- `get_daily_readiness(start_date, end_date) -> Result<DailyReadinessResponse>`
- `get_daily_stress(start_date, end_date) -> Result<DailyStressResponse>`
- `get_heartrate(start_date, end_date) -> Result<HeartRateResponse>`

**`models.rs`** — Add response types for each new endpoint. All types derive `Serialize` (for JSON output) and `Deserialize`.

### `oura_cli` crate (binary)

**`main.rs`** — Add 4 new variants to `Commands` enum, each with `--start-date`, `--end-date`, and `--json` flags. On `--json`, serialize response with `serde_json::to_string_pretty` and print. Otherwise, call a formatter.

---

## Output Formats

### Pretty print (default)

Score-based commands (`sleep`, `activity`, `readiness`) use a colored bar chart:
```
Sleep Score
──────────────────────────────────────────────────
04-05 │ ████████████████████████████████████      78
04-04 │ ████████████████████████████████████████  82
──────────────────────────────────────────────────
```
Color: green (≥80), yellow (≥60), red (<60).

Non-score commands use text tables:
```
Stress Level
──────────────────────────────────────────────────
04-05 │ stress=high  recovery=low
──────────────────────────────────────────────────

Heart Rate (bpm)
──────────────────────────────────────────────────
04-05 09:00 │ 62 bpm
04-05 09:05 │ 65 bpm
──────────────────────────────────────────────────
```

### JSON output (`--json`)

Raw API response serialized with `serde_json::to_string_pretty`. Useful for scripting and future MCP integration.

---

## GitHub Release CI

**File:** `.github/workflows/release.yml`  
**Trigger:** push of a `v*` tag

**Matrix targets:**
- `x86_64-apple-darwin` (macOS Intel)
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-unknown-linux-gnu` (Linux)

**Steps per target:**
1. `cargo build --release --target <target>`
2. Archive binary as `oura-<target>.tar.gz`
3. Upload to GitHub Releases via `gh release upload`

Cross-compilation handled via GitHub Actions runner matrix (no `cross` crate needed for the initial targets).

**Installation (manual, Phase 1):**
```bash
curl -L https://github.com/<user>/oura_cli_rs/releases/latest/download/oura-aarch64-apple-darwin.tar.gz | tar xz
mv oura ~/.local/bin/
```

Installation UX improvements (Homebrew tap, `cargo-dist`, etc.) are deferred to Phase 2.

---

## Out of Scope for Phase 1

- `workout`, `session`, `spo2`, `daily_stress_resilience`, `vO2_max`, `ring_configuration`, `tags`
- Webhook subscriptions
- Generic `raw` command
- MCP server
- Homebrew / cargo-dist distribution
- Config file for API key (env var only)
