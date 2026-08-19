use chrono::NaiveDateTime;

/// Parses the modal's "when" field: either the literal `"now"`, or a
/// `"YYYY-MM-DD HH:MM"` UTC timestamp. Returns unix seconds, or an error
/// message suitable for showing back to the user.
pub fn parse_when(input: &str, now_unix: i64) -> Result<i64, String> {
    let trimmed = input.trim();
    if trimmed.eq_ignore_ascii_case("now") {
        return Ok(now_unix);
    }

    let Ok(parsed) = NaiveDateTime::parse_from_str(&format!("{trimmed}:00"), "%Y-%m-%d %H:%M:%S")
    else {
        return Err(format!(
            "\"{trimmed}\" isn't a valid time. Use \"now\" or \"YYYY-MM-DD HH:MM\" (UTC)."
        ));
    };

    let scheduled_at = parsed.and_utc().timestamp();
    if scheduled_at < now_unix {
        return Err("That time is in the past.".to_string());
    }
    Ok(scheduled_at)
}

/// Parses the modal's optional reminder-lead-time field. A blank/absent input
/// means "no reminder".
pub fn parse_reminder_minutes(input: Option<&str>) -> Result<Option<i64>, String> {
    let Some(trimmed) = input.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(None);
    };
    match trimmed.parse::<i64>() {
        Ok(minutes) if minutes > 0 => Ok(Some(minutes)),
        _ => Err(format!(
            "\"{trimmed}\" isn't a valid number of minutes. Leave blank for no reminder."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i64 = 1_000_000;

    #[test]
    fn parse_when_accepts_now_case_insensitively() {
        assert_eq!(parse_when("now", NOW), Ok(NOW));
        assert_eq!(parse_when("NOW", NOW), Ok(NOW));
        assert_eq!(parse_when("  now  ", NOW), Ok(NOW));
    }

    #[test]
    fn parse_when_accepts_valid_future_datetime() {
        // 2001-09-09 01:47:00 UTC == 1_000_000_020
        let result = parse_when("2001-09-09 01:47", NOW);
        assert_eq!(result, Ok(1_000_000_020));
    }

    #[test]
    fn parse_when_rejects_malformed_string() {
        assert!(parse_when("not a date", NOW).is_err());
    }

    #[test]
    fn parse_when_rejects_past_datetime() {
        // 1970-01-01 00:00 UTC is unix time 0, well before NOW.
        assert!(parse_when("1970-01-01 00:00", NOW).is_err());
    }

    #[test]
    fn parse_reminder_minutes_blank_is_none() {
        assert_eq!(parse_reminder_minutes(None), Ok(None));
        assert_eq!(parse_reminder_minutes(Some("")), Ok(None));
        assert_eq!(parse_reminder_minutes(Some("   ")), Ok(None));
    }

    #[test]
    fn parse_reminder_minutes_accepts_positive_integer() {
        assert_eq!(parse_reminder_minutes(Some("10")), Ok(Some(10)));
    }

    #[test]
    fn parse_reminder_minutes_rejects_non_numeric_or_non_positive() {
        assert!(parse_reminder_minutes(Some("abc")).is_err());
        assert!(parse_reminder_minutes(Some("0")).is_err());
        assert!(parse_reminder_minutes(Some("-5")).is_err());
    }
}
