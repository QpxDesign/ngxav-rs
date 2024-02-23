use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct ArgParser {
    #[arg(short = 'f', long = "file")]
    pub file: String,

    #[arg(short = 's', long = "search")]
    pub search: Option<String>,

    #[arg(short = 'b', long = "start_date")]
    pub start_date: Option<String>,

    #[arg(short = 'e', long = "end_date")]
    pub end_date: Option<String>,

    #[arg(short = 'q', long = "host")]
    pub host: Option<String>,

    #[arg(short = 'r', long = "request")]
    pub request: Option<String>,

    #[arg(short = 't', long = "http_status")]
    pub http_status: Option<String>,

    // HTTP referer (like google.com or instagram)
    #[arg(short = 'o', long = "referer")]
    pub referer: Option<String>,

    #[arg(short = 'm', long = "large")]
    pub large: Option<usize>,

    #[arg(short = 'l', long = "lst")]
    pub last: Option<u64>,

    #[arg(short, long, action=ArgAction::SetTrue)]
    pub unique: Option<bool>,

    #[arg(short, long, action=ArgAction::SetTrue)]
    pub analytics: Option<bool>,

    #[arg(short = 'x', long = "sa", action=ArgAction::SetTrue)]
    pub session_analytics: Option<bool>,

    // plaintext search (faster)
    #[arg(short = 'p', long = "pt", action=ArgAction::SetTrue)]
    pub plain_text: Option<bool>,

    #[arg(short = 'd', long = "ip_ses")]
    pub ip_ses: Option<String>,

    #[arg(short = 'z', long = "browser")]
    pub browser: Option<String>,

    #[arg(short = 'v', long = "os")]
    pub os: Option<String>,

    #[arg(short = 'g', long = "bot")]
    pub bot: Option<bool>,

    #[arg(short = 'j', long = "device_category")]
    pub device_category: Option<String>,

    #[arg(short = 'w', long = "su", action=ArgAction::SetTrue)]
    pub session_unqiue: Option<bool>,

    #[arg(short = 'i', long = "threads")]
    pub thread_count: Option<i64>,
}
