use chrono::NaiveDate;
use eyre::{Result, bail, eyre}; // eyre and bail are used in parse implementation

pub struct DateRange {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

impl DateRange {
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
