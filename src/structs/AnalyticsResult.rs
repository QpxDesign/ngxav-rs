use std::collections::HashMap;

#[derive(Clone)]
pub struct AnalyticsResult {
    pub request_count: i64,
    pub total_bytes_sent: i64,
    pub top_requests: HashMap<String, TopResult>,
    pub top_hosts: HashMap<String, TopResult>,
    pub top_ips: HashMap<String, TopResult>,
}

#[derive(Clone)]
pub struct TopResult {
    pub text: String,
    pub count: i64,
}
