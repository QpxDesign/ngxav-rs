use chrono::prelude::*;
pub fn parse_input_time(input: &str, timezone: String) -> DateTime<Utc> {
    DateTime::parse_from_str(
        format!("{i} {tz}", i = input, tz = timezone,).as_str(),
        "%d/%b/%Y:%H:%M:%S %z",
    )
    .expect("Failed to parse date and time")
    .into()
}
