#[derive(Clone)]
pub struct LineParseResult {
    pub ip_address: String,
    pub time: String,
    pub host: String,
    pub referer: String,
    pub request: String,
    pub status: String,
    pub body_bytes_sent: i64,
    pub request_time: String,
    pub user_agent: String,
    pub full_text: String,
}
