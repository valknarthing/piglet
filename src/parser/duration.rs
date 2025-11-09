use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DURATION_REGEX: Regex = Regex::new(r"^(\d+(?:\.\d+)?)(ms|s|m|h)$").unwrap();
}

/// Parse duration string to milliseconds
/// Supports: 3000ms, 0.3s, 5m, 0.5h
pub fn parse_duration(duration: &str) -> Result<u64> {
    let caps = DURATION_REGEX
        .captures(duration.trim())
        .ok_or_else(|| anyhow::anyhow!("Invalid duration format: {}", duration))?;

    let value: f64 = caps[1]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid numeric value in duration"))?;

    let unit = &caps[2];

    let milliseconds = match unit {
        "ms" => value,
        "s" => value * 1000.0,
        "m" => value * 60.0 * 1000.0,
        "h" => value * 60.0 * 60.0 * 1000.0,
        _ => bail!("Unknown time unit: {}", unit),
    };

    if milliseconds < 0.0 {
        bail!("Duration cannot be negative");
    }

    Ok(milliseconds as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_milliseconds() {
        assert_eq!(parse_duration("3000ms").unwrap(), 3000);
        assert_eq!(parse_duration("500ms").unwrap(), 500);
    }

    #[test]
    fn test_parse_seconds() {
        assert_eq!(parse_duration("3s").unwrap(), 3000);
        assert_eq!(parse_duration("0.5s").unwrap(), 500);
        assert_eq!(parse_duration("1.5s").unwrap(), 1500);
    }

    #[test]
    fn test_parse_minutes() {
        assert_eq!(parse_duration("1m").unwrap(), 60000);
        assert_eq!(parse_duration("0.5m").unwrap(), 30000);
    }

    #[test]
    fn test_parse_hours() {
        assert_eq!(parse_duration("1h").unwrap(), 3600000);
        assert_eq!(parse_duration("0.5h").unwrap(), 1800000);
    }

    #[test]
    fn test_invalid_format() {
        assert!(parse_duration("invalid").is_err());
        assert!(parse_duration("10").is_err());
        assert!(parse_duration("10x").is_err());
    }
}
