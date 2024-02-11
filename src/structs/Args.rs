use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct ArgParser {
    /// Name of the person to greet
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

    #[arg(short = 'o', long = "referer")]
    pub referer: Option<String>,

    #[arg(short = 'm', long = "large")]
    pub large: Option<u8>,

    #[arg(short = 'l', long = "lst")]
    pub last: Option<u8>,

    #[arg(short, long, action=ArgAction::SetTrue)]
    pub unique: Option<bool>,

    #[arg(short, long, action=ArgAction::SetTrue)]
    pub analytics: Option<bool>,

    #[arg(short = 'x', long = "sa", action=ArgAction::SetTrue)]
    pub session_analytics: Option<bool>,

    #[arg(short = 'i', long = "ip_ses")]
    pub ip_session: Option<String>,

    #[arg(short = 'c', long = "conservemem")]
    pub conserve_memory: Option<bool>,
}
