use chrono::prelude::*;

pub fn parse_nginx_time_format(time: &str) -> DateTime<Utc> {
    DateTime::parse_from_str(time, "%d/%b/%Y:%H:%M:%S %z")
        .expect("Failed to parse date and time")
        .into()
}
