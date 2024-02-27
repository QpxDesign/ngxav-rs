use std::collections::HashMap;

#[derive(Clone)]
pub struct AnalyticsResult<'a> {
    pub request_count: i64,
    pub total_bytes_sent: i64,
    pub top_requests: HashMap<&'a str, TopResult<'a>>,
    pub top_hosts: HashMap<&'a str, TopResult<'a>>,
    pub top_ips: HashMap<&'a str, TopResult<'a>>,
}

#[derive(Clone)]
pub struct TopResult<'a> {
    pub text: &'a str,
    pub count: i64,
}
