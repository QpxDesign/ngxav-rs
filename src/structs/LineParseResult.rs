#[derive(Clone)]
pub struct LineParseResult<'a> {
    pub ip_address: &'a str,
    pub time: String,
    pub host: &'a str,
    pub referer: &'a str,
    pub request: &'a str,
    pub status: &'a str,
    pub body_bytes_sent: i64,
    pub request_time: i64,
    pub user_agent: &'a str,
    pub full_text: &'a str,
}
