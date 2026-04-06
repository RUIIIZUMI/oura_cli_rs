# Command Dispatch Redesign

**Date:** 2026-04-06  
**Branch:** feature/phase1

## Goal

Refactor `main.rs` command dispatch into a clean `command.rs` module. Each command becomes an independent struct implementing `Execute`, with shared types (`DateRange`, `Display`) removing duplication. The "no data" case is expressed via `Option` in the client return type rather than checked inline.

## Module Structure

```
oura_cli/src/
  main.rs       # parse CLI → from_cli() → execute()
  command.rs    # Execute trait, command structs, from_cli dispatcher
  display.rs    # Display struct, OutputMode, Displayable trait
  util.rs       # init_tracing (unchanged)

crates/core/src/
  client.rs     # client methods return Result<Option<XxxResponse>>
```

## Types

### `DateRange` (in `command.rs`)

```rust
pub struct DateRange {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

impl DateRange {
    pub fn parse(start: Option<String>, end: Option<String>) -> Result<Self>;
}
```

Validation rules:
- Format must be `YYYY-MM-DD` (via `NaiveDate::parse_from_str`)
- If both `start` and `end` are provided, `start <= end`

`DateRange` provides `as_start_str()` / `as_end_str()` helpers returning `Option<String>` for passing to the client.

### `OutputMode` + `Display` (in `display.rs`)

```rust
pub enum OutputMode { Pretty, Json }

pub struct Display {
    pub mode: OutputMode,
}

pub trait Displayable: serde::Serialize {
    fn display_pretty(&self);
}

impl Display {
    pub fn show<T: Displayable + serde::Serialize>(&self, data: &T) {
        match self.mode {
            OutputMode::Json => println!("{}", serde_json::to_string_pretty(data).unwrap()),
            OutputMode::Pretty => data.display_pretty(),
        }
    }
}
```

Each response type (`DailySleepResponse`, etc.) implements `Displayable`. The existing `print_*` functions in `display.rs` become `display_pretty` implementations.

### `Execute` trait + command structs (in `command.rs`)

```rust
#[async_trait]
pub trait Execute {
    async fn execute(&self, client: &OuraClient) -> Result<()>;
}

pub struct SleepCommand    { pub date_range: DateRange, pub display: Display }
pub struct ActivityCommand { pub date_range: DateRange, pub display: Display }
pub struct ReadinessCommand{ pub date_range: DateRange, pub display: Display }
pub struct StressCommand   { pub date_range: DateRange, pub display: Display }
pub struct HeartrateCommand{ pub date_range: DateRange, pub display: Display }
```

Each implements `Execute`. The pattern inside `execute` is uniform:

```rust
match client.get_daily_sleep(
    self.date_range.as_start_str().as_deref(),
    self.date_range.as_end_str().as_deref(),
).await? {
    None => println!("No sleep data found."),
    Some(resp) => self.display.show(&resp),
}
```

### `from_cli` dispatcher (in `command.rs`)

```rust
pub fn from_cli(cmd: Commands) -> Result<Box<dyn Execute + Send>> {
    match cmd {
        Commands::Sleep { start_date, end_date, json } => Ok(Box::new(SleepCommand {
            date_range: DateRange::parse(start_date, end_date)?,
            display: Display { mode: if json { OutputMode::Json } else { OutputMode::Pretty } },
        })),
        // ... other commands
    }
}
```

Validation errors (bad date format, start > end) surface here as `Err`, before the client is ever called.

## Client Changes (`crates/core/src/client.rs`)

All five public methods change return type from `Result<XxxResponse>` to `Result<Option<XxxResponse>>`.

```rust
pub async fn get_daily_sleep(...) -> Result<Option<DailySleepResponse>> {
    let resp: DailySleepResponse = self.get(&url).await?;
    Ok(if resp.data.is_empty() { None } else { Some(resp) })
}
```

The private `get<T>` helper is unchanged.

## `main.rs` After

```rust
let cmd = command::from_cli(cli.command)?;
cmd.execute(&client).await?;
```

All command logic, validation, and display decisions are gone from `main.rs`.

## Error Handling

| Situation | Behaviour |
|---|---|
| Bad date format | `from_cli` returns `Err` — eyre error shown |
| `start > end` | `from_cli` returns `Err` — eyre error shown |
| API HTTP error | `?` propagates from `client.get()` |
| Empty response | `Ok(None)` → command prints "No X data found." |

## Out of Scope

- Adding new commands
- Changing the display format of existing output
- Async trait without `async_trait` crate (use `async_trait` for now)
