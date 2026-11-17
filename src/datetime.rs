//! Test helpers for `jiff::Timestamp`.

pub(crate) fn parse(value: &str) -> Result<jiff::Timestamp, jiff::Error> {
    value.parse()
}

pub(crate) fn days(value: i64) -> jiff::SignedDuration {
    jiff::SignedDuration::from_hours(value * 24)
}

/// RFC 3339 text in the form the tests used before: a `+00:00` offset and
/// 0, 3, 6 or 9 fractional digits.
pub(crate) trait Rfc3339 {
    fn to_rfc3339(&self) -> String;
}

impl Rfc3339 for jiff::Timestamp {
    fn to_rfc3339(&self) -> String {
        let nanos = self.subsec_nanosecond().rem_euclid(1_000_000_000);
        let digits = if nanos == 0 {
            0
        } else if nanos % 1_000_000 == 0 {
            3
        } else if nanos % 1_000 == 0 {
            6
        } else {
            9
        };
        jiff::fmt::temporal::DateTimePrinter::new()
            .precision(Some(digits))
            .timestamp_with_offset_to_string(self, jiff::tz::Offset::UTC)
    }
}

#[cfg(test)]
mod tests {
    use super::Rfc3339;

    #[test]
    fn rfc3339_matches_chrono_text() {
        for (input, expected) in [
            ("2026-11-17T12:34:56Z", "2026-11-17T12:34:56+00:00"),
            ("2026-11-17T12:34:56.1Z", "2026-11-17T12:34:56.100+00:00"),
            (
                "2026-11-17T12:34:56.1234Z",
                "2026-11-17T12:34:56.123400+00:00",
            ),
            ("1969-12-31T23:59:59.5Z", "1969-12-31T23:59:59.500+00:00"),
            (
                "1969-12-31T23:59:59.1234567Z",
                "1969-12-31T23:59:59.123456700+00:00",
            ),
        ] {
            assert_eq!(
                input.parse::<jiff::Timestamp>().unwrap().to_rfc3339(),
                expected
            );
        }
    }
}
